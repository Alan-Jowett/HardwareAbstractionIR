param(
    [switch]$Release,
    [string]$ContainerImage = "ghcr.io/alan-jowett/sonde-esp-dev:v5.4.1-20260707-cacc8ed"
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

& docker version --format "{{.Server.Version}}" 2>$null | Out-Null
if ($LASTEXITCODE -ne 0) {
    throw "Docker is required to run the ESP32-C3 smoke test."
}

$mountRoot = $PSScriptRoot
$elfPathInContainer = "/work/target/riscv32imc-unknown-none-elf/$profile/embassy-smoke"
$qemuCommand = "timeout 15s qemu-system-riscv32 -nographic -machine esp32c3 -serial mon:stdio -device loader,file=$elfPathInContainer,cpu-num=0"
$stdoutPath = Join-Path $env:TEMP ("esp32c3-smoke-" + [guid]::NewGuid().ToString() + ".out")
$stderrPath = Join-Path $env:TEMP ("esp32c3-smoke-" + [guid]::NewGuid().ToString() + ".err")
try {
    $dockerArgs = "run --rm -v `"${mountRoot}:/work`" $ContainerImage sh -lc `"$qemuCommand`""
    $process = Start-Process -FilePath "docker" -ArgumentList $dockerArgs -NoNewWindow -Wait -PassThru -RedirectStandardOutput $stdoutPath -RedirectStandardError $stderrPath
    $qemuExit = $process.ExitCode
    $stdoutText = if (Test-Path $stdoutPath) { Get-Content $stdoutPath -Raw } else { "" }
    $stderrText = if (Test-Path $stderrPath) { Get-Content $stderrPath -Raw } else { "" }
    $outputText = ($stdoutText + $stderrText).Trim()
}
finally {
    Remove-Item $stdoutPath, $stderrPath -ErrorAction SilentlyContinue
}

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
