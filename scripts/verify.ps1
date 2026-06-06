Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Invoke-Checked {
    param([Parameter(Mandatory = $true)][string[]]$Command)

    $exe = $Command[0]
    $arguments = @($Command | Select-Object -Skip 1)
    & $exe @arguments

    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}

Invoke-Checked @("cargo", "fmt", "--check")
Invoke-Checked @("cargo", "clippy", "--all-targets", "--all-features", "--", "-D", "warnings")
Invoke-Checked @("cargo", "test")
Invoke-Checked @("cargo", "build", "--release")
