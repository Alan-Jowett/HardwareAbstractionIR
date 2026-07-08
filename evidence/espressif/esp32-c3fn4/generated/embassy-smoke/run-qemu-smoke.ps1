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

$elfPath = Join-Path $PSScriptRoot "target\riscv32imc-unknown-none-elf\$profile\embassy-smoke"
if (-not (Test-Path $elfPath)) {
    throw "Expected ELF not found at $elfPath"
}

$elfPathWsl = (& wsl wslpath -a ($elfPath -replace "\\", "/")).Trim()
$wslUser = (& wsl whoami).Trim()
$qemuPath = "/home/$wslUser/qemu-riscv32/build/qemu-system-riscv32"
& wsl test -x $qemuPath
if ($LASTEXITCODE -ne 0) {
    $qemuPath = "qemu-system-riscv32"
}

$output = & wsl timeout 15s $qemuPath -nographic -machine esp32c3 -serial mon:stdio -kernel $elfPathWsl 2>&1
$qemuExit = $LASTEXITCODE
$outputText = ($output | Out-String).Trim()

if ($outputText) {
    Write-Host $outputText
}

if ($outputText -match "FAIL:") {
    throw "Smoke firmware reported a failure."
}

if ($outputText -notmatch "PASS") {
    throw "Smoke firmware did not report PASS."
}

if ($qemuExit -ne 0 -and $qemuExit -ne 124) {
    throw "QEMU exited with status $qemuExit."
}
