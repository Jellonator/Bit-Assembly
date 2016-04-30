# Bit-Assembly
An assembly language based on bits

Check out doc.md for help on how to write in bit assembly.
Use `bit-asm.exe --help` for help on how to use the interpreter.

## Why should I use Bit Assembly?
Why should you use Bit Assembly? That is a great question, and I am glad I live in a country that can ask questions like these. Folks, I promise to build the best language I can, ok? We will make assembly language great again.

In all seriousness though, there is no reason to use this other than as a joke.

## Building:
 * Make sure you have Rust, Cargo, and GMP installed.
 * use `cargo build --release` to build the program into target/release/

You can now execute Bit Assembly with `cargo run -- {flags}` or `./target/release/bit-asm {flags}`.

## Hello World:
create a file called `test.asm`, and set it's contents to the following:
```asm
push 512
mov [0:512], "Hello, World!\n"
ext print, [0:512]
```

After this, run `cargo run -- --file test.asm` to test out this program. You should see the text "Hello, World!" appear on the command line.

To understand this program, I will break it down step-by-step:
 * `push 512` will push 512 bits onto the stack, which is enough for 64 characters.
 * `mov [0:512], "Hello, World!\n"` will move the string value into the newly allocated space of 512 bits. Since the specified string is shorter than 64 characters, this will work fine.
 * `ext print, [0:512]` will call the external 'print' function, which can take a value and print it as a string, provided that it is a valid UTF-8 string.

Note that the entire program can be condensed into `ext print, "Hello, World!\n"`, but the extra stuff is there for example.
