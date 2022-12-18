//! Basic player, inspired by symphonia-player & symphonia getting-started
mod symphonia_wrapper;
use log::error;
use std::{env, path::Path, time::Instant};

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
    // symphonia_wrapper::parse(music_path);
    let code = 0;
    let start = Instant::now();
    let samples_from_file = symphonia_wrapper::get_file_samples(music_path);
    let duration = start.elapsed();
    if let Some(samples) = samples_from_file {
        println!(
            "print my samples : {} computed in {:?}",
            samples.len(),
            duration
        );
    }

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
