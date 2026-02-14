# obsidian-tidy

> WORKING IN PROGRESS

**A fast, incremental linter for Obsidian vaults, written in Rust.**  
Inspired by `clang-tidy` and built for power users with large knowledge bases.

[![GitHub release](https://img.shields.io/github/v/release/CryptoGladi/obsidian-tidy)](https://github.com/CryptoGladi/obsidian-tidy/releases)
[![Build status](https://img.shields.io/github/actions/workflow/status/CryptoGladi/obsidian-tidy/ci.yml)](https://github.com/CryptoGladi/obsidian-tidy/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-orange.svg)](https://www.rust-lang.org)

## Why obsidian-tidy?

The existing [obsidian-linter](https://github.com/platers/obsidian-linter) plugin works well, but if you have thousands of notes you might feel the performance drag.  
`obsidian-tidy` is a **standalone binary** that brings true **incremental analysis**, **full multi‑threading**, and **zero overhead** to your workflow.  

- 🚀 **Blazing fast** – written in Rust, uses all CPU cores via `rayon`.
- ⚡ **Incremental** – only checks changed files; caches the note graph.
- 🔌 **Standalone binary** – no Electron, no Obsidian API – direct filesystem access.
- 🧩 **Custom rules in Lua** – write your own checks without recompiling.
- 🐙 **Git pre‑commit hooks** – stop broken notes from being committed.
- 🔁 **One‑command migration** – import all your settings from the JS plugin.
- 🏭 **Nix‑powered builds** – reproducible, cross‑platform binaries.

## Features

### 🏎 Performance
- **Compiled to native code** – no interpreter overhead.
- **Multi‑threaded analysis** – thanks to [`obsidian-parser`](https://github.com/CryptoGladi/obsidian-parser) and Rayon.
- **Direct filesystem access** – bypass any abstraction layers.

### 🔍 Incremental checking
- First run creates a cache of your vault.
- Subsequent runs only process changed files and update the cached graph.
- Ideal for vaults with 5000+ notes.

### 🧠 Custom rules with Lua
- Define rules in easy‑to‑read Lua scripts.
- Access the full note content, frontmatter, and even the link graph.
- Auto‑fix violations directly from your rule.
- Why Lua? Lightweight, embeddable, and user‑friendly – no compilation needed.

### 🔁 Seamless migration from the JS plugin
```bash
obsidian-tidy migrate --from js-linter
```

### 🐙 Git integration
- Run checks as a pre-commit hook – prevent commits that break your vault.
- Use `.gitignore` patterns to skip files automatically.

### 🚫 Flexible ignoring
- Respect `.gitignore`
- Custom ignore file (`.obtidyignore`)
- Per‑rule ignore patterns
- Ignore notes with specific tags

## Installation

### Pre‑built binaries

Download the latest release for your platform from the releases page:

### Using Nix (recommended for NixOS / home‑manager users)
```bash
# Run directly
nix run github:CryptoGladi/obsidian-tidy

# Or install it permanently
nix profile add github:CryptoGladi/obsidian-tidy
```

## Usage

### Initialize a configuration file
```bash
obsidian-tidy init
```

Creates a default `.obsidian-tidy.toml` in the current directory (usually your vault root).

### Run linter
```bash
obsidian-tidy check
```

### Other useful commands
```bash
# List all available built‑in rules
obsidian-tidy list-rules
```

## Configuration
```toml
[general]
ignore = [ "templates/**", "*.tmp.md" ]
respect_gitignore = true

[rules]
# Built‑in rules
"yaml.title" = "warn"
"yaml.timestamp" = { level = "error", format = "YYYY-MM-DD" }
"heading.capitalize" = "off"

# Custom Lua rule
[rules.custom]
path = "rules/my-rule.lua"
level = "warn"
```

Custom Lua rule example (`rules/my-rule.lua`):
```lua
-- Rule: every note must have a "status" tag in frontmatter
function check(note)
    if not note.frontmatter or not note.frontmatter.tags then
        return { message = "Missing tags field" }
    end
    local tags = note.frontmatter.tags
    if type(tags) == "string" then tags = { tags } end
    if not lib.tbl_contains(tags, "status") then
        return { message = "Missing 'status' tag" }
    end
    return nil  -- no violation
end

-- Optional auto‑fix
function fix(note)
    if not note.frontmatter then note.frontmatter = {} end
    if not note.frontmatter.tags then
        note.frontmatter.tags = { "status" }
    elseif type(note.frontmatter.tags) == "string" then
        note.frontmatter.tags = { note.frontmatter.tags, "status" }
    else
        table.insert(note.frontmatter.tags, "status")
    end
    return note
end
```

## License

MIT © [CryptoGladi](https://github.com/CryptoGladi)

**obsidian-tidy** – Because your knowledge base deserves a fast linter.
