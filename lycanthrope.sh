#!/usr/bin/env bash
set -euo pipefail

# ═══════════════════════════════════════════════════
#  🐺 Lycanthrope Installer
#  Linux / macOS / Android (Termux)
# ═══════════════════════════════════════════════════

VERSION="0.1.0"
BINARY_NAME="lycanthrope"
REPO_DIR="$(cd "$(dirname "$0")" && pwd)"

# ── Цвета ──
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
BOLD='\033[1m'
NC='\033[0m'

banner() {
    echo -e "${MAGENTA}"
    echo //                                                                                                                   
    echo '     //                                                                                                                   '
    echo '     // ____ ____     ___  ____         _     ___      _____________ ____    ___________      ____   ________  __________ '
    echo '     // `MM' `MM(     )M' 6MMMMb/      dM.    `MM\     `M'MMMMMMMMMM `MM'    `MM`MMMMMMMb.   6MMMMb  `MMMMMMMb.`MMMMMMMMM '
    echo '     //  MM   `MM.    d' 8P    YM     ,MMb     MMM\     M /   MM   \  MM      MM MM    `Mb  8P    Y8  MM    `Mb MM      \ '
    echo '     //  MM    `MM.  d' 6M      Y     d'YM.    M\MM\    M     MM      MM      MM MM     MM 6M      Mb MM     MM MM        '
    echo '     //  MM     `MM d'  MM           ,P `Mb    M \MM\   M     MM      MM      MM MM     MM MM      MM MM     MM MM    ,   '
    echo '     //  MM      `MM'   MM           d'  YM.   M  \MM\  M     MM      MMMMMMMMMM MM    .M9 MM      MM MM    .M9 MMMMMMM   '
    echo '     //  MM       MM    MM          ,P   `Mb   M   \MM\ M     MM      MM      MM MMMMMMM9' MM      MM MMMMMMM9' MM    `   '
    echo '     //  MM       MM    MM          d'    YM.  M    \MM\M     MM      MM      MM MM  \M\   MM      MM MM        MM        '
    echo '     //  MM       MM    YM      6  ,MMMMMMMMb  M     \MMM     MM      MM      MM MM   \M\  YM      M9 MM        MM        '
    echo '     //  MM    /  MM     8b    d9  d'      YM. M      \MM     MM      MM      MM MM    \M\  8b    d8  MM        MM      / '
    echo '     // _MMMMMMM _MM_     YMMMM9 _dM_     _dMM_M_      \M    _MM_    _MM_    _MM_MM_    \M\_ YMMMM9  _MM_      _MMMMMMMMM '
    echo '     //                                                                                                                   '
    echo '     //                                                                                                                   '
    echo '     //                     ${BOLD}Lycanthrope${NC} - TCP Connection Hijacker   '                                       
    echo -e "${NC}"
    echo -e "${BOLD}    Installer v${VERSION}${NC}"
    echo ""
}

info()    { echo -e "${CYAN}[*]${NC} $1"; }
success() { echo -e "${GREEN}[✓]${NC} $1"; }
warn()    { echo -e "${YELLOW}[!]${NC} $1"; }
error()   { echo -e "${RED}[✗]${NC} $1"; }

# ── Определяем платформу ──
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux*)
            if [ -d "/data/data/com.termux" ] || [ -n "${TERMUX_VERSION:-}" ]; then
                PLATFORM="android"
                INSTALL_DIR="$HOME/.local/bin"
                SHELL_RC="$HOME/.bashrc"
            else
                PLATFORM="linux"
                INSTALL_DIR="/usr/local/bin"
                SHELL_RC="$HOME/.bashrc"
                # Проверяем zsh
                if [ -f "$HOME/.zshrc" ]; then
                    SHELL_RC="$HOME/.zshrc"
                fi
            fi
            ;;
        Darwin*)
            PLATFORM="macos"
            INSTALL_DIR="/usr/local/bin"
            SHELL_RC="$HOME/.zshrc"
            if [ ! -f "$SHELL_RC" ]; then
                SHELL_RC="$HOME/.bash_profile"
            fi
            ;;
        *)
            error "Unsupported OS: $OS"
            error "Use lycanthrope.bat for Windows"
            exit 1
            ;;
    esac

    info "Platform: ${BOLD}$PLATFORM${NC} ($ARCH)"
}

# ── Проверяем зависимости ──
check_dependencies() {
    info "Checking dependencies..."

    # Rust
    if ! command -v cargo &> /dev/null; then
        warn "Rust not found. Installing via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        success "Rust installed"
    else
        local rust_ver
        rust_ver=$(rustc --version | cut -d' ' -f2)
        success "Rust found: $rust_ver"
    fi

    # C compiler
    if ! command -v cc &> /dev/null && ! command -v gcc &> /dev/null; then
        warn "C compiler not found. Installing..."
        case "$PLATFORM" in
            linux)
                sudo apt-get update && sudo apt-get install -y build-essential
                ;;
            macos)
                xcode-select --install 2>/dev/null || true
                ;;
            android)
                pkg install clang
                ;;
        esac
        success "C compiler installed"
    else
        success "C compiler found"
    fi

    # libpcap
    case "$PLATFORM" in
        linux)
            if ! dpkg -l libpcap-dev &> /dev/null 2>&1; then
                info "Installing libpcap-dev..."
                sudo apt-get update && sudo apt-get install -y libpcap-dev
                success "libpcap installed"
            else
                success "libpcap found"
            fi
            ;;
        macos)
            # libpcap предустановлен на macOS
            success "libpcap found (pre-installed)"
            ;;
        android)
            if ! pkg list-installed 2>/dev/null | grep -q libpcap; then
                info "Installing libpcap..."
                pkg install libpcap
                success "libpcap installed"
            else
                success "libpcap found"
            fi
            ;;
    esac
}

