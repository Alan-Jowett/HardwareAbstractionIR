param(
    [switch]$Release
)

$ErrorActionPreference = "Stop"

$manifestPath = Join-Path $PSScriptRoot "Cargo.toml"
$profile = if ($Release) { "release" } else { "debug" }
$buildArgs = @("build", "--manifest-path", $manifestPath, "--target", "thumbv7m-none-eabi")
if ($Release) {
    $buildArgs += "--release"
}

& cargo @buildArgs
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

$elfPath = Join-Path $PSScriptRoot "target\thumbv7m-none-eabi\$profile\lm3s-embassy-smoke"
if (-not (Test-Path $elfPath)) {
    throw "Expected ELF not found at $elfPath"
}

$elfPathWsl = (& wsl wslpath -a ($elfPath -replace "\\", "/")).Trim()
$qemuCommand = "timeout 20s qemu-system-arm -M lm3s6965evb -display none -monitor none -serial stdio -semihosting-config enable=on,target=native -kernel '$elfPathWsl'"

$output = & wsl bash -lc $qemuCommand 2>&1
$qemuExit = $LASTEXITCODE
$outputText = ($output | Out-String).Trim()

if ($outputText) {
    Write-Host $outputText
}

if ($qemuExit -eq 124) {
    throw "QEMU smoke run timed out."
}

if ($outputText -match "FAIL:") {
    throw "Smoke firmware reported a failure."
}

if ($outputText -notmatch "UART0 smoke ok") {
    throw "UART0 stdio verification text was not observed in QEMU output."
}

if ($qemuExit -ne 0) {
    throw "QEMU exited with status $qemuExit."
}
