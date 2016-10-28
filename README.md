# emojy

A brainfuck dialect using emojis.

## Commands

It uses the follwing mapping:

| Brainfuck command | Emoji |
|-------------------|-------|
| +                 | 👍    |
| -                 | 👎    |
| >                 | 👆    |
| <                 | 👇    |
| ,                 | 👄    |
| .                 | 👂    |
| [                 | 🔁    |
| ]                 | 🔚    |

## Example Scripts

Two example scripts are included in the `examples/` directory: `rot13.em` and `hello_world.em`. They are the emojy version of the brainfuck examples from [wikipedia](https://en.wikipedia.org/wiki/Brainfuck).

## Usage

You need a working installation of [rust](https://rustlang.org) and [cargo](https://cargo.io). Then build it using

```
cargo build
```

After that it can be run using

```
target/debug/emojy [TRANSLATION_FILE] EMOJY_FILE
```

`TRANSLATION_FILE` is the optional path to the translation file (if not given, `./translation` is used). `EMOJY_FILE` is the path to the source.

## License

The Code is under a GPL license.