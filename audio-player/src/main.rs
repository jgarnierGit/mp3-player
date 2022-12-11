//! Basic player, inspired by symphonia-player & symphonia getting-started
mod symphonia_wrapper;
use log::error;
use std::{env, path::Path};

fn main() {
    // required to have access to deep symphonia error log trace
    pretty_env_logger::init();
    //let music_path = Path::new("./assets/Turn Down the Lights.mp3");
    let music_path =
        Path::new("D:/Documents/prog/rust/mp3Player/audio-project/audio-player/assets/Bully.mp3");
    println!("abs path {:?}", music_path.canonicalize().unwrap());
    //let music_path = Path::new("./assets/MLKDream.mp3");
    println!("{:?}", env::current_dir());
    println!("trying to fetch {:?}", music_path.display());
    symphonia_wrapper::parse(music_path);
    let code = 0;
    /*
    let code = match symphonia_wrapper::playTrack(music_path) {
        Ok(code) => code,
        Err(err) => {
            error!("{}", err.to_string().to_lowercase());
            -1
        }
    };*/
    std::process::exit(code)
}
