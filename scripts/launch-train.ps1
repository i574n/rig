param(
    [Parameter(Mandatory)][string] $Name,
    [switch] $Snapshot,
    [switch] $LiveTree,
    [switch] $SkipRegen,
    [switch] $Resume,
    [string] $RunnerHash = '9A99BF53121417BD',
    [int] $CellsPort = 13977,
    [int] $HoldWaitMinutes = 240,
    [string] $Node = '',
    [string] $NodeMirror = 'kit-train',
    [string] $SshHost = '',
    [switch] $NodeStage,
    [string] $ResumeNodeJob = ''
)
$ErrorActionPreference = 'Stop'
if (-not $SshHost) { $SshHost = $Node }
if (-not $LiveTree) { $Snapshot = [switch]::new($true) }
. (Join-Path $PSScriptRoot 'user-env.ps1')
$kitRoot = Split-Path $PSScriptRoot -Parent
$kit = Join-Path $env:LOCALAPPDATA 'kit\bin\kit.exe'
if ($Node -and -not $NodeStage) {
    Set-Location $kitRoot
    $touched = @(git -C $kitRoot status --porcelain --untracked-files=all -- src | ForEach-Object { $_.Substring(3).Trim() } | Where-Object { $_ -match '\.spi$|(^|/)Cargo\.toml$' -and (Test-Path -LiteralPath (Join-Path $kitRoot $_)) })
    if ($touched) {
        $refusedRows = @(); $chunks = 0
        for ($at = 0; $at -lt $touched.Count; $at += 150) {
            $chunk = $touched[$at..([Math]::Min($at + 149, $touched.Count - 1))]
            $part = (& $kit lint precheck @chunk 2>&1 | Out-String).Trim()
            $chunks++
            if ($part -match '(kit|rig) lint precheck refused') { $refusedRows += @($part -split "`n" | Where-Object { $_ -match '^\S+\.spi:\d+: ' }) }
            elseif ($part -notmatch '(kit|rig) lint precheck ok') { throw "precheck gave no verdict for files $at to $($at + $chunk.Count - 1), refusing the launch:`n$(($part -split "`n" | Select-Object -Last 3) -join "`n")" }
        }
        $pre = if ($refusedRows.Count) { "kit lint precheck refused`n$($refusedRows -join "`n")" } else { "kit lint precheck ok ($chunks chunks)" }
        if ($pre -match '(kit|rig) lint precheck refused') {
            $rows = @($pre -split "`n" | Where-Object { $_ -match '^\S+\.spi:\d+: ' })
            $rawText = { param($row) if ($row -notmatch '^(\S+\.spi):(\d+): ') { return $false }; $line = [int]$Matches[2]; $inside = $false; $n = 0; foreach ($l in (Get-Content -LiteralPath (Join-Path $kitRoot $Matches[1]))) { $n++; $marks = ([regex]::Matches($l, '"""')).Count; if ($n -eq $line) { return ($inside -or $marks -gt 0) }; if ($marks % 2 -eq 1) { $inside = -not $inside } }; $false }
            $inRaw = @($rows | Where-Object { & $rawText $_ })
            $changed = @{}
            $headLines = @{}
            $inHead = {
                param($file, $line)
                $source = if (git -C $kitRoot ls-files -- $file) { $file } else { $file -replace '^src/rig_', 'src/kit_' }
                if ($source -eq $file -and -not (git -C $kitRoot ls-files -- $file)) { return $false }
                if (-not (git -C $kitRoot ls-files -- $source)) { return $false }
                if (-not $headLines.ContainsKey($source)) { $headLines[$source] = [Collections.Generic.HashSet[string]]::new([string[]]@(git -C $kitRoot show "HEAD:$source")) }
                $text = @(Get-Content -LiteralPath (Join-Path $kitRoot $file))[$line - 1]
                $back = [regex]::Replace([regex]::Replace($text, '(?<![A-Za-z0-9_])rig_(?=[a-z0-9])', 'kit_'), '(?<![A-Za-z0-9_-])rig-(?=[a-z0-9])', 'kit-')
                $headLines[$source].Contains($text) -or $headLines[$source].Contains($back)
            }
            $unchanged = {
                param($row)
                if ($row -notmatch '^(\S+\.spi):(\d+): ') { return $false }
                $file = $Matches[1]; $line = [int]$Matches[2]
                if (& $inHead $file $line) { return $true }
                if (-not (git -C $kitRoot ls-files -- $file)) { return $false }
                if (-not $changed.ContainsKey($file)) {
                    $set = [Collections.Generic.HashSet[int]]::new()
                    foreach ($h in (git -C $kitRoot diff -U0 HEAD -- $file | Select-String '^@@ -\S+ \+(\d+)(?:,(\d+))? @@')) {
                        $start = [int]$h.Matches[0].Groups[1].Value
                        $count = if ($h.Matches[0].Groups[2].Success) { [int]$h.Matches[0].Groups[2].Value } else { 1 }
                        for ($n = $start; $n -lt $start + $count; $n++) { [void]$set.Add($n) }
                    }
                    $changed[$file] = $set
                }
                -not $changed[$file].Contains($line)
            }
            $old = @($rows | Where-Object { -not (& $rawText $_) -and (& $unchanged $_) })
            $real = @($rows | Where-Object { -not (& $rawText $_) -and -not (& $unchanged $_) })
            if ($real.Count) { throw "precheck refused the snapshot; fix before a node train:`n$($real -join "`n")" }
            $pre = "kit lint precheck ok after excusing $($inRaw.Count) rows inside triple-quoted raw blocks and $($old.Count) rows on lines unchanged since HEAD"
        }
        $notebookFindings = foreach ($rel in ($touched | Where-Object { $_ -like '*.spi' })) {
            $lines = @(Get-Content -LiteralPath (Join-Path $kitRoot $rel))
            $cliCheck = @($lines | Select-String -SimpleMatch 'rig_cli_check' -List).Count -gt 0
            $packageDir = Split-Path (Join-Path $kitRoot $rel)
            $projPath = Join-Path $packageDir 'package.spiproj'
            if (Test-Path -LiteralPath $projPath) {
                $listed = @([regex]::Matches((Get-Content -LiteralPath $projPath -Raw), '(?m)^\s+\|?([a-z][a-z0-9_]*)-?\s*$') | ForEach-Object { $_.Groups[1].Value })
                $own = Split-Path $packageDir -Leaf
                for ($k = 0; $k -lt $lines.Count; $k++) {
                    if ($lines[$k] -match '^\s*//') { continue }
                    $code = $lines[$k] -replace '"(?:[^"\\]|\\.)*"', '' -replace "\$'[^']*'", ''
                    foreach ($ref in [regex]::Matches($code, '(?<![\w''.])([a-z][a-z0-9_]*)\.([a-z][a-z0-9_]*)\.[a-z_]')) {
                        $used = $ref.Groups[1].Value
                        if ($used -ne $own -and $listed -notcontains $used -and (Test-Path -LiteralPath (Join-Path (Split-Path $packageDir) "$used\package.spiproj"))) { "${rel}:$($k + 1): uses package $used, which this package.spiproj does not list" }
                    }
                }
            }
            for ($k = 1; $k -lt $lines.Count; $k++) {
                if ($lines[$k] -match '^\s+\+\+# ' -and $lines[$k - 1] -match '^\s*:= [\w'']+ = \S' -and $lines[$k - 1] -notmatch '=\s*$') { "${rel}:$($k + 1): a := value continued on the next line with ++# (the parser drops the binding); bind each piece first" }
            }
            $keywordLocal = [regex]'^\s*:= (open|and|inl|let|type|union|nominal|prototype|instance|join|real)\s*(=|:)|\\ \((open|and|inl|let|type|union|nominal|prototype|instance|join|real)\s*:|\(\(?~?(open|and|inl|let|type|union|nominal|prototype|instance|join|real)\s*:'
            for ($k = 0; $k -lt $lines.Count; $k++) {
                if ($lines[$k] -notmatch '^\s*//' -and $keywordLocal.IsMatch(($lines[$k] -replace '"(?:[^"\\]|\\.)*"', ''))) { "${rel}:$($k + 1): a keyword used as a local or parameter name (the parser drops the block with only a warning)" }
            }
            $concats = @($lines | Where-Object { $_ -notmatch '^\s*//' } | Where-Object { ($_ -replace '"(?:[^"\\]|\\.)*"', '') -match '\+\+#' }).Count
            if ($concats -and -not @($lines | Select-String -SimpleMatch "sm'_operators" -List).Count) { "${rel}: uses ++# without opening sm'_operators$(if ($cliCheck) { ' (in a rig_cli_check test)' })" }
            for ($n = 0; $n -lt $lines.Count; $n++) {
                if ($lines[$n] -notmatch '^(:# |test )' -or $lines[$n] -match ' static =') { continue }
                $j = $n - 1; while ($j -ge 0 -and $lines[$j].Trim() -eq '') { $j-- }
                if ($j -lt 0 -or $lines[$j] -notmatch '^///> ') { "${rel}:$($n + 1): a test block without its own ///> target line" }
            }
        }
        if (@($notebookFindings).Count) { throw "notebook precheck refused the snapshot (each costs a whole train):`n$($notebookFindings -join "`n")" }
        $lockLog = Join-Path ([IO.Path]::GetTempPath()) "kit-train-lock-$PID.txt"
        Push-Location (Join-Path $kitRoot 'src')
        try { $null = & cargo metadata --offline --locked --format-version 1 2>$lockLog; $lockExit = $LASTEXITCODE } finally { Pop-Location }
        if ($lockExit -ne 0) { throw "lock precheck refused the snapshot: src/Cargo.lock does not match the manifests (the node regen runs --locked); run cargo update --workspace --offline in src:`n$((Get-Content -LiteralPath $lockLog -ErrorAction SilentlyContinue | Select-Object -First 3) -join "`n")" }
        if ($pre -notmatch '(kit|rig) lint precheck ok') { throw "precheck gave no verdict, refusing the launch: $(($pre -split "`n")[-1])" }
        $verdict = ($pre -split "`n")[-1]
        [Console]::WriteLine("$(Get-Date -Format HH:mm:ss) launch-train $Name $verdict ($($touched.Count) touched files)")
    }
    & $kit job start "kit-$Name-node" 43200000 pwsh -NoProfile -File $PSCommandPath -Name $Name -Node $Node -NodeMirror $NodeMirror -SshHost $SshHost -NodeStage -CellsPort $CellsPort
    exit $LASTEXITCODE
}
if ($NodeStage) {
    $remote = (Resolve-Path -LiteralPath (Join-Path $kitRoot '..\i574n.github\scripts\remote.ps1')).Path
    function Say([string] $text) { [Console]::WriteLine("$(Get-Date -Format HH:mm:ss) node-train $Name $text") }
    function Get-TreeFiles([string] $dir) {
        foreach ($entry in [IO.Directory]::EnumerateFileSystemEntries($dir)) {
            if ([IO.Directory]::Exists($entry)) { if ([IO.Path]::GetFileName($entry) -notin 'target', '.cache', '.git', 'bench', 'node_modules', 'vendor') { Get-TreeFiles $entry } }
            elseif ([IO.Path]::GetExtension($entry) -in '.spi', '.spir', '.rs', '.toml', '.spiproj', '.ps1', '.lock') { $entry }
        }
    }
    function Get-Manifest([string] $root, [string[]] $dirs) {
        @(foreach ($dir in $dirs) {
            $base = Join-Path $root $dir
            if ([IO.Directory]::Exists($base)) { Get-TreeFiles $base | ForEach-Object { "$($_.Substring($root.Length + 1)) $((Get-FileHash -LiteralPath $_).Hash)" } }
        }) | Sort-Object
    }
    function Invoke-NodePwsh([string] $script) {
        $encoded = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($script))
        $lines = @(ssh $SshHost "pwsh -NoProfile -NonInteractive -EncodedCommand $encoded")
        if ($LASTEXITCODE -ne 0) { throw "node command failed (exit $LASTEXITCODE): $($lines | Select-Object -Last 3)" }
        $lines
    }
    $snapshotRoot = Join-Path $kitRoot ".cache\release-tree\$Name"
    $libCopy = Join-Path $kitRoot ".cache\release-tree\$Name-lib\lib\spiral"
    $mirror = "C:\home\remote\$NodeMirror"
    if ($ResumeNodeJob) {
        if (-not (Test-Path -LiteralPath $libCopy)) { throw "no launch snapshot under $snapshotRoot; cannot resume $ResumeNodeJob" }
        $jobId = $ResumeNodeJob
        $before = @(Get-Manifest $snapshotRoot @('src') | Where-Object { $_ -match '^src\\\S+\.rs ' })
        Say "resuming node half $jobId against the launch snapshot ($($before.Count) generated files)"
    } else {
        $dllPath = 'AppData\Local\spiral-bin\bin\single-flight\SpiralCompiler\Release\net11.0\SpiralCompiler.dll'
        $corePath = $dllPath.Replace('SpiralCompiler.dll', 'SpiralCompilerCore.dll')
        $boxParity = "dll=$((Get-FileHash (Join-Path $HOME $dllPath)).Hash) core=$((Get-FileHash (Join-Path $HOME $corePath)).Hash) $(rustc --version)"
        $nodeParity = (Invoke-NodePwsh "`"dll=`$((Get-FileHash (Join-Path `$HOME '$dllPath')).Hash) core=`$((Get-FileHash (Join-Path `$HOME '$corePath')).Hash) `$(rustc --version)`"") -join ''
        if ($boxParity -ne $nodeParity) { throw "toolchain parity: box [$boxParity] node [$nodeParity]; refusing a node train" }
        Say "parity ok: $boxParity"
        $regenDir = 'C:\Users\alias\AppData\Local\spiral-bin\kit-regeneration'
        $space = (Invoke-NodePwsh "`$f = [math]::Round((Get-PSDrive C).Free/1GB,1); `$s = @(Get-ChildItem '$regenDir' -Directory -ErrorAction SilentlyContinue | Where-Object { `$_.LastWriteTime -lt (Get-Date).AddHours(-6) }).Count; `"`$f `$s`"") -join ''
        $freeGb, $staleDirs = $space.Trim() -split ' '
        if ([double]$freeGb -lt 5) { throw "node C: has $freeGb GB free (< 5 GB; a regen needs ~1.5 GB plus caches); $staleDirs staged workspace(s) older than 6 h sit in $regenDir; free space first, refusing a node train" }
        Say "node disk ok: $freeGb GB free, $staleDirs stale staged workspace(s) in kit-regeneration"
        $snapshotRoot = Join-Path $kitRoot ".cache\release-tree\$Name"
        $libCopy = Join-Path $kitRoot ".cache\release-tree\$Name-lib\lib\spiral"
        $liveLib = (Resolve-Path -LiteralPath (Join-Path $kitRoot '..\spiral\lib\spiral')).Path
        $treeDirs = @('src', 'scripts', 'compiler-contracts')
        & robocopy $kitRoot $snapshotRoot /MIR /XD target .cache .git /NFL /NDL /NJH /NJS /NP | Out-Null
        if ($LASTEXITCODE -ge 8) { throw "snapshot copy failed (robocopy exit $LASTEXITCODE)" }
        & robocopy $liveLib $libCopy /MIR /XD bench target /NFL /NDL /NJH /NJS /NP | Out-Null
        if ($LASTEXITCODE -ge 8) { throw "lib copy failed (robocopy exit $LASTEXITCODE)" }
        $snapshotManifest = Get-Manifest $snapshotRoot $treeDirs
        $libManifest = Get-Manifest (Split-Path $libCopy) @('spiral')
        foreach ($repo in 'spiral', 'kit') {
            $sync = & $remote $Node sync $repo -Mirror $NodeMirror 2>&1 | Out-String
            if ($sync -notmatch "REMOTE-SYNC-OK $repo") { throw "sync $repo failed: $(($sync -split "`n" | Select-Object -Last 3) -join ' ')" }
        }
        $mirror = "C:\home\remote\$NodeMirror"
        $walker = 'function W([string] $d) { foreach ($e in [IO.Directory]::EnumerateFileSystemEntries($d)) { if ([IO.Directory]::Exists($e)) { if ([IO.Path]::GetFileName($e) -notin ''target'', ''.cache'', ''.git'', ''bench'', ''node_modules'', ''vendor'') { W $e } } elseif ([IO.Path]::GetExtension($e) -in ''.spi'', ''.spir'', ''.rs'', ''.toml'', ''.spiproj'', ''.ps1'', ''.lock'') { $e } } }'
        function Get-NodeManifest([string] $root, [string[]] $dirs) {
            $list = ($dirs | ForEach-Object { "'$_'" }) -join ', '
            @(Invoke-NodePwsh "$walker; @(foreach (`$dir in @($list)) { `$b = Join-Path '$root' `$dir; if ([IO.Directory]::Exists(`$b)) { W `$b | ForEach-Object { `"`$(`$_.Substring($($root.Length + 1))) `$((Get-FileHash -LiteralPath `$_).Hash)`" } } }) | Sort-Object")
        }
        foreach ($pair in @(@($snapshotRoot, "$mirror\kit", $treeDirs, $snapshotManifest), @((Split-Path $libCopy), "$mirror\spiral\lib", @('spiral'), $libManifest))) {
            $boxRoot, $nodeRoot, $dirs, $want = $pair
            $wantMap = @{}; foreach ($row in $want) { $rel, $h = $row -split ' ', 2; $wantMap[$rel] = $h }
            $haveMap = @{}; foreach ($row in (Get-NodeManifest $nodeRoot $dirs)) { $rel, $h = $row -split ' ', 2; $haveMap[$rel] = $h }
            $push = @($wantMap.Keys | Where-Object { $haveMap[$_] -ne $wantMap[$_] })
            $drop = @($haveMap.Keys | Where-Object { -not $wantMap.ContainsKey($_) })
            if (-not $want.Count) { throw "box manifest of $boxRoot is empty; refusing to reconcile $nodeRoot against nothing" }
            if ($drop.Count -gt [Math]::Max(25, $haveMap.Count / 10)) { throw "reconcile would remove $($drop.Count) of $($haveMap.Count) files under $nodeRoot; refusing (a manifest root is likely wrong)" }
            if ($push.Count) {
                $list = Join-Path ([IO.Path]::GetTempPath()) "node-train-$Name-push.txt"
                $tar = Join-Path ([IO.Path]::GetTempPath()) "node-train-$Name-push.tar"
                [IO.File]::WriteAllLines($list, @($push | ForEach-Object { $_.Replace('\', '/') }))
                & tar.exe -cf $tar -C $boxRoot -T $list
                if ($LASTEXITCODE -ne 0) { throw "tar of $($push.Count) differing files failed" }
                scp -q $tar "${SshHost}:C:/home/remote/stage/node-train-push.tar"
                Invoke-NodePwsh "tar.exe -xf C:\home\remote\stage\node-train-push.tar -C '$nodeRoot'; if (`$LASTEXITCODE) { throw 'untar failed' }" | Out-Null
                Remove-Item $list, $tar -ErrorAction Ignore
            }
            if ($drop.Count) { Invoke-NodePwsh ("foreach (`$f in @(" + (($drop | ForEach-Object { "'$nodeRoot\$_'" }) -join ', ') + ")) { Remove-Item -LiteralPath `$f -Force }") | Out-Null }
            $again = Get-NodeManifest $nodeRoot $dirs
            if (($again -join "`n") -ne ($want -join "`n")) { throw "node mirror $nodeRoot still differs from the snapshot after reconciling; refusing a node train" }
            Say "node mirror $nodeRoot = snapshot ($($want.Count) files; pushed $($push.Count), removed $($drop.Count))"
        }
        Invoke-NodePwsh "Copy-Item 'C:\home\bin\kit.exe' '$mirror\kit\kit.exe' -Force" | Out-Null
        $boxState = Join-Path $kitRoot '.cache\release-train'
        $lastInstalled = Join-Path $boxState 'last-installed.txt'
        if (Test-Path -LiteralPath $lastInstalled) {
            $lastName = (Get-Content -LiteralPath $lastInstalled -TotalCount 1).Trim()
            $nodeState = "$($mirror.Replace('\', '/'))/kit/.cache/release-train"
            Invoke-NodePwsh "New-Item -ItemType Directory -Force '$mirror\kit\.cache\release-train\$lastName' | Out-Null" | Out-Null
            scp -q $lastInstalled "${SshHost}:$nodeState/last-installed.txt"
            scp -q (Join-Path $boxState "$lastName\shipped-manifest.txt") "${SshHost}:$nodeState/$lastName/shipped-manifest.txt"
            Say "node notebook selection uses the box's last installed train $lastName"
        }
        $before = Invoke-NodePwsh "Get-ChildItem '$mirror\kit\src' -Recurse -File -Filter *.rs | Where-Object { `$_.FullName -notmatch '\\target\\' } | ForEach-Object { `"`$(`$_.FullName.Substring('$mirror\kit\'.Length)) `$((Get-FileHash `$_.FullName).Hash)`" }"
        $nodeKit = (Get-Content ~/.config/remote-machines.json -Raw | ConvertFrom-Json -AsHashtable)[$Node].kit
        if (-not $nodeKit) { throw "remote-machines.json has no kit path for $Node" }
        $start = & $kit job start --on $SshHost --bin $nodeKit "kit-$Name" 36000000 pwsh -NoProfile -File "$mirror\kit\scripts\release-train.ps1" -Name $Name -KitRoot "$mirror\kit" -Lib "$mirror\spiral\lib\spiral" -NotebooksOnly 2>&1 | Out-String
        $jobId = [regex]::Match($start, 'id=(\S+)').Groups[1].Value
        if (-not $jobId) { throw "node job did not start: $start" }
        Say "node half started: $jobId"
    }
    $misses = 0
    $giveUp = (Get-Date).AddHours(11)
    while ($true) {
        $wait = try { & $kit job wait $jobId 43200000 2>&1 | Out-String } catch { "$_" }
        if ($wait -match '(kit|rig) job wait ok') { break }
        $misses++
        Say "node wait lost its route (miss $misses, exit $LASTEXITCODE, retrying until $($giveUp.ToString('HH:mm'))): $(($wait.Trim() -split "`n" | Select-Object -Last 1))"
        if ((Get-Date) -ge $giveUp) { throw "node half UNKNOWN: no route for the whole node job timeout ($misses misses); the job may still be on $Node. Resume with -NodeStage -ResumeNodeJob $jobId once it answers" }
        Start-Sleep -Seconds ([Math]::Min(300, 60 * $misses))
    }
    & $kit job tail $jobId 40 2>&1 | Where-Object { $_ -match '^\d\d:\d\d:\d\d ' } | ForEach-Object { Say "node: $_" }
    if ($wait -notmatch 'exit=0\b') { throw "node half failed: $(($wait -split "`n" | Select-Object -Last 1))" }
    $after = Invoke-NodePwsh "Get-ChildItem '$mirror\kit\src' -Recurse -File -Filter *.rs | Where-Object { `$_.FullName -notmatch '\\target\\' } | ForEach-Object { `"`$(`$_.FullName.Substring('$mirror\kit\'.Length)) `$((Get-FileHash `$_.FullName).Hash)`" }"
    $changed = @(Compare-Object $before $after -PassThru | Where-Object SideIndicator -eq '=>')
    $out = Join-Path $kitRoot ".cache\release-train\$Name"
    [void][IO.Directory]::CreateDirectory($out)
    $verdictFile = Join-Path $out 'node-verdicts.txt'
    scp -q "${SshHost}:$($mirror.Replace('\', '/'))/kit/.cache/release-train/$Name/notebooks-verdicts.txt" $verdictFile
    if (-not (Test-Path -LiteralPath $verdictFile)) { throw 'node verdicts were not fetched' }
    $copied = 0; $conflicts = 0
    foreach ($row in $changed) {
        $relative, $hash = $row -split ' ', 2
        $snapshotFile = Join-Path $snapshotRoot $relative
        $launchHash = if (Test-Path -LiteralPath $snapshotFile) { (Get-FileHash -LiteralPath $snapshotFile).Hash } else { $null }
        scp -q "${SshHost}:$($mirror.Replace('\', '/'))/kit/$($relative.Replace('\', '/'))" $snapshotFile
        if ((Get-FileHash -LiteralPath $snapshotFile).Hash -ne $hash) { throw "fetched $relative does not match the node's hash" }
        $liveFile = Join-Path $kitRoot $relative
        $liveHash = if (Test-Path -LiteralPath $liveFile) { (Get-FileHash -LiteralPath $liveFile).Hash } else { $null }
        if ($liveHash -ceq $launchHash) { Copy-Item -LiteralPath $snapshotFile -Destination $liveFile -Force; $copied++ }
        else { Say "regen output not copied back (edited in the live tree since the snapshot): $relative"; $conflicts++ }
    }
    Say "fetched $($changed.Count) regenerated outputs by hash; $copied copied back to the live tree, $conflicts left"
    Set-Location $kitRoot
    & $kit job start --heavy "kit-$Name" 18000000 pwsh -NoProfile -File (Join-Path $PSScriptRoot 'release-train.ps1') -Name $Name -Resume -Lib $libCopy -Verdicts $verdictFile
    exit $LASTEXITCODE
}
$runner = (Resolve-Path -LiteralPath (Join-Path $kitRoot '..\spiral\workspace\target\release\spiral.exe')).Path
$hash = (Get-FileHash -LiteralPath $runner).Hash.Substring(0, 16)
if ($RunnerHash -and $hash -ne $RunnerHash) { throw "Cells runner is $hash; this train pins $RunnerHash (pass -RunnerHash '' to accept any)" }
$battery = Get-CimInstance Win32_Battery | Select-Object -First 1
if ($battery -and $battery.BatteryStatus -ne 2) { throw 'the box is on battery; trains run on AC power' }
$python = [Environment]::GetEnvironmentVariable('SPIRAL_PYTHON', 'User')
if ($python) { $env:Path = "$(Split-Path $python);$env:Path" }
$env:SPIRAL_CELLS_SHELL_LAUNCHER = $kit
$env:SPIRAL_CELLS_COMPILER_PORT = "$CellsPort"
if (Get-NetTCPConnection -LocalPort $CellsPort -State Listen -ErrorAction SilentlyContinue) { throw "port $CellsPort is taken; pass another -CellsPort" }
$holdDir = Join-Path $env:LOCALAPPDATA 'kit\train-holds'
$holdDeadline = (Get-Date).AddMinutes($HoldWaitMinutes)
while ($true) {
    $live = @(Get-ChildItem -LiteralPath $holdDir -Filter *.hold -File -ErrorAction SilentlyContinue | ForEach-Object {
        $fields = @{}
        foreach ($line in Get-Content -LiteralPath $_.FullName) { $at = $line.IndexOf('='); if ($at -gt 0) { $fields[$line.Substring(0, $at).Trim()] = $line.Substring($at + 1).Trim() } }
        $until = [datetime]::MinValue
        $holderPid = 0
        $alive = [int]::TryParse("$($fields['pid'])", [ref] $holderPid) -and [bool](Get-Process -Id $holderPid -ErrorAction SilentlyContinue)
        $current = [datetime]::TryParse("$($fields['until'])", [ref] $until) -and (Get-Date) -lt $until
        if ($alive -or $current) { "$($fields['holder']) until $($fields['until'])" }
    })
    if (-not $live.Count) { break }
    if ((Get-Date) -ge $holdDeadline) { throw "trains are held ($($live -join '; ')); waited $HoldWaitMinutes min" }
    [Console]::WriteLine("$(Get-Date -Format HH:mm) train held: $($live -join '; '); waiting")
    Start-Sleep 60
}
$trainArgs = @('-NoProfile', '-File', (Join-Path $PSScriptRoot 'release-train.ps1'), '-Name', $Name)
if ($Snapshot) { $trainArgs += '-Snapshot' }
if ($SkipRegen) { $trainArgs += '-SkipRegen' }
if ($Resume) { $trainArgs += '-Resume' }
if ($Snapshot -or $Resume) {
    $libCopy = Join-Path $kitRoot ".cache/release-tree/$Name-lib/lib/spiral"
    & robocopy (Join-Path $kitRoot '..\spiral\lib\spiral') $libCopy /MIR /XD bench target /NFL /NDL /NJH /NJS /NP | Out-Null
    if ($LASTEXITCODE -ge 8) { throw "lib copy failed (robocopy exit $LASTEXITCODE)" }
    $trainArgs += @('-Lib', $libCopy)
}
Set-Location $kitRoot
& $kit job start --heavy "kit-$Name" 18000000 pwsh @trainArgs
