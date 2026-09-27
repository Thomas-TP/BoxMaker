param(
    [switch]$SkipBuild,
    [string]$OutputDir,
    [string]$IdentityFile
)
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
Set-Location -LiteralPath $projectRoot
if (-not $SkipBuild) {
    & bun run desktop:build
    if ($LASTEXITCODE -ne 0) { throw 'Échec de compilation de Boxmaker.' }
}
$version = (Get-Content -LiteralPath (Join-Path $projectRoot 'package.json') -Raw | ConvertFrom-Json).version
& bun run release:check
if ($LASTEXITCODE -ne 0) { throw 'Versions ou notes de release incohérentes.' }
$identityPath = if ($IdentityFile) { $IdentityFile } else { Join-Path $projectRoot 'store\identity.json' }
if (-not (Test-Path -LiteralPath $identityPath -PathType Leaf)) {
    throw 'Identité Store absente : réservez Boxmaker dans Partner Center, puis créez store/identity.json à partir de store/identity.example.json.'
}
$identity = Get-Content -LiteralPath $identityPath -Raw | ConvertFrom-Json
foreach ($field in @('name', 'publisher', 'publisherDisplayName')) {
    if (-not $identity.$field -or $identity.$field -match '^VALEUR ') { throw "Identité Store incomplète : $field." }
}
$versionMatch = [regex]::Match($version, '^(\d+)\.(\d+)\.(\d+)$')
if (-not $versionMatch.Success) { throw 'Le Store exige une version stable X.Y.Z sans suffixe.' }
$msixVersion = "$($versionMatch.Groups[1].Value).$($versionMatch.Groups[2].Value).$($versionMatch.Groups[3].Value).0"
$sdkBin = Join-Path ${env:ProgramFiles(x86)} 'Windows Kits\10\bin'
$makeAppx = Get-ChildItem -LiteralPath $sdkBin -Directory -ErrorAction SilentlyContinue |
    Sort-Object Name -Descending |
    ForEach-Object { Join-Path $_.FullName 'x64\makeappx.exe' } |
    Where-Object { Test-Path -LiteralPath $_ } |
    Select-Object -First 1
if (-not $makeAppx) { throw 'MakeAppx.exe introuvable : installer le SDK Windows.' }
$stage = Join-Path $projectRoot "artifacts\msix-stage-$version-$([guid]::NewGuid().ToString('N'))"
$output = if ($OutputDir) { $OutputDir } else { Join-Path $projectRoot 'artifacts\releases' }
New-Item -ItemType Directory -Path $stage,(Join-Path $stage 'Assets'),$output -Force | Out-Null
$targetRoot = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $projectRoot 'target' }
Copy-Item -LiteralPath (Join-Path $targetRoot 'release\boxmaker.exe') -Destination (Join-Path $stage 'Boxmaker.exe') -Force
foreach ($name in @('StoreLogo', 'Square150x150Logo', 'Square44x44Logo')) {
    Copy-Item -LiteralPath (Join-Path $projectRoot "src-tauri\icons\$name.png") -Destination (Join-Path $stage 'Assets') -Force
}
Set-Content -LiteralPath (Join-Path $stage 'boxmaker-store.txt') -Value 'Microsoft Store channel' -Encoding ascii
[xml]$manifest = Get-Content -LiteralPath (Join-Path $projectRoot 'store\AppxManifest.template.xml') -Raw
$manifest.Package.Identity.Name = [string]$identity.name
$manifest.Package.Identity.Publisher = [string]$identity.publisher
$manifest.Package.Identity.Version = $msixVersion
$manifest.Package.Properties.PublisherDisplayName = [string]$identity.publisherDisplayName
$manifest.Save((Join-Path $stage 'AppxManifest.xml'))
$package = Join-Path $output "Boxmaker-$version-Store-submission-UNSIGNED.msix"
& $makeAppx pack /d $stage /p $package /o
if ($LASTEXITCODE -ne 0) { throw 'Échec de création du MSIX.' }
& (Join-Path $PSScriptRoot 'checksums.ps1') -Directory $output
Write-Output "MSIX pour Partner Center (non signé, non installable directement) : $package"
