# Minesweeper Soroban Smart Contract 💣

Proyek ini adalah implementasi permainan klasik **Minesweeper** menggunakan [Soroban SDK](https://soroban.stellar.org/) untuk jaringan Stellar. Smart contract ini memungkinkan pemain untuk menginisialisasi papan permainan, membuka kotak, dan melacak status permainan (menang/kalah) secara *on-chain*.

## 🌟 Fitur Utama

- **Inisialisasi Dinamis (`init_game`)**: Pemain dapat membuat papan permainan dengan ukuran (*width* & *height*) dan jumlah ranjau (*mine_count*) yang dapat dikustomisasi.
- **Sistem Keamanan**: Memastikan jumlah ranjau tidak melebihi atau sama dengan ukuran papan permainan (mencegah *bug* logika).
- **Mekanik Reveal (`reveal`)**: Membuka kotak di koodinat `(x, y)`. Mengembalikan jumlah ranjau di sekitar kotak (0-8), atau `99` jika pemain mengenai ranjau (BOOM!).
- **Status Permainan (`get_status`)**: Mengecek kondisi papan, ranjau yang ada, dan kotak yang sudah dibuka oleh pemain.
- **Penyimpanan Aman**: Menggunakan `persistent storage` dari Soroban agar *state* permainan masing-masing *user* tersimpan dengan aman dan spesifik berdasarkan alamat (*address*) mereka.

## 🛠 Prasyarat

Sebelum mulai melakukan *compile* atau *deploy*, pastikan Anda sudah menginstal:
1. [Rust](https://www.rust-lang.org/tools/install)
2. Target WebAssembly (WASM):
   ```bash
   rustup target add wasm32-unknown-unknown