# [W][O][R][D][L][E] &mdash; Rust-based Wordle game, in your terminal!

Ever head of the world-wide browser game phenomenon, [Wordle](https://www.nytimes.com/games/wordle/index.html) by The New York Times? If not, here's a short rundown on what the game is:

In Wordle, your goal is to guess a random 5-letter word in 6 tries. With each guess, you reveal more information about the word by getting feedback on the characters you entered: When a character is not in the word, it's marked with a red color, if it is in the word but in the wrong place, then it's marked with a yellow color, and if the word is in the word and in the correct place, then it's drawn in green. The secret word changes every day.

This application takes the original Wordle concept and puts it right in your terminal.

---

## Install

In order to install this program in its current state, you need to have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.

Then, you have some options:

### 1. Run once without installation

If you don't want the executable installed on your system, then simply clone the project and run the following command in the root directory:

```bash
cargo run
```

Then start playing the game!

---

### 2. Install binary on your system

Running the command below will install the `rs-wordle` executable and command in `$HOME/.cargo/bin` on Unix based systems or `%USERPROFILE%\.cargo\bin` on Windows

```bash
cargo install --path .
```

---

#### Have fun! 🦧