# ── Сборка ──
build_project() {
    info "Building Lycanthrope (release mode)..."
    echo ""

    cd "$REPO_DIR"

    if cargo build --release 2>&1; then
        echo ""
        success "Build successful"
    else
        echo ""
        error "Build failed!"
        exit 1
    fi
}

# ── Установка бинарника ──
install_binary() {
    local src="$REPO_DIR/target/release/$BINARY_NAME"

    if [ ! -f "$src" ]; then
        error "Binary not found: $src"
        exit 1
    fi

    info "Installing to ${BOLD}$INSTALL_DIR/$BINARY_NAME${NC}"

    # Создаём директорию если нужно
    mkdir -p "$INSTALL_DIR"

    # Копируем
    if [ "$PLATFORM" = "linux" ] && [ "$INSTALL_DIR" = "/usr/local/bin" ]; then
        sudo cp "$src" "$INSTALL_DIR/$BINARY_NAME"
        sudo chmod 755 "$INSTALL_DIR/$BINARY_NAME"
    else
        cp "$src" "$INSTALL_DIR/$BINARY_NAME"
        chmod 755 "$INSTALL_DIR/$BINARY_NAME"
    fi

    success "Binary installed"
}

# ── Добавляем в PATH ──
setup_path() {
    # Проверяем, уже ли в PATH
    if echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
        success "$INSTALL_DIR already in PATH"
        return
    fi

    info "Adding ${BOLD}$INSTALL_DIR${NC} to PATH..."

    local export_line="export PATH=\"$INSTALL_DIR:\$PATH\""

    # Проверяем, не добавлено ли уже в rc файл
    if [ -f "$SHELL_RC" ] && grep -qF "$INSTALL_DIR" "$SHELL_RC"; then
        success "PATH entry already in $SHELL_RC"
    else
        echo "" >> "$SHELL_RC"
        echo "# 🐺 Lycanthrope" >> "$SHELL_RC"
        echo "$export_line" >> "$SHELL_RC"
        success "Added to $SHELL_RC"
    fi

    # Применяем сейчас
    export PATH="$INSTALL_DIR:$PATH"
}

# ── Проверяем установку ──
verify_install() {
    echo ""
    info "Verifying installation..."

    if command -v "$BINARY_NAME" &> /dev/null; then
        local location
        location=$(which "$BINARY_NAME")
        success "Found: $location"

        echo ""
        echo -e "${CYAN}───────────────────────────────────────${NC}"
        "$BINARY_NAME" --version 2>/dev/null || true
        echo -e "${CYAN}───────────────────────────────────────${NC}"
        echo ""

        success "Installation complete!"
        echo ""
        echo -e "${GREEN}Usage:${NC}"
        echo -e "  ${BOLD}lycanthrope --help${NC}"
        echo -e "  ${BOLD}sudo lycanthrope eth0 192.168.1.10:0 192.168.1.20:23${NC}"
        echo -e "  ${BOLD}sudo lycanthrope --tui eth0 10.0.0.5:4444 10.0.0.1:23${NC}"
        echo ""

        if [ "$SHELL_RC" != "" ]; then
            warn "Run ${BOLD}source $SHELL_RC${NC} or restart terminal to update PATH"
        fi
    else
        error "Installation verification failed!"
        error "Binary not found in PATH"
        error "Try: source $SHELL_RC"
        exit 1
    fi
}

# ── Удаление ──
uninstall() {
    banner
    info "Uninstalling Lycanthrope..."

    local bin_path="$INSTALL_DIR/$BINARY_NAME"

    if [ -f "$bin_path" ]; then
        if [ "$PLATFORM" = "linux" ] && [ "$INSTALL_DIR" = "/usr/local/bin" ]; then
            sudo rm -f "$bin_path"
        else
            rm -f "$bin_path"
        fi
        success "Removed $bin_path"
    else
        warn "Binary not found at $bin_path"
    fi

    # Убираем из shell rc
    if [ -f "$SHELL_RC" ]; then
        sed -i.bak '/# 🐺 Lycanthrope/d' "$SHELL_RC" 2>/dev/null || true
        sed -i.bak "/$BINARY_NAME/d" "$SHELL_RC" 2>/dev/null || true
        rm -f "${SHELL_RC}.bak"
        success "Cleaned $SHELL_RC"
    fi

    success "Lycanthrope uninstalled"
}

# ── Main ──
main() {
    banner
    detect_platform

    # Обработка аргументов
    case "${1:-install}" in
        install|--install|-i)
            check_dependencies
            build_project
            install_binary
            setup_path
            verify_install
            ;;
        uninstall|--uninstall|-u|remove)
            uninstall
            ;;
        build|--build|-b)
            check_dependencies
            build_project
            success "Build only — not installed"
            ;;
        --help|-h)
            echo "Usage: $0 [command]"
            echo ""
            echo "Commands:"
            echo "  install     Build and install (default)"
            echo "  uninstall   Remove Lycanthrope"
            echo "  build       Build only, don't install"
            echo "  --help      Show this help"
            ;;
        *)
            error "Unknown command: $1"
            echo "Run: $0 --help"
            exit 1
            ;;
    esac
}

main "$@"