<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="Perplexity Desktop" width="128" height="128">
</p>

<h1 align="center">Perplexity Desktop</h1>

<p align="center">
  <strong>A native desktop client for Perplexity AI</strong><br>
  <em>Un client desktop natif pour Perplexity AI</em>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-v2-blue?logo=tauri" alt="Tauri v2">
  <img src="https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey" alt="Platforms">
  <img src="https://img.shields.io/github/license/kaen/perplexity-desktop" alt="License">
</p>

---

## ✨ Features / Fonctionnalités

| English | Français |
|---------|----------|
| 🪟 Chromeless window with custom titlebar | 🪟 Fenêtre sans bordures avec titlebar personnalisée |
| 🎨 macOS-style window controls | 🎨 Contrôles de fenêtre style macOS |
| 📌 System tray with quick actions | 📌 Icône système avec actions rapides |
| ⌨️ Global shortcut to show/hide | ⌨️ Raccourci global pour afficher/masquer |
| 🔒 Session persistence | 🔒 Persistance de session |
| 🚀 Lightweight (~10MB) | 🚀 Léger (~10MB) |

## 📥 Installation

### Windows
Download `.msi` or `-setup.exe` from [Releases](../../releases).

### macOS
Download `.dmg` from [Releases](../../releases).

### Linux

| Distribution | Package |
|--------------|---------|
| Ubuntu/Debian | `.deb` |
| Fedora/RHEL | `.rpm` |
| Universal | `.AppImage` |

```bash
# AppImage (all distros)
chmod +x Perplexity_*.AppImage
./Perplexity_*.AppImage

# Debian/Ubuntu
sudo dpkg -i perplexity_*.deb

# Fedora
sudo dnf install ./perplexity-*.rpm
```

## ⌨️ Keyboard Shortcuts / Raccourcis clavier

| Shortcut | Action |
|----------|--------|
| `Super+Shift+P` | Show/Hide window • Afficher/Masquer |
| `Alt+Shift+P` | Fallback shortcut • Raccourci alternatif |

## 🖱️ Tray Menu / Menu système

| English | Français |
|---------|----------|
| Show/Hide | Afficher/Masquer |
| New conversation | Nouvelle conversation |
| Quit | Quitter |

## 🛠️ Build from source / Compiler depuis les sources

### Prerequisites / Prérequis

- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/)

**Linux only:**
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel libappindicator-gtk3-devel librsvg2-devel
```

### Build

```bash
# Install dependencies
pnpm install

# Development
pnpm tauri dev

# Production build
pnpm tauri build
```

Output in `src-tauri/target/release/bundle/`

## 📄 License

MIT

---

<p align="center">
  <sub>Built with ❤️ using <a href="https://tauri.app">Tauri</a></sub>
</p>
