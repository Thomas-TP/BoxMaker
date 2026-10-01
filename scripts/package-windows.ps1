param([switch]$SkipBuild, [string]$OutputDir, [string]$SignParams, [string]$AzureTrustedSignFile, [ValidateSet('x64', 'arm64')][string]$Architecture = 'x64')
$ErrorActionPreference = 'Stop'
if ($SignParams -and $AzureTrustedSignFile) { throw 'Choisir une seule méthode de signature.' }
if ($AzureTrustedSignFile -and -not (Test-Path -LiteralPath $AzureTrustedSignFile -PathType Leaf)) { throw 'Fichier de configuration Azure Artifact Signing introuvable.' }
$azureSigningPath = if ($AzureTrustedSignFile) { (Resolve-Path -LiteralPath $AzureTrustedSignFile).Path } else { $null }
$projectRoot = Split-Path -Parent $PSScriptRoot
Set-Location -LiteralPath $projectRoot
if (-not $SkipBuild) {
    & bun run desktop:build
    if ($LASTEXITCODE -ne 0) { throw 'Échec de compilation de Boxmaker.' }
}
$version = (Get-Content -LiteralPath (Join-Path $projectRoot 'package.json') -Raw | ConvertFrom-Json).version
& bun run release:check
if ($LASTEXITCODE -ne 0) { throw 'Versions ou notes de release incohérentes.' }
$stage = Join-Path $projectRoot "artifacts\stage-$version-$Architecture"
$output = if ($OutputDir) { $OutputDir } else { Join-Path $projectRoot "artifacts\releases" }
New-Item -ItemType Directory -Path $stage,$output -Force | Out-Null
$targetRoot = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $projectRoot 'target' }
Copy-Item -LiteralPath (Join-Path $targetRoot 'release\boxmaker.exe') -Destination (Join-Path $stage 'Boxmaker.exe') -Force
Copy-Item -LiteralPath (Join-Path $projectRoot 'LICENSE') -Destination $stage -Force
$packId = if ($Architecture -eq 'arm64') { 'Swiss3Design.Boxmaker.Arm64' } else { 'Swiss3Design.Boxmaker' }
$packArgs = @('pack', '--packId', $packId, '--packVersion', $version, '--packDir', $stage, '--mainExe', 'Boxmaker.exe', '--packTitle', 'Boxmaker', '--packAuthors', "Thomas Prud'homme", '--outputDir', $output, '--framework', 'webview2', '--icon', (Join-Path $projectRoot 'src-tauri\icons\icon.ico'), '--releaseNotes', (Join-Path $projectRoot "docs\releases\v$version.md"))
if ($Architecture -eq 'arm64') { $packArgs += @('--runtime', 'win-arm64', '--channel', 'win-arm64') }
if ($SignParams) { $packArgs += @('--signParams', $SignParams) }
if ($azureSigningPath) { $packArgs += @('--azureTrustedSignFile', $azureSigningPath) }
& dotnet tool run vpk -- @packArgs
if ($LASTEXITCODE -ne 0) { throw 'Échec du packaging Velopack.' }
if ($SignParams -or $AzureTrustedSignFile) {
    $setup = @(Get-ChildItem -LiteralPath $output -Filter "$packId*-Setup.exe" -File)
    if ($setup.Count -ne 1) { throw 'Installateur Velopack introuvable ou ambigu.' }
    $signature = Get-AuthenticodeSignature -LiteralPath $setup[0].FullName
    if ($signature.Status -ne 'Valid') { throw "Signature de l'installateur non valide : $($signature.Status)." }
    Write-Output "Éditeur Windows vérifié : $($signature.SignerCertificate.Subject)"
}
& (Join-Path $PSScriptRoot 'checksums.ps1') -Directory $output
Write-Output "Installateur Velopack : $output"
