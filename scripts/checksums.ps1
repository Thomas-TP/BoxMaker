param([Parameter(Mandatory = $true)][string]$Directory)
$ErrorActionPreference = 'Stop'
$checksums = Get-ChildItem -LiteralPath $Directory -File |
    Where-Object { $_.Name -ne 'SHA256SUMS.txt' } |
    Sort-Object Name |
    ForEach-Object {
        $hasher = [Security.Cryptography.SHA256]::Create()
        $stream = [IO.File]::OpenRead($_.FullName)
        try {
            $digest = [BitConverter]::ToString($hasher.ComputeHash($stream)).Replace('-', '').ToLowerInvariant()
            "{0}  {1}" -f $digest, $_.Name
        } finally {
            $stream.Dispose()
            $hasher.Dispose()
        }
    }
[IO.File]::WriteAllLines((Join-Path $Directory 'SHA256SUMS.txt'), [string[]]$checksums, [Text.Encoding]::ASCII)
