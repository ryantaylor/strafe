# strafe

`strafe` is a developmental tool meant to assist in reverse engineering the format of Company of Heroes 3 replay commands. It was built using [vault](https://github.com/ryantaylor/vault) and intended to improve that library's replay parsing by making it easier to understand how replay commands are formatted.

## Usage

Compile:
```
$ cargo build --release
```

Then run:
```
$ target/release/strafe
Usage: strafe <FILE>

Arguments:
  <FILE>  Path to a CoH3 replay file

Options:
  -h, --help     Print help
  -V, --version  Print version
```

If you want to prevent lines from wrapping in order to make it easier to compare different commands, you can pipe the output of the command to `less`:
```
$ target/release/strafe /path/to/replay | less -S
```
