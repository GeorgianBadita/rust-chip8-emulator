# rust-chip8-emulator

- [rust-chip8-emulator](#rust-chip8-emulator)
  * [Get Started](#get-started)
  * [Resources](#resources)
  * [Examples](#examples)
    + [Keypad Test](#keypad-test)
    + [IBM Logo Test](#ibm-logo-test)
    + [Pong](#pong)
 
Consolidating my Rust knowledge, by writing an emulator for the Chip8 programming language.

## Get Started
Simply download the project and run: `cargo run <PATH_TO_ROM_FILE>`. Image sclae, number of instructions executed per second and whether to show debug data or not can be tweaked using command line arguments, please see `cargo run chip8_emulator --help` for more information.

## Resources 

[How to write an emulator](https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)

[CHIP8 Interpreter specification](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

## Examples

### Keypad Test
![Keypad Test](https://i.postimg.cc/2j0j4cSS/Screenshot-2022-06-09-at-18-32-13.png)

### IBM Logo Test
![IBM Logo](https://i.postimg.cc/Hkyp1T4r/Screenshot-2022-06-09-at-18-32-29.png)

### Pong
![Pong](https://i.postimg.cc/pdMP23qq/Screenshot-2022-06-09-at-18-32-47.png)

