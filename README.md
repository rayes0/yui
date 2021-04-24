# yui

A simple and *minimal* UNIX shell written in Rust.

`yui` is primarily interactive focused, although it does have a small parser for light scripting. It's purpose is mainly to help me learn Rust, and the basics of interfacing and managing UNIX processes.

## To build

**You will need:**

- `rustc`
- `cargo`

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

- [ ] *Documentation*
- [X] Run basic commands with args
- [X] Quoted strings: `""` and `''`
- [ ] Simple signal handling
- [ ] Exit code handling
- [X] `~` expansion for homedir
- [ ] Comprehensive Bash-like history expansion:
	- [ ] `!!` history expansion
	- [ ] Bash-like "magic space"
	- [ ] history expansion by both relative and absolute index
	- [ ] Advanced expansion with globbing: `!*`, `!$`, `!^`
	- [X] Reverse history search
- [ ] Differentiate between login and non login shell
- [ ] Job control
  - [ ] `jobs`, `bg` and `fg` builtins
  - [ ] CTRL-Z
- [ ] Run command with temporary environment, eg: `PATH=/bin ls`

**Editing**

- [X] Utilize readline vi or emacs modes to edit lines
- [ ] Support for multi-line commands (with `\`)
- [ ] Support using external editor to edit commands

**Parser**

- [ ] *small* parser for light scripting only, mainly to make the config somewhat bashrc-like (partly finished)
	- [X] Full line comments starting with `#`
	- [ ] Partial line comments
	- [ ] `if` conditionals
- [ ] functions

**Operators and Syntax**

- [ ] `test`, `[`, and `]` for testing conditionals
	- [ ] `==`, `!=`, `<`, `>`
- [ ] AND and OR: `&&` and `||`
- [ ] End of command: `;`
- [ ] Basic arithmetic: `+`, `-`, `/`, `*`, `%`
	- [ ] Follow order of operations
	- [ ] Float calculations
- [ ] Pipes via `|`
- [ ] Redirections via `>` and `<`
- [ ] `\` for escaping characters
- [ ] Support for globs: `*`, `[...]`, `?`, `{...}` etc.
- [ ] Command substitution (subshells?) through `$()` and backticked strings

**Builtins**

- [ ] `echo`
  - [X] Print basic text
  - [ ] Support printing styled text
  - [ ] Support same flags as Bash's builtin version
- [X] `cd`
- [X] `exit`
	- [ ] Specify custom exit code
- [ ] `history`
- [ ] `alias`
- [ ] `exec`
- [X] `export` (env vars)
- [ ] `bind`, to create custom keybinds

**Completion and Hinting**

- [ ] `complete` command for custom completions
- [X] Directory and file completions
- [ ] Command completions
- [X] Automatic completion hinting from history
	- [ ] Intelligent hinting according to cwd
- [ ] Host completion for ssh

**Customization**

- [ ] **Read settings from configuration file**
- [ ] Optional truecolor support
- [ ] Simple prompt customization

**Won't do** (Things that will *not* be implemented into `yui`):

- `for`, `while`, `until`, etc. Any kind of looping - use a proper scripting language for that
- Advanced arithmetic, precision above 8-9 digits, trigonometry, etc. - use an appropriate tool for that
- Advanced/special expansions: `$$`, `${foo/foo/bar}`, `${foo##.*}`, etc. - You should probably be using a proper scripting language if you need these

</details>

---

For those wondering, yes, it is named after the character Yui Hirasawa from the [K-ON! series](https://en.wikipedia.org/wiki/K-On!).
