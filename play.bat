@echo off
REM Launcher para o cliente Korangar.
REM Precisa rodar a partir de korangar\korangar para encontrar os assets.

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

if not exist "%CWD%\data.grf" if not exist "%CWD%\archive\data" (
    echo [erro] assets nao encontrados em "%CWD%"
    echo Coloque data.grf/rdata.grf nessa pasta ou extraia a GRF em archive\data.
    pause
    exit /b 1
)

cd /d "%CWD%"
"%EXE%" %*

if errorlevel 1 pause
endlocal
