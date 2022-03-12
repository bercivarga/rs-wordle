# [W][O][R][D][L][E] &mdash; Rust-based Wordle game, in your terminal!

Ever heard of the world-wide phenomenon, [Wordle](https://www.nytimes.com/games/wordle/index.html) from the good folks at The New York Times? If not, here's a short rundown on what the game is:

In Wordle, your goal is to guess a random 5-letter word in 6 tries. With each guess, you reveal more information about the word by getting feedback on the characters you entered: When a character is not in the word, it's marked with a red color, if it is in the word but in the wrong place, then it's marked with a yellow color, and if the word is in the word and in the correct place, then it's drawn in green. The secret word changes every day.

This application takes the original Wordle concept and puts it right in your terminal.

```bash
[A][L][I][E][N] ❌ ❌ ❌ ❌ ❌
[S][T][A][R][S] 🟨 ❌ ❌ ❌ 🟨
[H][U][S][K][Y] ✅ ✅ ✅ ✅ ✅
---------------
You won 🎉
```

---

## Play the game via the prebuilt executable

If you don't want to build the app yourself, I've included a prebuilt executable in the `release/` folder that you can just run once you downloaded it.

To proceed, clone the repo:

```bash
git clone https://github.com/bercivarga/rs-wordle.git
cd rs-wordle
```

Then, navigate to the `release/` folder to find the `rs-wordle` executable.
You can open the binary executable with the `open` command.

```bash
cd release
open rs-wordle
```

Then play the game! You can always exit with `ctrl + c`.

---
## Run and/or install the app yourself

In order to install this program in its current state, you need to have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.

Then, you have some options:

### 1. Run once without installation

If you don't want the executable installed on your system, then simply run the following command in the root directory:

```bash
cargo run
```

Then start playing the game!

---

### 2. Install binary on your system

Run the command below to install the `rs-wordle` executable and command in `$HOME/.cargo/bin` on Unix based systems or `%USERPROFILE%\.cargo\bin` on Windows:

```bash
cargo install --path .
```

---

#### Have fun! 🦧