# yui

A simple and *minimal* **interactive** UNIX shell written in Rust. It was written to help me learn Rust as well as the basics of UNIX shells and process management.

## Planned Features

<details>
<summary>Click to expand!</summary>

**Core**

- [X] Run basic commands with args
- [ ] *small* parser for light scripting only
- [ ] Quoted strings: `""` and `''`
- [ ] Simple signal handling
- [ ] Exit code handling
- [ ] Command substitution through `$()` and backticked strings
- [ ] `~` expansion for homedir
- [ ] `!!` history expansion
- [ ] Handle line wrapping properly for commands longer than the terminal window (Take into account the length of the prompt)
- [ ] Implement job control
  - [ ] `jobs`, `bg` and `fg` commands
  - [ ] CTRL-z

**Editing**

- [ ] Utilize readline vi and emacs modes to edit lines
- [ ] Support for multi-line commands (with `\`)
- [ ] Support using external editor to edit commands

**Operators**

- [ ] AND and OR: `&&` and `||`, also negations (`!`)
- [ ] End of command: `;`
- [ ] Basic arithmetic: `+`, `-`, `/`, `*`, `%`
- [ ] Pipes via `|`
- [ ] Redirections via `>` and `<`, (maybe `>>` and `<<` too for convenience)

**Builtins**

- [ ] `echo`
  - [X] Print basic text
  - [ ] Support for escape sequences
  - [ ] Support same flags as Bash's builtin version
- [X] `cd`
- [X] `exit`
- [ ] `history`
- [ ] `alias`
- [ ] `exec`
- [ ] `export` (env vars)

**Completions**

- [ ] `complete` command
- [ ] Directory completions

**Customization**

- [ ] Simple prompt customization
- [ ] Read settings from configuration file
</details>

---

For those wondering, yes, it is named after the character Yui Hirasawa from the [K-ON! series](https://en.wikipedia.org/wiki/K-On!).
