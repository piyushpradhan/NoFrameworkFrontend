@echo off
REM Enable echo for each command and exit on error
setlocal enabledelayedexpansion

REM Run wasm-pack build with target as web
wasm-pack build --target wasm32-unknown-unknown

REM Disable echoing and exit
endlocal
