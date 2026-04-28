@echo off
REM Launcher para o cliente Korangar.
REM Precisa rodar a partir de korangar\korangar para encontrar data.grf / rdata.grf.

setlocal
set "ROOT=%~dp0"
set "CWD=%ROOT%korangar\korangar"
set "EXE=%ROOT%korangar\target\release\korangar.exe"

if not exist "%EXE%" (
    echo [erro] korangar.exe nao encontrado em "%EXE%"
    echo Compile primeiro com: cargo build --release  ^(em %ROOT%korangar^)
    pause
    exit /b 1
)

if not exist "%CWD%\data.grf" (
    echo [erro] data.grf nao encontrado em "%CWD%"
    echo Coloque data.grf e rdata.grf nessa pasta antes de rodar.
    pause
    exit /b 1
)

cd /d "%CWD%"
"%EXE%" %*

if errorlevel 1 pause
endlocal
