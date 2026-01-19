# fzopnr 🚀

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/rust-v1.75%2B-blue.svg)
![Platform](https://img.shields.io/badge/platform-macos%20%7C%20linux-lightgrey)
![Build Status](https://github.com/k-kawa/fzopnr/actions/workflows/ci.yml/badge.svg)

**fzopnr** is a blazing fast CLI tool that reads configuration files from your current working directory (recursively) and home directory to open URLs. It features an integrated fuzzy finder for seamless navigation.

---

## ✨ Features

- **Recursive Config Discovery**: Automatically finds `.fzopnr.toml` in your project hierarchy.
- **Fuzzy Search**: Built-in fuzzy finder (`skim`) to quickly select URLs.
- **Priority Merging**: Smartly overrides global configs with project-specific URLs.
- **Cross-Platform**: Works on macOS and Linux.

## 🎥 Demo

![Demo](demo.gif)

## 📦 Installation
### Homebrew
```bash
brew install k-kawa/fzopnr/fzopnr
```

### Binary Download
Download the latest binary for your platform from the [Releases page](https://github.com/k-kawa/fzopnr/releases).

### From Source
```bash
cargo install --path .
```

## 🚀 Usage

### 1. Interactive Mode
Run without arguments to launch the fuzzy finder:

```bash
fzopnr
```

### 2. Direct Mode
Open a specific key directly:

```bash
fzopnr google
```

## ⚙️ Configuration

Create a `.fzopnr.toml` file in your home directory or project directory:

```toml
[urls]
google = "https://google.com"
github = "https://github.com"
issue-tracker = "https://jira.company.com"
```

### Configuration Precedence

Configuration files are loaded and merged in the following order (later overrides earlier):

1.  **Home Directory**: `~/.fzopnr.toml` (Base configuration)
2.  **Parent Directories**: Scanning from the root down to the current directory.
3.  **Current Directory**: `./.fzopnr.toml` (Highest priority)

If the same key is defined in multiple files, the **most specific** (closest to the current working directory) value will be used.
