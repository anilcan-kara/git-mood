# 🎨 git-mood

> Color your GitHub contribution graph with the mood of your git commits.

`git-mood` is a blazingly fast, zero-dependency local CLI tool written in Rust. It hooks into your git workflow, analyzes the sentiment of your commit messages and diff stats in real time, and logs your daily coding "mood". 

Using a local database, it generates a beautiful, custom-colored contribution graph SVG (shining with **Turquoise**, **Red**, **Yellow**, **Purple**, and **Green**) that you can embed directly in your GitHub Profile README.

---

## 🎭 The Mood Map

| Color | Mood | Trigger Rule |
| :--- | :--- | :--- |
| **🔵 Turquoise** | **Productive & Happy** | Commits with positive keywords (`feat`, `add`, `awesome`, `super`, `love`, `success`) |
| **🔴 Red** | **Bug-Fixing / Stressed** | Commits containing bugs, fails, crashes, errors, or swear words |
| **🟡 Yellow** | **Cleanups & Refactoring** | Commits with refactor keywords OR where deletions represent $\ge 70\%$ of the total diff |
| **🟣 Purple** | **Weekend Warrior** | Commits made on Saturday or Sunday |
| **🟢 Green** | **Steady Progress** | Default / Neutral commits (chores, fixes, docs) |

---

## 🚀 Key Features

- **Blazingly Fast**: Compiled binary in Rust. Starts and finishes in less than 2 milliseconds.
- **Privacy First**: Fully local. No external AI APIs, no trackers, no network requests during commit.
- **Smart Diff Parsing**: Dynamically calculates refactoring mood when you delete more code than you write.
- **Sleek Dark Mode SVG**: Generates a premium, responsive widget that matches GitHub's dark theme perfectly.

---

## 📦 Installation

Download the precompiled binary for your system from the [Latest Releases](https://github.com/anilcan-kara/git-mood/releases):

- **macOS (Apple Silicon)**: [git-mood-darwin-arm64](https://github.com/anilcan-kara/git-mood/releases/latest/download/git-mood-darwin-arm64)
- **macOS (Intel)**: [git-mood-darwin-x64](https://github.com/anilcan-kara/git-mood/releases/latest/download/git-mood-darwin-x64)
- **Linux (x64)**: [git-mood-linux-x64](https://github.com/anilcan-kara/git-mood/releases/latest/download/git-mood-linux-x64)
- **Linux (ARM64)**: [git-mood-linux-arm64](https://github.com/anilcan-kara/git-mood/releases/latest/download/git-mood-linux-arm64)
- **Windows (x64)**: [git-mood-win32-x64.exe](https://github.com/anilcan-kara/git-mood/releases/latest/download/git-mood-win32-x64.exe)

Move the downloaded binary to a folder in your system `$PATH` (e.g. `/usr/local/bin` or `%USERPROFILE%\AppData\Local\Microsoft\WindowsApps`) and rename it to `git-mood`.

---

## 🛠️ Usage

### 1. Initialize hook in your project
Navigate to any git repository and initialize the commit-msg hook:
```bash
git-mood init
```

### 2. Write code & commit
Work on your code as usual. When you commit, `git-mood` automatically analyzes the message and prints live feedback:
```bash
git commit -m "feat: add beautiful dashboard ui"

# Output:
# [git-mood] Sentiment Analyzed: Positive (Turquoise) 🚀
#            Diff: +142 / -12 lines
```

### 3. Check your mood dashboard
See a weekly breakdown and stats directly in your terminal:
```bash
git-mood status
```

### 4. Sync to your GitHub Profile
To show your custom mood contribution graph on your GitHub Profile:

1. Clone your special **GitHub Profile Repository** (the repo with the same name as your username, e.g., `github.com/username/username`).
2. Run `git-mood sync` inside that repository:
   ```bash
   git-mood sync
   ```
   This will generate a `git-mood.svg` file, automatically commit it, and push it to your remote repo!
3. Add the following line to your Profile `README.md`:
   ```markdown
   ![git-mood](https://raw.githubusercontent.com/<your-username>/<your-username>/main/git-mood.svg)
   ```

---

## 🛡️ License

MIT License. See [LICENSE](LICENSE) for details.
