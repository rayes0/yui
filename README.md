# yui

A simple and *minimal*, primarily interactive-focused UNIX shell written in Rust, although it does have a small parser for light scripting. It's purpose is mainly to help me learn Rust, and the basics of interfacing and managing UNIX processes.

## To build

**You will need:**

- `rustc`
- `cargo` (***nightly*** *toolchain*)

Clone the repo:

```sh
git clone https://github.com/rayes0/yui.git
cd yui
```

If you just want to try yui (dev version):

```sh
cargo run
```

**Building**:

```sh
cargo build --release
```

## Features (Current and Planned)

<details>
<summary>Click to expand!</summary>

**Core**

- [X] Run basic commands with args
- [X] Quoted strings: `""` and `''`
	- [ ] Nested quotes? (don't know if this is feasible, and normal Bash doesn't do it)
- [ ] Simple signal handling
- [ ] Exit code handling
- [ ] Command substitution through `$()` and backticked strings
- [X] `~` expansion for homedir
- [ ] `!!` history expansion
- [ ] Differentiate between login and non login shell
- [ ] Implement job control
  - [ ] `jobs`, `bg` and `fg` builtins
  - [ ] CTRL-Z
- [ ] *small* parser for light scripting only, mainly to make the config somewhat bashrc-like (partly finished)
	- [ ] Full line comments starting with `#`

**Editing**

- [X] Utilize readline vi or emacs modes to edit lines
- [ ] Support for multi-line commands (with `\`)
- [ ] Support using external editor to edit commands

**Operators**

- [ ] AND and OR: `&&` and `||`
- [ ] End of command: `;`
- [ ] Basic arithmetic: `+`, `-`, `/`, `*`, `%`
- [ ] Pipes via `|`
- [ ] Redirections via `>` and `<`, (maybe `>>` and `<<` too)

**Builtins**

- [ ] `echo`
  - [X] Print basic text
  - [ ] Support printing styled text
  - [ ] Support same flags as Bash's builtin version
- [X] `cd`
- [X] `exit`
- [ ] `history`
- [ ] `alias` or `function`
- [ ] `exec`
- [X] `export` (env vars)
- [ ] `bind`, to create custom keybinds

**Completions**

- [ ] `complete` command
- [X] Directory completions
- [ ] Command completions
- [X] Automatic completion hinting from history

**Customization**

- [ ] Optional truecolor support
- [ ] Simple prompt customization
- [ ] Read settings from configuration file

</details>

---

For those wondering, yes, it is named after the character Yui Hirasawa from the [K-ON! series](https://en.wikipedia.org/wiki/K-On!).
