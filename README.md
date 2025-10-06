# Maestro 🎵

[![Build and Release](https://github.com/z3k-io/maestro/actions/workflows/release.yaml/badge.svg?branch=main)](https://github.com/z3k-io/maestro/actions/workflows/release.yaml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows-lightgrey.svg)](https://github.com/z3k-io/maestro)

---
### Ditch Window's uninspired audio controls. 

Maestro is a beautiful, modern audio mixer for Windows. Control individual application volumes with mappable key bindings.

## ✨ Features

- 🎨 **Beautiful UI** - Modern interface that replaces Windows' basic volume controls
- 🔧 **Configurable** - Remap keybinds, change themes, and customize behavior
- 🚀 **Lightweight** - Built with Tauri for optimal performance

## 📦 Installation

#### Download the [latest release](https://github.com/z3k-io/maestro/releases)

#### Requirements
- Windows 10/11
- WebView2 Runtime (pre-installed with modern releases)

## 🚀 Quick Start

1. Download and install Maestro
2. Runs at startup by default

## 🛠️ Development

### Architecture
Built with [Tauri](https://tauri.app), Windows API integration, audio session management, and hardware communication managed in [Rust](https://rust-lang.org/). UI uses [WebView2](https://learn.microsoft.com/en-us/microsoft-edge/webview2/), built with Vue.

### Development Setup

#### Requirements
- **Rust** (1.8+)
- **Bun** (or equivalent runtime)

#### Getting Started
```bash
# Clone the repository
git clone https://github.com/z3k-io/maestro.git

# Install dependencies
bun install

# Start development server
bun start

# Build the executable
bun run build
```

## 🤝 Contributing

Contributions are welcome!
- 🐛 **Report Bugs**: Open an issue with detailed reproduction steps
- 💡 **Feature Requests**: Suggest new features or improvements
- 🔧 **Code Contributions**: Submit pull requests for bug fixes or features


#### Future Plans
- Hardware integrations
- Automatic updates  
- Multiple output devices
- Input device support


## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

**Made with ❤️ for Windows enthusiasts**
