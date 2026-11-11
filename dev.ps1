[CmdletBinding(DefaultParameterSetName = 'Run')]
param(
    [Parameter(Mandatory, ParameterSetName = 'Run')][string[]]$Package,
    [Parameter(ParameterSetName = 'Run')][string[]]$TestPackage = @(),
    [Parameter(Mandatory, ParameterSetName = 'List')][switch]$List,
    [switch]$CheckOnly,
    [switch]$Publish,
    [switch]$Offline,
    [string]$CompilerBundle = ($env:RIG_SPIRAL_BUNDLE ?? $env:KIT_SPIRAL_BUNDLE),
    [ValidateRange(1, 600)][int]$TimeoutSec = 180,
    [ValidateRange(1, 8)][int]$Parallel = 3,
    [switch]$Full
)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'compiler-contracts/workspace.ps1')
if ($List) {
    Get-KitOwners -Root $PSScriptRoot -IncludeAuxiliary | Select-Object Package, Member, Input, Target
    return
}
$Package = @($Package | ForEach-Object { $_ -split ',' } | ForEach-Object Trim | Where-Object { $_ })
$Package = @(Get-KitOwners -Root $PSScriptRoot -Package $Package -IncludeAuxiliary | ForEach-Object Package | Select-Object -Unique)
$TestPackage = @($TestPackage | ForEach-Object { $_ -split ',' } | ForEach-Object Trim | Where-Object { $_ })
if ($TestPackage.Count) { $TestPackage = @(Get-KitOwners -Root $PSScriptRoot -Package $TestPackage -IncludeAuxiliary | ForEach-Object Package | Select-Object -Unique) }
& (Join-Path $PSScriptRoot 'compiler-contracts/test-regeneration.ps1') `
    -KitRoot $PSScriptRoot -Package $Package -TestPackage $TestPackage -CompilerBundle $CompilerBundle -IncludeAuxiliary `
    -TargetDirectory (Join-Path $PSScriptRoot 'src/target/dev-validation') `
    -TimeoutSec $TimeoutSec -CargoCheck -Test:(-not $CheckOnly) -SkipRelease `
    -Publish:$Publish -Offline:$Offline -Parallel $Parallel -Incremental:(-not $Full)
