use graphtorio_game::data::parsing::RawGameData;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("data.bin");

    let data_path = PathBuf::from("./data").join("files");
    let game_data_path = data_path.join("game_data.yaml");

    let raw_game_data = RawGameData::load_yaml(&game_data_path).unwrap();
    let serialized = bincode::encode_to_vec(&raw_game_data, bincode::config::standard()).unwrap();
    let compressed = zstd::encode_all(&serialized[..], 22).unwrap();

    let mut file = File::create(dest_path).unwrap();
    file.write_all(&compressed).unwrap();
}
