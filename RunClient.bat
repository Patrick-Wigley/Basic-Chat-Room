@echo off
cd ./Client

if NOT exist 'target/release/Client.exe' (
    @REM Build Release for app
    echo Building App, this may take a moment
    cargo build --release
)

@REM Run application
start ./target/release/Client.exe