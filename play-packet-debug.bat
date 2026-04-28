@echo off
REM Abre dois terminais:
REM 1) logs normais do cliente, via play.bat
REM 2) pacotes baixo nivel, via packet-trace.log

setlocal
set "ROOT=%~dp0"
set "TRACE=%ROOT%korangar\korangar\packet-trace.log"
set "TAIL=%ROOT%packet-debug-terminal.ps1"

tasklist /FI "IMAGENAME eq korangar.exe" 2>NUL | find /I "korangar.exe" >NUL
if not errorlevel 1 (
    echo [erro] korangar.exe ja esta aberto.
    echo Feche o cliente antes de iniciar os dois terminais de debug.
    pause
    exit /b 1
)

pushd "%ROOT%korangar"
cargo build --release -p korangar --features debug
if errorlevel 1 (
    popd
    echo [erro] falha ao compilar Korangar com feature debug.
    echo Feche qualquer korangar.exe aberto e tente novamente.
    pause
    exit /b 1
)
popd

start "Korangar packet debug" powershell -NoExit -ExecutionPolicy Bypass -File "%TAIL%" -TracePath "%TRACE%"
start "Korangar client logs" cmd /k ""%ROOT%play.bat" %*"

endlocal
