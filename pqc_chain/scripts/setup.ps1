# Setup do ambiente Entangle (Windows)
# Execute como Administrator se necessário.

Write-Host "=== Entangle — Setup de Ambiente ===" -ForegroundColor Cyan

# Verificar Rust
if (-not (Get-Command rustup -ErrorAction SilentlyContinue)) {
    Write-Host "Instalando Rust via winget..." -ForegroundColor Yellow
    winget install Rustlang.Rustup --accept-package-agreements --accept-source-agreements
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
}

# Target WASM (obrigatório para runtime Substrate)
Write-Host "Adicionando target wasm32-unknown-unknown..." -ForegroundColor Green
rustup target add wasm32-unknown-unknown

# Componentes úteis
rustup component add rustfmt clippy rust-src

# Verificar protoc
if (-not (Get-Command protoc -ErrorAction SilentlyContinue)) {
    Write-Host "AVISO: protoc não encontrado. Instale com: choco install protoc" -ForegroundColor Yellow
}

Write-Host "`nSetup concluído. Próximos passos:" -ForegroundColor Green
Write-Host "  cargo build --release" -ForegroundColor White
Write-Host "  ./target/release/entangle-node --dev" -ForegroundColor White
