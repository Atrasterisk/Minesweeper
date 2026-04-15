#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, Env, Vec, panic_with_error
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    GameNotFound = 1,
    GameOver = 2,
    OutOfBounds = 3,
    InvalidBoard = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Game(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameBoard {
    pub width: u32,
    pub height: u32,
    pub mines: Vec<u32>,
    pub revealed: Vec<u32>,
    pub is_over: bool,
}

#[contract]
pub struct MinesweeperContract;

#[contractimpl]
impl MinesweeperContract {
    /// Inisialisasi permainan baru untuk pemain (player)
    pub fn init_game(env: Env, player: Address, width: u32, height: u32, mine_count: u32) {
        player.require_auth();

        let total_cells = width * height;
        
        // Validasi agar jumlah ranjau tidak melebihi atau sama dengan kapasitas papan
        if mine_count >= total_cells {
            panic_with_error!(&env, Error::InvalidBoard);
        }

        let mut mines = Vec::new(&env);
        let mut count = 0;
        
        // Penempatan ranjau secara acak
        while count < mine_count {
            // Menggunakan Turbofish ::<u64> untuk mendikte tipe data secara absolut ke compiler
            let rand_pos = env.prng().gen_range::<u64>(0..(total_cells as u64)) as u32;
            
            if !mines.contains(&rand_pos) {
                mines.push_back(rand_pos);
                count += 1;
            }
        }

        let game = GameBoard {
            width,
            height,
            mines,
            revealed: Vec::new(&env),
            is_over: false,
        };

        // Menggunakan persistent storage
        env.storage().persistent().set(&DataKey::Game(player), &game);
    }

    /// Membuka kotak pada koordinat x dan y
    pub fn reveal(env: Env, player: Address, x: u32, y: u32) -> u32 {
        player.require_auth();

        let key = DataKey::Game(player.clone());
        let mut game: GameBoard = env.storage().persistent().get(&key)
            .unwrap_or_else(|| panic_with_error!(&env, Error::GameNotFound));

        if game.is_over {
            panic_with_error!(&env, Error::GameOver);
        }

        if x >= game.width || y >= game.height {
            panic_with_error!(&env, Error::OutOfBounds);
        }

        let index = y * game.width + x;

        // Cek apakah pemain mengenai ranjau
        if game.mines.contains(&index) {
            game.is_over = true;
            env.storage().persistent().set(&key, &game);
            return 99; // 99 melambangkan ledakan (BOOM)
        }

        // Tandai kotak sebagai terbuka
        if !game.revealed.contains(&index) {
            game.revealed.push_back(index);
        }

        // Hitung ranjau di kotak tetangga (maksimal 8)
        let mut mine_count = 0;
        for i in -1..=1i32 {
            for j in -1..=1i32 {
                if i == 0 && j == 0 { continue; }
                
                let nx = x as i32 + i;
                let ny = y as i32 + j;

                // Pastikan tetangga berada di dalam batas papan
                if nx >= 0 && nx < game.width as i32 && ny >= 0 && ny < game.height as i32 {
                    let n_idx = (ny as u32) * game.width + (nx as u32);
                    if game.mines.contains(&n_idx) {
                        mine_count += 1;
                    }
                }
            }
        }

        // Simpan status game (state) terbaru ke storage
        env.storage().persistent().set(&key, &game);
        
        // Mengembalikan jumlah ranjau di sekitar (0-8)
        mine_count 
    }

    /// Mendapatkan status papan game pemain saat ini
    pub fn get_status(env: Env, player: Address) -> GameBoard {
        env.storage().persistent().get(&DataKey::Game(player))
            .unwrap_or_else(|| panic_with_error!(&env, Error::GameNotFound))
    }
}