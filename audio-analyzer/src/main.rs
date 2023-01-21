use audio_player::{MetadataParserBuilder, MetadataParserWrapper};
use std::path::PathBuf;

use std::path::Path;
use std::sync::{Arc, Mutex};

mod pixel_buf;
mod spectrum_analyzer;
use crate::spectrum_analyzer::*;
use audio_player::playTrack;

use std::thread::{self};

use aubio_rs::Smpl;
use aubio_rs::Tempo;
use hound::WavReader;

const BUF_SIZE: usize = 512;
const HOP_SIZE: usize = 256;
const I16_TO_SMPL: Smpl = 1.0 / (1 << 16) as Smpl;
mod args;
use args::Cli;
use clap::Parser;
use log::error;

fn main() {
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
    let metadata_parser = MetadataParserBuilder::build();
    let beat_algo = match args.beat_detection.as_str() {
        "Energy" => aubio_rs::OnsetMode::Energy,
        "Hfc" => aubio_rs::OnsetMode::Hfc,
        "Complex" => aubio_rs::OnsetMode::Complex,
        "Phase" => aubio_rs::OnsetMode::Phase,
        "WPhase" => aubio_rs::OnsetMode::WPhase,
        "SpecDiff" => aubio_rs::OnsetMode::SpecDiff,
        "Kl" => aubio_rs::OnsetMode::Kl,
        "Mkl" => aubio_rs::OnsetMode::Mkl,
        "SpecFlux" => aubio_rs::OnsetMode::SpecFlux,
        _ => aubio_rs::OnsetMode::SpecFlux,
    };
    let beats = get_wav_beats(music_path, beat_algo);
    if args.visualization {
        static_spectrum_display(music_path, beats, metadata_parser);
    } else if args.live_visualization {
        live_spectrum_display(music_path, beats, metadata_parser);
        return Ok(1);
    } else if args.save_spectrum {
        if let Some(output_path) = args.output.as_deref() {
            save_as_png(music_path, output_path, beats, metadata_parser);
            return Ok(1);
        }
    }
    Ok(1)
}

fn get_wav_beats(music_path: &Path, beat_algo: aubio_rs::OnsetMode) -> Vec<f64> {
    // example taken from https://github.com/katyo/aubio-rs/blob/master/src/tempo.rs
    let mut reader = WavReader::open(music_path).unwrap();
    let format = reader.spec();
    let mut samples = reader.samples::<i16>();
    let period = 1.0 / format.sample_rate as Smpl;
    let mut time: f32 = 0.0;
    let mut offset = 0;

    let mut tempo = Tempo::new(beat_algo, BUF_SIZE, HOP_SIZE, format.sample_rate).unwrap();

    let mut beats: Vec<f64> = Vec::new();
    loop {
        let block = samples
            .by_ref()
            .map(|sample| sample.map(|sample: i16| sample as Smpl * I16_TO_SMPL))
            .take(HOP_SIZE)
            .collect::<Result<Vec<Smpl>, _>>()
            .unwrap();

        if block.len() == HOP_SIZE {
            let beat = tempo.do_result(block.as_slice().as_ref()).unwrap();
            if beat != 0.0 {
                beats.push(time as f64);
            }
        }

        offset += block.len();
        time = offset as Smpl * period;

        if block.len() < HOP_SIZE {
            break;
        }
    }
    beats
}

fn static_spectrum_display(
    music_path: &Path,
    beats: Vec<f64>,
    metadata_parser: Box<dyn MetadataParserWrapper>,
) {
    if let Some((samples_formatted, beats_formatted, _frame_rate, _channel_nb, _frame_number)) =
        analyze_samples(&metadata_parser, music_path, &beats)
    {
        draw_static_into_window(music_path, &samples_formatted, &beats_formatted).unwrap();
    }
}

fn live_spectrum_display(
    music_path: &Path,
    beats: Vec<f64>,
    metadata_parser: Box<dyn MetadataParserWrapper>,
) {
    if let Some((samples_formatted, beats_formatted, _frame_rate, _channel_nb, _frame_number)) =
        analyze_samples(&metadata_parser, music_path, &beats)
    {
        live_play(music_path.to_path_buf(), samples_formatted, beats_formatted);
    }
}

fn save_as_png(
    music_path: &Path,
    output_path: &Path,
    beats: Vec<f64>,
    metadata_parser: Box<dyn MetadataParserWrapper>,
) {
    if let Some((samples_formatted, beats_formatted, _frame_rate, _channel_nb, _frame_number)) =
        analyze_samples(&metadata_parser, music_path, &beats)
    {
        draw_into_png(output_path, &samples_formatted, &beats_formatted).unwrap();
    }
}

fn live_play(music_path: PathBuf, samples_formatted: Box<Vec<f32>>, beats_formatted: Vec<f64>) {
    let music_path_1 = music_path.clone();
    let music_path_2 = music_path.clone();
    let sound_sync = Arc::new(Mutex::new(false));
    let ts_sound_sync = Arc::new(Mutex::new((0_u64, (0_u64, 0_u64, 0_f64))));
    let sound_sync_listener = Arc::clone(&sound_sync);
    let ts_sync_emit = Arc::clone(&ts_sound_sync);
    let player_t = thread::spawn(move || {
        playTrack(
            music_path_1.as_path(),
            ts_sync_emit,
            Arc::clone(&sound_sync_listener),
        )
        .unwrap();
        let mut sound_sync_finished = sound_sync_listener
            .try_lock()
            .expect("couldn't lock at end of play");
        *sound_sync_finished = false;
    });
    let sound_sync_launcher = Arc::clone(&sound_sync);
    let ts_sync_listen = Arc::clone(&ts_sound_sync);
    let windows_t = thread::spawn(move || {
        draw_live_cursor_into_window(
            music_path_2.as_path(),
            &samples_formatted,
            &beats_formatted,
            ts_sync_listen,
            sound_sync_launcher,
        )
        .unwrap();
    });
    player_t.join().unwrap();
    windows_t.join().unwrap();
}
