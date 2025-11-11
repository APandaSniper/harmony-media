@echo off
REM test-platforms.bat

echo Testing Web (WASM)...
cd crates\harmony-client
cargo check --target wasm32-unknown-unknown
if %errorlevel% equ 0 (
    echo ✅ Web compilation successful
) else (
    echo ❌ Web compilation failed
    exit /b 1
)

echo Testing Windows Desktop...
cargo check --target x86_64-pc-windows-msvc
if %errorlevel% equ 0 (
    echo ✅ Windows Desktop compilation successful
) else (
    echo ❌ Windows Desktop compilation failed
    exit /b 1
)

echo.
echo Note: iOS compilation requires macOS with Xcode
echo Note: Android compilation requires Android NDK