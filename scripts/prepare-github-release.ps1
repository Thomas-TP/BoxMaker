param(
    [string]$SourceDir = 'artifacts/releases',
    [string]$OutputDir = 'artifacts/public-release',
    [ValidateSet('x64', 'arm64')][string]$Architecture = 'x64'
)
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
Set-Location -LiteralPath $projectRoot

$version = (Get-Content -LiteralPath (Join-Path $projectRoot 'package.json') -Raw | ConvertFrom-Json).version
$source = [IO.Path]::GetFullPath((Join-Path $projectRoot $SourceDir))
$output = [IO.Path]::GetFullPath((Join-Path $projectRoot $OutputDir))
New-Item -ItemType Directory -Path $output -Force | Out-Null
if (@(Get-ChildItem -LiteralPath $output -File).Count -ne 0) {
    throw "Le dossier de publication doit être vide : $output"
}

$names = if ($Architecture -eq 'arm64') {
    @(
        'assets.win-arm64.json',
        'RELEASES-win-arm64',
        'releases.win-arm64.json',
        "Swiss3Design.Boxmaker.Arm64-$version-full.nupkg",
        'Swiss3Design.Boxmaker.Arm64-win-arm64-Portable.zip',
        'Swiss3Design.Boxmaker.Arm64-win-arm64-Setup.exe'
    )
} else {
    @(
        'assets.win.json',
        'RELEASES',
        'releases.win.json',
        "Swiss3Design.Boxmaker-$version-full.nupkg",
        'Swiss3Design.Boxmaker-win-Portable.zip',
        'Swiss3Design.Boxmaker-win-Setup.exe'
    )
}
foreach ($name in $names) {
    $file = Join-Path $source $name
    if (-not (Test-Path -LiteralPath $file -PathType Leaf)) { throw "Fichier de release absent : $name" }
    Copy-Item -LiteralPath $file -Destination $output
}

& (Join-Path $PSScriptRoot 'checksums.ps1') -Directory $output
if (@(Get-ChildItem -LiteralPath $output -File | Where-Object { $_.Name -match '\.(msix|msixbundle|appx|appxbundle)$' }).Count -ne 0) {
    throw 'Un paquet applicatif non vérifié ne doit pas être publié sur GitHub.'
}
Write-Output "Fichiers GitHub préparés sans MSIX non signé : $output"
