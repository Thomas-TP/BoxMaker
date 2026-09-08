. (Join-Path $PSScriptRoot 'rust-env.ps1')
& cargo @args
exit $LASTEXITCODE
