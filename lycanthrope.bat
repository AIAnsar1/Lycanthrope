@echo off
setlocal EnableDelayedExpansion
chcp 65001 >nul 2>&1
title 🐺 Lycanthrope Installer

:: ═══════════════════════════════════════════════════
::  🐺 Lycanthrope Installer — Windows
:: ═══════════════════════════════════════════════════

set "VERSION=0.1.0"
set "BINARY_NAME=lycanthrope"
set "REPO_DIR=%~dp0"
set "INSTALL_DIR=%USERPROFILE%\.lycanthrope\bin"

:: ── Обработка аргументов ──
if "%1"=="uninstall" goto :uninstall
if "%1"=="--uninstall" goto :uninstall
if "%1"=="-u" goto :uninstall
if "%1"=="--help" goto :show_help
if "%1"=="-h" goto :show_help
if "%1"=="build" goto :build_only

:: ── Главная установка ──
call :banner
call :check_admin
call :check_dependencies
call :build_project
if errorlevel 1 goto :build_failed
call :install_binary
call :setup_path
call :verify_install
goto :end

:: ═══════════════════════════════════════════════════
::  Functions
:: ═══════════════════════════════════════════════════

:banner
echo.
echo [95 //                                                                                                                    [0m
echo [95 //                                                                                                                    [0m
echo [95 // ____ ____     ___  ____         _     ___      _____________ ____    ___________      ____   ________  __________  [0m
echo [95 // `MM' `MM(     )M' 6MMMMb/      dM.    `MM\     `M'MMMMMMMMMM `MM'    `MM`MMMMMMMb.   6MMMMb  `MMMMMMMb.`MMMMMMMMM  [0m
echo [95 //  MM   `MM.    d' 8P    YM     ,MMb     MMM\     M /   MM   \  MM      MM MM    `Mb  8P    Y8  MM    `Mb MM      \  [0m
echo [95 //  MM    `MM.  d' 6M      Y     d'YM.    M\MM\    M     MM      MM      MM MM     MM 6M      Mb MM     MM MM         [0m
echo [95 //  MM     `MM d'  MM           ,P `Mb    M \MM\   M     MM      MM      MM MM     MM MM      MM MM     MM MM    ,    [0m
echo [95 //  MM      `MM'   MM           d'  YM.   M  \MM\  M     MM      MMMMMMMMMM MM    .M9 MM      MM MM    .M9 MMMMMMM    [0m
echo [95 //  MM       MM    MM          ,P   `Mb   M   \MM\ M     MM      MM      MM MMMMMMM9' MM      MM MMMMMMM9' MM    `    [0m
echo [95 //  MM       MM    MM          d'    YM.  M    \MM\M     MM      MM      MM MM  \M\   MM      MM MM        MM         [0m
echo [95 //  MM       MM    YM      6  ,MMMMMMMMb  M     \MMM     MM      MM      MM MM   \M\  YM      M9 MM        MM         [0m
echo [95 //  MM    /  MM     8b    d9  d'      YM. M      \MM     MM      MM      MM MM    \M\  8b    d8  MM        MM      /  [0m
echo [95 // _MMMMMMM _MM_     YMMMM9 _dM_     _dMM_M_      \M    _MM_    _MM_    _MM_MM_    \M\_ YMMMM9  _MM_      _MMMMMMMMM  [0m
echo [95 //                                                                                                                    [0m
echo [95 //                                         TCP Connection Hijacker                                                    [0m
echo [95 //                                                                                                                    [0m
echo.                                                                                                                          [0m 
echo                                            [1mInstaller v%VERSION% — Windows[0m                                       
echo.
exit /b             

:check_admin
echo [36m[*][0m Checking privileges...
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo [33m[!][0m Not running as Administrator
    echo [33m[!][0m PATH modification will be user-level only
) else (
    echo [32m[✓][0m Running as Administrator
)
exit /b

:check_dependencies
echo [36m[*][0m Checking dependencies...

:: Rust
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo [33m[!][0m Rust not found. Please install from https://rustup.rs
    echo [33m[!][0m After installing, restart this script.
    pause
    exit /b 1
) else (
    for /f "tokens=2" %%v in ('rustc --version') do (
        echo [32m[✓][0m Rust found: %%v
    )
)

:: MSVC
where cl >nul 2>&1
if %errorlevel% neq 0 (
    echo [33m[!][0m MSVC compiler not found
    echo [33m[!][0m Install Visual Studio Build Tools:
    echo [33m[!][0m https://visualstudio.microsoft.com/visual-cpp-build-tools/
    echo [33m[!][0m Or run from "Developer Command Prompt for VS"
)

:: Npcap
if not defined NPCAP_SDK (
    :: Попробуем найти автоматически
    if exist "C:\NpcapSDK\Lib\x64\Packet.lib" (
        set "NPCAP_SDK=C:\NpcapSDK"
        echo [32m[✓][0m Npcap SDK found: C:\NpcapSDK
    ) else if exist "%USERPROFILE%\NpcapSDK\Lib\x64\Packet.lib" (
        set "NPCAP_SDK=%USERPROFILE%\NpcapSDK"
        echo [32m[✓][0m Npcap SDK found: %USERPROFILE%\NpcapSDK
    ) else (
        echo [33m[!][0m Npcap SDK not found
        echo [33m[!][0m Download from: https://npcap.com/#download
        echo.
        set /p "NPCAP_SDK=Enter Npcap SDK path (or press Enter to skip): "
        if "!NPCAP_SDK!"=="" (
            echo [31m[✗][0m Npcap SDK required for build
            echo [31m[✗][0m Set NPCAP_SDK environment variable and retry
            pause
            exit /b 1
        )
    )
) else (
    echo [32m[✓][0m Npcap SDK: %NPCAP_SDK%
)

:: Проверяем Npcap runtime
sc query npcap >nul 2>&1
if %errorlevel% neq 0 (
    echo [33m[!][0m Npcap runtime not installed
    echo [33m[!][0m Download from: https://npcap.com/
) else (
    echo [32m[✓][0m Npcap runtime installed
)

exit /b

:build_project
echo.
echo [36m[*][0m Building Lycanthrope (release mode)...
echo.

cd /d "%REPO_DIR%"
cargo build --release
if %errorlevel% neq 0 (
    exit /b 1
)

echo.
echo [32m[✓][0m Build successful
exit /b

:build_failed
echo.
echo [31m[✗][0m Build failed!
echo [31m[✗][0m Check errors above and fix dependencies
pause
goto :end

:install_binary
set "SRC=%REPO_DIR%target\release\%BINARY_NAME%.exe"

if not exist "%SRC%" (
    echo [31m[✗][0m Binary not found: %SRC%
    pause
    goto :end
)

echo [36m[*][0m Installing to [1m%INSTALL_DIR%[0m

:: Создаём директорию
if not exist "%INSTALL_DIR%" (
    mkdir "%INSTALL_DIR%"
)

:: Копируем
copy /Y "%SRC%" "%INSTALL_DIR%\%BINARY_NAME%.exe" >nul
if %errorlevel% equ 0 (
    echo [32m[✓][0m Binary installed
) else (
    echo [31m[✗][0m Failed to copy binary
    pause
    goto :end
)

exit /b

:setup_path
echo [36m[*][0m Configuring PATH...

:: Проверяем, уже ли в User PATH
for /f "tokens=2*" %%a in ('reg query "HKCU\Environment" /v PATH 2^>nul') do (
    set "USER_PATH=%%b"
)

echo !USER_PATH! | findstr /i /c:"%INSTALL_DIR%" >nul 2>&1
if %errorlevel% equ 0 (
    echo [32m[✓][0m Already in PATH
    exit /b
)

:: Добавляем в User PATH
if defined USER_PATH (
    set "NEW_PATH=!USER_PATH!;%INSTALL_DIR%"
) else (
    set "NEW_PATH=%INSTALL_DIR%"
)

:: Записываем в реестр (User level, не требует Admin)
reg add "HKCU\Environment" /v PATH /t REG_EXPAND_SZ /d "!NEW_PATH!" /f >nul 2>&1
if %errorlevel% equ 0 (
    echo [32m[✓][0m Added to User PATH
) else (
    echo [33m[!][0m Failed to update PATH in registry
    echo [33m[!][0m Manually add: %INSTALL_DIR%
)

:: Обновляем PATH для текущей сессии
set "PATH=%INSTALL_DIR%;%PATH%"

:: Оповещаем систему об изменении переменных
:: (чтобы другие окна подхватили)
powershell -Command "[System.Environment]::SetEnvironmentVariable('PATH', [System.Environment]::GetEnvironmentVariable('PATH', 'User') , 'User')" >nul 2>&1

echo [32m[✓][0m PATH updated

exit /b

:verify_install
echo.
echo [36m[*][0m Verifying installation...

where %BINARY_NAME% >nul 2>&1
if %errorlevel% equ 0 (
    for /f "delims=" %%p in ('where %BINARY_NAME%') do (
        echo [32m[✓][0m Found: %%p
    )

    echo.
    echo [36m═══════════════════════════════════════[0m
    %BINARY_NAME% --version 2>nul
    echo [36m═══════════════════════════════════════[0m
    echo.

    echo [32m[✓][0m Installation complete!
    echo.
    echo [32mUsage:[0m
    echo   [1mlycanthrope --help[0m
    echo   [1mlycanthrope --tui eth0 192.168.1.10:0 192.168.1.20:23[0m
    echo.
    echo [33m[!][0m Restart your terminal for PATH changes to take effect
    echo [33m[!][0m Run as Administrator for raw socket operations
) else (
    echo [31m[✗][0m Verification failed!
    echo [31m[✗][0m Binary not found in PATH
    echo.
    echo [33m[!][0m Try restarting your terminal
    echo [33m[!][0m Or manually add to PATH: %INSTALL_DIR%
)

echo.
pause
exit /b

:: ═══════════════════════════════════════════════════
::  Uninstall
:: ═══════════════════════════════════════════════════
:uninstall
call :banner
echo [36m[*][0m Uninstalling Lycanthrope...

:: Удаляем бинарник
if exist "%INSTALL_DIR%\%BINARY_NAME%.exe" (
    del /f "%INSTALL_DIR%\%BINARY_NAME%.exe"
    echo [32m[✓][0m Removed %INSTALL_DIR%\%BINARY_NAME%.exe
) else (
    echo [33m[!][0m Binary not found at %INSTALL_DIR%
)

:: Удаляем из PATH
for /f "tokens=2*" %%a in ('reg query "HKCU\Environment" /v PATH 2^>nul') do (
    set "CURRENT_PATH=%%b"
)

if defined CURRENT_PATH (
    set "CLEAN_PATH=!CURRENT_PATH:%INSTALL_DIR%;=!"
    set "CLEAN_PATH=!CLEAN_PATH:;%INSTALL_DIR%=!"
    set "CLEAN_PATH=!CLEAN_PATH:%INSTALL_DIR%=!"

    reg add "HKCU\Environment" /v PATH /t REG_EXPAND_SZ /d "!CLEAN_PATH!" /f >nul 2>&1
    echo [32m[✓][0m Removed from PATH
)

:: Удаляем пустую директорию
if exist "%INSTALL_DIR%" (
    rmdir "%INSTALL_DIR%" 2>nul
    if not exist "%INSTALL_DIR%" (
        echo [32m[✓][0m Removed directory %INSTALL_DIR%
    )
)

echo.
echo [32m[✓][0m Lycanthrope uninstalled
echo.
pause
goto :end

:: ═══════════════════════════════════════════════════
::  Build Only
:: ═══════════════════════════════════════════════════
:build_only
call :banner
call :check_dependencies
call :build_project
if errorlevel 1 goto :build_failed
echo.
echo [32m[✓][0m Build complete (not installed)
echo [36m[*][0m Binary: %REPO_DIR%target\release\%BINARY_NAME%.exe
pause
goto :end

:: ═══════════════════════════════════════════════════
::  Help
:: ═══════════════════════════════════════════════════
:show_help
call :banner
echo Usage: %~nx0 [command]
echo.
echo Commands:
echo   (default)     Build and install
echo   uninstall     Remove Lycanthrope
echo   build         Build only, don't install
echo   --help        Show this help
echo.
echo Locations:
echo   Binary:  %INSTALL_DIR%\%BINARY_NAME%.exe
echo   PATH:    Added to User environment
echo.
goto :end

:end
endlocal