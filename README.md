# 🦀✨ Rust SoBundle 🎁

**Rust SoBundle** is a Rust-powered CLI tool that:

* 🔗 Collects all shared library dependencies for your executable
* 🎨 Optionally includes **Qt** libs
* 🗂️ Wraps everything into an **AppDir**
* 📦 And bundles it into a **self-executing** `.run` installer via [`makeself`](https://makeself.io/)!

Your app, all packed up and ready to go — no fuss, no extra config. 😎

---

## 🚀 What Is It?

**Rust SoBundle** helps Linux devs create portable applications with ease:

✅ Collects `.so` dependencies
✅ Supports Qt-based apps
✅ Builds a structured AppDir
✅ Generates a fully executable `.run` file
✅ No installer frameworks or root permissions needed

---

## 🧰 Usage

```bash
so_bundle -e <executable> [-a <appdir>] [-q <qt-path>] [-b]
```

### 🔧 CLI Flags

| Flag              | Description                                          |
| ----------------- | ---------------------------------------------------- |
| `-e` / `--exec`   | 🧠 **Required**: Path to your main executable        |
| `-a` / `--appdir` | 📁 Optional: Custom AppDir output path               |
| `-q` / `--qt`     | 🎨 Optional: Include Qt from the given path          |
| `-b` / `--bundle` | 📦 Optional: Package into a `.run` file via makeself |
| ` ` / `--exclude` | 🚫 Optional, repeatable: skip libraries whose SONAME starts with the given prefix |

---

## ✨ Example

```bash
so_bundle -e ./my_app -q /usr/lib/qt6 -b --exclude libOpenGL.so --exclude libGLX.so
```

This will:

1. 📦 Collect all shared libs needed by `my_app`
2. 🧩 Add Qt libraries (if provided)
3. 🗂️ Create a portable AppDir
4. 🎁 Generate if needed `my_app.run` – a **self-extracting, self-executing** bundle

✅ Just distribute `my_app.run` — users can execute it **immediately**:

```bash
./my_app.run
```

No `chmod`, no extra steps, just run and go! 🏃‍♀️💨

---

## 🧾 Output

* 🗂️ `my_app.AppDir/` → Contains your app and its dependencies
* 🎁 `my_app.run` → A self-executing, makeself-packed archive

---

## 📃 License

MIT License © You 🧑‍💻
Use it. Share it. Make it yours. 💡

---

## 🔥 TL;DR

> `so_bundle` takes your binary and gives you a ready-to-run `.run`
> 🎨 Qt support? ✅
> 📂 AppDir format? ✅
> 🏁 Self-executing installer? ✅
>
> Ship your app like a pro. 📦

---

🦀 **Rust SoBundle**: Because packaging Linux apps shouldn’t be a pain.
