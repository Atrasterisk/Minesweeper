# Minesweeper Soroban Smart Contract 💣

This project is an implementation of the classic **Minesweeper** game using the [Soroban SDK](https://soroban.stellar.org/) for the Stellar network. This smart contract allows players to initialize a game board, reveal squares, and track the game status (win/loss) *on-chain*.

## 🌟 Key Features

- **Dynamic Initialization (`init_game`)**: Players can create a game board with customizable dimensions (*width* & *height*) and number of mines (*mine_count*).
- **Safety System**: Ensures the number of mines does not exceed or equal the total size of the game board (preventing logic bugs).
- **Reveal Mechanic (`reveal`)**: Reveals a square at coordinates `(x, y)`. Returns the number of adjacent mines (0-8), or `99` if the player hits a mine (BOOM!).
- **Game Status (`get_status`)**: Checks the condition of the board, the existing mines, and the squares that have already been revealed by the player.
- **Secure Storage**: Utilizes Soroban's `persistent storage` so that each user's game state is safely stored and mapped specifically to their address.

## 🛠 Prerequisites

Before you start compiling or deploying, make sure you have installed:
1. [Rust](https://www.rust-lang.org/tools/install)
2. WebAssembly (WASM) Target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
## 🚀 Testnet Deployment
Contract ID: CDRKQG4F6KR66YMYARWRSYKZIZP2KC64JVEOPQAM674ARGY5TLF3BCGF
