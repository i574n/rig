# nodebench rate.ps1 - fleet-relative 0-1000 rating + rank from raw nodebench lines.
# CANONICAL copy lives here (kit/infra/nodebench). the fleet coordinator carries a synced copy so its bundle is
# self-contained for the nodes that hold it; this repo is the source of truth.
#
# Each dimension is scored LOG-RELATIVE to the fleet: fleet-min -> 0, fleet-max -> 1000 on an ln axis,
# so a fixed performance RATIO is a fixed number of points (a 2x faster node is always the same gap up),
# which is the right lens when the fleet spans ~10x (1-core .. 28-core nodes). Overall is a
# weighted mean over the dimensions a node actually has (weights renormalize), so a pending-disk or
# software-GPU node is ranked on what it measured, never zeroed for a missing probe.
#
# Input  : scores.tsv  (TAB: nickname <tab> encoding <tab> raw-nodebench-line).  '#'/blank lines ignored.
# Usage  : ./rate.ps1 [-ScoresPath <scores.tsv>]           # prints the ranked table
#          . ./rate.ps1 ; Get-FleetRatings <scores.tsv>    # dot-source for the objects (connect.ps1 does this)

param([string]$ScoresPath = (Join-Path $PSScriptRoot 'scores.tsv'))

function Get-FleetRatings {
    param([Parameter(Mandatory)][string]$ScoresPath)

    $weights = @{ parallel = 0.28; single = 0.24; mem = 0.18; disk = 0.22; gpu = 0.08 }

    # --- parse rows into per-node derived metrics (higher = better for every one) ---
    $nodes = @()
    foreach ($line in Get-Content -LiteralPath $ScoresPath) {
        if ($line -match '^\s*#' -or $line.Trim() -eq '') { continue }
        $parts = $line -split "`t", 3
        if ($parts.Count -lt 3) { continue }
        $raw = $parts[2]
        $kv = @{}
        foreach ($m in [regex]::Matches($raw, '(\w+)=(-?\d+(?:\.\d+)?)')) { $kv[$m.Groups[1].Value] = [double]$m.Groups[2].Value }
        $hwGpu = $raw -match 'GPU_CLASS=(discrete|integrated)'

        $n = [ordered]@{ nickname = $parts[0]; encoding = $parts[1]; raw = $raw; m = @{} }
        if ($kv.ContainsKey('ST_MS')   -and $kv['ST_MS']   -gt 0) { $n.m['single']   = 1000.0 / $kv['ST_MS'] }
        if ($kv.ContainsKey('MT_MS')   -and $kv['MT_MS']   -gt 0 -and $kv.ContainsKey('CORES')) { $n.m['parallel'] = $kv['CORES'] * 1000.0 / $kv['MT_MS'] }
        if ($kv.ContainsKey('MEM_GBPS')-and $kv['MEM_GBPS']-gt 0) { $n.m['mem']      = $kv['MEM_GBPS'] }
        if ($kv.ContainsKey('DWR_MBPS')-and $kv['DWR_MBPS']-gt 0) { $n.m['dwr']      = $kv['DWR_MBPS'] }
        # DIOPS cap + DSYNC floor: a writeback-cached VM returns ~0 fsync and 100k+ "iops" - that is cache, not
        # durable storage, so clamp it to "very good" instead of letting one node dominate the disk dimension ~1000x.
        if ($kv.ContainsKey('DIOPS')   -and $kv['DIOPS']   -gt 0) { $n.m['diops']    = [math]::Min($kv['DIOPS'], 50000.0) }
        if ($kv.ContainsKey('DSYNC_MS')) { $ds = [math]::Max([double]$kv['DSYNC_MS'], 0.05); $n.m['dsync'] = 1000.0 / $ds }
        if ($hwGpu -and $kv.ContainsKey('GPU_MS') -and $kv['GPU_MS'] -gt 0) { $n.m['gpu'] = 1000.0 / $kv['GPU_MS'] }
        $nodes += [pscustomobject]$n
    }
    if ($nodes.Count -eq 0) { return @() }

    # --- log-relative normalizer per dimension (fleet min->0, max->1000) ---
    $dims = 'single','parallel','mem','dwr','diops','dsync','gpu'
    $lnmin = @{}; $lnmax = @{}
    foreach ($d in $dims) {
        $vals = $nodes | Where-Object { $_.m.ContainsKey($d) } | ForEach-Object { [math]::Log($_.m[$d]) }
        if ($vals) { $lnmin[$d] = ($vals | Measure-Object -Minimum).Minimum; $lnmax[$d] = ($vals | Measure-Object -Maximum).Maximum }
    }
    function L([hashtable]$m, [string]$d) {
        if (-not $m.ContainsKey($d) -or -not $lnmin.ContainsKey($d)) { return $null }
        if ($lnmax[$d] -eq $lnmin[$d]) { return 500.0 }
        $v = 1000.0 * ([math]::Log($m[$d]) - $lnmin[$d]) / ($lnmax[$d] - $lnmin[$d])
        [math]::Max(0.0, [math]::Min(1000.0, $v))
    }

    # --- per-node dimension scores + weighted overall over present dims ---
    foreach ($n in $nodes) {
        $single = L $n.m 'single'; $parallel = L $n.m 'parallel'; $mem = L $n.m 'mem'; $gpu = L $n.m 'gpu'
        # disk = log-normalized sub-metrics blended (write .5 / durable-IOPS .3 / fsync-latency .2)
        $dwr = L $n.m 'dwr'; $diops = L $n.m 'diops'; $dsync = L $n.m 'dsync'
        $disk = $null
        $dw = @(); $dv = @()
        if ($null -ne $dwr)   { $dw += 0.5; $dv += $dwr }
        if ($null -ne $diops) { $dw += 0.3; $dv += $diops }
        if ($null -ne $dsync) { $dw += 0.2; $dv += $dsync }
        if ($dv.Count) { $sw = ($dw | Measure-Object -Sum).Sum; $disk = (0..($dv.Count-1) | ForEach-Object { $dw[$_]*$dv[$_] } | Measure-Object -Sum).Sum / $sw }

        $scores = @{ parallel = $parallel; single = $single; mem = $mem; disk = $disk; gpu = $gpu }
        $num = 0.0; $den = 0.0
        foreach ($k in $weights.Keys) { if ($null -ne $scores[$k]) { $num += $weights[$k]*$scores[$k]; $den += $weights[$k] } }
        $overall = if ($den -gt 0) { [math]::Round($num/$den, 1) } else { 0.0 }

        $n | Add-Member NoteProperty rating   $overall
        $n | Add-Member NoteProperty single   ($(if($null -ne $single){[int]$single}else{$null}))
        $n | Add-Member NoteProperty parallel ($(if($null -ne $parallel){[int]$parallel}else{$null}))
        $n | Add-Member NoteProperty mem      ($(if($null -ne $mem){[int]$mem}else{$null}))
        $n | Add-Member NoteProperty disk     ($(if($null -ne $disk){[int]$disk}else{$null}))
        $n | Add-Member NoteProperty gpu      ($(if($null -ne $gpu){[int]$gpu}else{$null}))
    }

    $ranked = $nodes | Sort-Object -Property rating -Descending
    $i = 0
    foreach ($n in $ranked) { $i++; $n | Add-Member NoteProperty rank $i -Force }
    $ranked
}

if ($MyInvocation.InvocationName -ne '.') {
    if (-not (Test-Path -LiteralPath $ScoresPath)) { Write-Error "scores file not found: $ScoresPath"; exit 1 }
    $r = Get-FleetRatings -ScoresPath $ScoresPath
    $r | Format-Table @{n='#';e='rank';w=3}, @{n='rating';e='rating';w=7}, nickname, encoding,
        @{n='par';e='parallel';w=4}, @{n='sgl';e='single';w=4}, @{n='mem';e='mem';w=4}, @{n='disk';e='disk';w=4}, @{n='gpu';e='gpu';w=4} -AutoSize
}
