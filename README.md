# 🧰 Ucli — Minimalist Unix Command-Line Tools in Rust

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

**Ucli** is a collection of simple, fast, and safe Unix command-line tools reimagined in Rust. Perfect for learning systems programming, handling files, and working with text streams using modern tools.

> 🚀 Built in Rust — blazing fast, memory-safe, and beginner-friendly!

---

## 📦 Available Commands

| Command | Description                              |
| ------- | ---------------------------------------- |
| `cat`   | Output contents of a file line-by-line   |
| `ls`    | List directory contents                  |
| `grep`  | Search for matching lines in a file      |
| `view`  | View file content interactively (`less`) |
| `sort`  | Sort lines in a file                     |
| `tail`  | Show the last N lines of a file          |
| `wc`    | Count lines, words, and bytes in a file  |

### 🔧 Example Usages

```bash
ucli cat file.txt
ucli ls -a
ucli grep "error" logs.txt -i -n
ucli tail logs.txt -n 10 -f
ucli wc file.txt -l -w -c
```

Each command supports relevant flags similar to classic Unix tools.

---

## 🦀 Build from Source

```bash
git clone https://github.com/ibilalkayy/ucli
cd ucli
cargo build --release
```

This will generate a `ucli` binary in `target/release/ucli`.

---

## 🙌 Contributing

Contributions are welcome! Feel free to open issues, suggest new features, or submit pull requests. Help make Ucli a better learning and productivity tool for everyone.
