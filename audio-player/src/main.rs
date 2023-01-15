//! Basic player, inspired by symphonia-player & symphonia getting-started
mod symphonia_wrapper;
use args::Cli;
use log::error;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::thread::{self};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

mod args;
use clap::Parser;

fn main() {
    // required to have access to deep symphonia error log trace
    pretty_env_logger::init();
    let args = Cli::parse();

    let code = match run(&args) {
        Ok(code) => code,
        Err(err) => {
            error!("{}", err.to_string().to_lowercase());
            -1
        }
    };
    std::process::exit(code)
}

fn run(args: &Cli) -> Result<i32, Box<dyn std::error::Error>> {
    let music_path = Path::new(&args.input);

    if args.analyze {
        symphonia_wrapper::parse(music_path);
    }
    if args.full_audio_sample {
        process_audio_sample(music_path);
    }
    if args.live_audio_sample {
        process_live_audio_sample(music_path);
    }
    if let Some(tags) = args.tag.as_deref() {
        process_tag(music_path, tags);
    }
    if args.play {
        return process_play(music_path);
    }
    Ok(1)
}

fn process_audio_sample(music_path: &Path) {
    let samples_from_file = symphonia_wrapper::get_file_samples(music_path);
    if let Some(samples) = samples_from_file {
        println!("Audio samples : {}", samples.len());
    } else {
        println!("No sample found for {:?}", music_path);
    }
}

fn process_live_audio_sample(music_path: &Path) {
    let (tx, rx) = channel::<(usize, usize, Vec<f32>)>();

    let mut live_sample_written = Rc::new(1);

    let player_h = symphonia_wrapper::get_live_sample(music_path, tx, &mut live_sample_written);
    let listener_h = thread::spawn(move || {
        for (packet_id, _buffer_len, _buffer_data) in rx {
            println!("{:?}", packet_id);
        }
    });
    player_h.join().unwrap();
    listener_h.join().unwrap();
}

fn process_tag(music_path: &Path, tags: &[String]) {
    let iter_tags = tags.into_iter();
    for tag in iter_tags {
        // TODO next improvment is to pass tags list directly and not reopening file each time.
        let res = symphonia_wrapper::get_metadata_string(music_path, &tag).unwrap();
        println!("tag :{}= {}", tag, res);
    }
}

fn process_play(music_path: &Path) -> Result<i32, Box<dyn std::error::Error>> {
    let temp_param_1 = Arc::new(Mutex::new((0_u64, (0_u64, 0_u64, 0_f64))));
    let temp_param_2 = Arc::new(Mutex::new(true));
    let code = match symphonia_wrapper::playTrack(music_path, temp_param_1, temp_param_2) {
        Ok(code) => Ok(code),
        Err(err) => Err(err.into()),
    };
    return code;
}
