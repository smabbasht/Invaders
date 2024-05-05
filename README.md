# Invaders

Invaders is a naive rust re-built of classic space invaders game, It has
voiceovers for fun in urdu to capture local context.

## Setup

In order to play the game on your local machine, you should have `rust`
installed. If you don't already have `rust` on your machine, please visit
[installation docs](https://www.rust-lang.org/tools/install) for instructions.
A simple `rustup` installation would do the trick. For Unix-like OSes, you can
run the command below to install `rustup`:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installing rust, you can play the game by following these steps:

```
git clone https://github.com/smabbasht/Invaders/
cd Invaders
cargo run --release
```

## Game Logic
The game logic is simple. Just kill all the invaders through your ships
shooting mechanism before any of them reaches your territory, invading your
space.

## Controls

Controls are pretty intuitive:
| Key | Effect | 
| ---- | ----- | 
| `Left` | Move the spaceship left | 
| `Right` | Move the spaceship right | 
| `Space` or `Enter` | Shoot | 
| `q` or `Esc` | Quit the Game, You lose in this case | 

Have fun!
