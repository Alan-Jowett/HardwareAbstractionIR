param(
    [switch]$Release
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $false

$manifestPath = Join-Path $PSScriptRoot "Cargo.toml"
$profile = if ($Release) { "release" } else { "debug" }
$buildArgs = @("build", "--manifest-path", $manifestPath, "--target", "riscv32imc-unknown-none-elf")
if ($Release) {
    $buildArgs += "--release"
}

Push-Location $PSScriptRoot
try {
    & cargo @buildArgs
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}
finally {
    Pop-Location
}

$elfPath = Join-Path $PSScriptRoot "target\riscv32imc-unknown-none-elf\$profile\embassy-pwm-smoke"
if (-not (Test-Path $elfPath)) {
    throw "Expected ELF not found at $elfPath"
}

$binPath = "$elfPath.bin"
& llvm-objcopy -O binary $elfPath $binPath
if ($LASTEXITCODE -ne 0) {
    throw "llvm-objcopy failed while producing $binPath"
}

Write-Host "Wrote $binPath"
