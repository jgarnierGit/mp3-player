//!
//! known limitation :
//! I had to add a hardcoded 48000Hz to sample spec in order to play it successfully on windows.(current configuration is : 2 canals, 24bits, 48000Hz)
//! It is not really noticable as most music rate are near this one, but still, a resampling would be great (as all player does I guess)
//! For now I keep in mind those issues :
//!    https://github.com/RustAudio/cpal/issues/593#issuecomment-1185260068
//! bug detected https://github.com/pdeljanov/Symphonia/issues/43 =>merged look for a 0.5.2 ! (example in symphonia-play)
//!
//! If I want to go deeper on this way,
//!  cpal::platform::Device::supported_output_configs(); => to get windows configuration
//! use cpal::default_host;
//! use cpal::platform::Device;
//! + do the resampling...

use log::warn;
use std::path::Path;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{DecoderOptions, FinalizeResult, CODEC_TYPE_NULL};
use symphonia::core::errors::{Error, Result};
use symphonia::core::formats::{FormatReader, Track};
use symphonia::core::units::TimeBase;

use lazy_static::lazy_static;
use std::io::Write;

use super::commons::{self};
use super::output;

#[derive(Copy, Clone)]
struct PlayTrackOptions {
    track_id: u32,
    seek_ts: u64,
}

pub fn get_file_samples(audio_path: &Path) -> Option<Box<Vec<f32>>> {
    let mut probed = match commons::get_probe(audio_path) {
        Ok(probe) => probe,
        Err(err) => panic!("Unsupported format {}", err),
    };
    let mut format = probed.format;
    // Get the default track.
    let track = format.default_track().unwrap();
    let track_id = track.id;
    let decode_opts: DecoderOptions = Default::default();
    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decode_opts)
        .expect("unsupported codec");
    let mut sample_buf = None;
    let mut sample_array = Box::new(Vec::new());

    let result = loop {
        // Get the next packet from the format reader.
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(err) => break Err(err),
        };

        // If the packet does not belong to the selected track, skip it.
        if packet.track_id() != track_id {
            continue;
        }
        // Decode the packet into audio samples, ignoring any decode errors.
        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                // The decoded audio samples may now be accessed via the audio buffer if per-channel
                // slices of samples in their native decoded format is desired. Use-cases where
                // the samples need to be accessed in an interleaved order or converted into
                // another sample format, or a byte buffer is required, are covered by copying the
                // audio buffer into a sample buffer or raw sample buffer, respectively. In the
                // example below, we will copy the audio buffer into a sample buffer in an
                // interleaved order while also converting to a f32 sample format.

                // If this is the *first* decoded packet, create a sample buffer matching the
                // decoded audio buffer format.
                if sample_buf.is_none() {
                    // Get the audio buffer specification.
                    let spec = *audio_buf.spec();

                    // Get the capacity of the decoded buffer. Note: This is capacity, not length!
                    let duration = audio_buf.capacity() as u64;

                    // Create the f32 sample buffer.
                    sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
                }
                // Copy the decoded audio buffer into the sample buffer in an interleaved format.
                if let Some(buf) = &mut sample_buf {
                    buf.copy_interleaved_ref(audio_buf);

                    sample_array.extend_from_slice(buf.samples());
                }
            }
            Err(err) => break Err(err),
        }
    };
    // Return if a fatal error occured.
    ignore_end_of_stream_error(result).unwrap();
    Some(sample_array)
}

pub fn get_live_sample(
    audio_path: &Path,
    live_sample: Sender<(usize, usize, Vec<f32>)>, // FIXME with an Arc<Mutex> to avoid desync
    live_sample_written: &mut Rc<usize>,
) -> JoinHandle<()> {
    let audio_p = audio_path.clone().to_owned();

    let closure_get_live_sample =
        move |packet_idx: usize, sample_buffer: &mut SampleBuffer<f32>| {
            live_sample
                .send((
                    packet_idx,
                    sample_buffer.len(),
                    sample_buffer.samples().to_vec(),
                ))
                .unwrap();
        };

    let handler = thread::spawn(move || {
        apply_on_sample(audio_p.as_path(), closure_get_live_sample);
    });
    handler
}

/// No sound feedback
///
pub fn apply_on_sample<T>(audio_path: &Path, mut callback: T)
where
    T: FnMut(usize, &mut SampleBuffer<f32>),
{
    let mut probed = match commons::get_probe(audio_path) {
        Ok(probe) => probe,
        Err(err) => panic!("Unsupported format {}", err),
    };
    let mut format = probed.format;
    // Get the default track.
    let track = format.default_track().unwrap();
    let track_id = track.id;
    let decode_opts: DecoderOptions = Default::default();
    let play_opts = PlayTrackOptions {
        track_id: track_id,
        seek_ts: 0.0 as u64,
    };
    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decode_opts)
        .expect("unsupported codec");
    let mut sample_buf = None;

    let mut packet_idx: usize = 0;

    let result = loop {
        // Get the next packet from the format reader.
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(err) => break Err(err),
        };

        // If the packet does not belong to the selected track, skip it.
        if packet.track_id() != track_id {
            continue;
        }
        // Decode the packet into audio samples, ignoring any decode errors.
        match decoder.decode(&packet) {
            Ok(mut audio_buf) => {
                // The decoded audio samples may now be accessed via the audio buffer if per-channel
                // slices of samples in their native decoded format is desired. Use-cases where
                // the samples need to be accessed in an interleaved order or converted into
                // another sample format, or a byte buffer is required, are covered by copying the
                // audio buffer into a sample buffer or raw sample buffer, respectively. In the
                // example below, we will copy the audio buffer into a sample buffer in an
                // interleaved order while also converting to a f32 sample format.

                // If this is the *first* decoded packet, create a sample buffer matching the
                // decoded audio buffer format.
                if sample_buf.is_none() {
                    // Get the audio buffer specification.
                    let spec = *audio_buf.spec();

                    // Get the capacity of the decoded buffer. Note: This is capacity, not length!
                    let duration = audio_buf.capacity() as u64;

                    // Create the f32 sample buffer.
                    sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
                }
                // Copy the decoded audio buffer into the sample buffer in an interleaved format.
                if let Some(buf) = &mut sample_buf {
                    packet_idx += 1;
                    buf.copy_interleaved_ref(audio_buf);
                    callback(packet_idx, buf);
                }
            }
            Err(err) => break Err(err),
        }
    };
    // Return if a fatal error occured.
    ignore_end_of_stream_error(result).unwrap();
}

pub fn play_track(
    music_path: &Path,
    live_sample: Arc<Mutex<(u64, (u64, u64, f64))>>,
    start_play: Arc<Mutex<bool>>,
) -> Result<i32> {
    let probed = match commons::get_probe(music_path) {
        Ok(probe) => probe,
        Err(err) => panic!("Unsupported format {}", err),
    };

    // Get the instantiated format reader.
    let format = probed.format;

    // Get the value of the track option, if provided.
    let track = None;
    let seek_time = None;
    // Set the decoder options.
    let decode_opts: DecoderOptions = Default::default();
    // The audio output device. First is None
    let mut audio_output = None;
    {
        if let Ok(start_p) = start_play.try_lock() {
            println!("received starting to play !!");
        } else {
            println!("couldn't lock for playing");
        }
    }
    println!("playin");
    // Play it!
    play(
        format,
        &mut audio_output,
        track,
        seek_time,
        &decode_opts,
        live_sample,
        false,
    )
}

fn play(
    mut reader: Box<dyn FormatReader>,
    audio_output: &mut Option<Box<dyn output::AudioOutput>>,
    track_num: Option<usize>,
    seek_time: Option<f64>,
    decode_opts: &DecoderOptions,
    live_sample: Arc<Mutex<(u64, (u64, u64, f64))>>,
    no_progress: bool,
) -> Result<i32> {
    // If the user provided a track number, select that track if it exists, otherwise, select the
    // first track with a known codec.
    let track = track_num
        .and_then(|t| reader.tracks().get(t))
        .or_else(|| first_supported_track(reader.tracks()))
        .expect("no supported audio tracks");

    let seek_ts = seek_time.unwrap_or(0.0);

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decode_opts)
        .expect("unsupported codec");

    // Store the track identifier, it will be used to filter packets.
    let track_id = track.id;

    let play_opts = PlayTrackOptions {
        track_id,
        seek_ts: seek_ts as u64,
    };

    // Get the selected track's timebase and duration.
    let tb = track.codec_params.time_base;
    let dur = track
        .codec_params
        .n_frames
        .map(|frames| track.codec_params.start_ts + frames);

    let result = loop {
        // Get the next packet from the media format.
        let packet = match reader.next_packet() {
            Ok(packet) => packet,
            Err(err @ Error::ResetRequired) => {
                // The track list has been changed. Re-examine it and create a new set of decoders,
                // then restart the decode loop. This is an advanced feature and it is not
                // unreasonable to consider this "the end." As of v0.5.0, the only usage of this is
                // for chained OGG physical streams.
                // unimplemented!();
                break Err(err);
            }
            Err(err) => break Err(err),
        };

        // If the packet does not belong to the selected track, skip over it.
        if packet.track_id() != track_id {
            continue;
        }

        // Consume any new metadata that has been read since the last packet.
        while !reader.metadata().is_latest() {
            // Pop the old head of the metadata queue.
            reader.metadata().pop();
            // Consume the new metadata at the head of the metadata queue.
        }

        // Decode the packet into audio samples.
        match decoder.decode(&packet) {
            Ok(decoded) => {
                // Consume the decoded audio samples (see below).
                // If the audio output is not open, try to open it.
                if audio_output.is_none() {
                    // Get the audio buffer specification. This is a description of the decoded
                    // audio buffer's sample format and sample rate.
                    let mut spec = *decoded.spec();
                    spec.rate = 48000;
                    //println!("spec are {:?}", spec);

                    // Get the capacity of the decoded buffer. Note that this is capacity, not
                    // length! The capacity of the decoded buffer is constant for the life of the
                    // decoder, but the length is not.
                    let duration = decoded.capacity() as u64;

                    // Try to open the audio output.
                    audio_output.replace(output::try_open(spec, duration).unwrap());
                } else {
                    // TODO: Check the audio spec. and duration hasn't changed.
                }

                // Write the decoded audio samples to the audio output if the presentation timestamp
                // for the packet is >= the seeked position (0 if not seeking).
                if packet.ts() >= play_opts.seek_ts {
                    if !no_progress {
                        print_progress(packet.ts(), dur, tb);
                    }
                    if let Some(tb) = tb {
                        let formatted_time = get_timestamp_formatted(packet.ts(), tb);

                        if let Some(audio_output) = audio_output {
                            {
                                let mut live_sample_mut = live_sample
                                    .try_lock()
                                    .expect("couldn't acquire write lock playin sync");
                                *live_sample_mut = (packet.ts(), formatted_time);
                            }
                            audio_output.write(decoded).unwrap()
                        }
                    }
                }
            }
            Err(Error::DecodeError(err)) => {
                // Decode errors are not fatal. Print the error message and try to decode the next
                // packet as usual.
                warn!("decode error: {}", err);
            }
            Err(err) => break Err(err),
        }
    };

    // Return if a fatal error occured.
    ignore_end_of_stream_error(result)?;
    // Finalize the decoder and return the verification result if it's been enabled.
    do_verification(decoder.finalize())
}

fn first_supported_track(tracks: &[Track]) -> Option<&Track> {
    tracks
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
}

fn ignore_end_of_stream_error(result: Result<()>) -> Result<()> {
    match result {
        Err(Error::IoError(err))
            if err.kind() == std::io::ErrorKind::UnexpectedEof
                && err.to_string() == "end of stream" =>
        {
            // Do not treat "end of stream" as a fatal error. It's the currently only way a
            // format reader can indicate the media is complete.
            Ok(())
        }
        _ => result,
    }
}

fn do_verification(finalization: FinalizeResult) -> Result<i32> {
    match finalization.verify_ok {
        Some(is_ok) => {
            // Got a verification result.
            println!("verification: {}", if is_ok { "passed" } else { "failed" });

            Ok(i32::from(!is_ok))
        }
        // Verification not enabled by user, or unsupported by the codec.
        _ => Ok(0),
    }
}

fn get_timestamp_formatted(ts: u64, tb: TimeBase) -> (u64, u64, f64) {
    let t = tb.calc_time(ts);

    let hours = t.seconds / (60 * 60);
    let mins = (t.seconds % (60 * 60)) / 60;
    let secs = f64::from((t.seconds % 60) as u32) + t.frac;
    (hours, mins, secs)
}

fn print_progress(ts: u64, dur: Option<u64>, tb: Option<TimeBase>) {
    // Get a string slice containing a progress bar.
    fn progress_bar(ts: u64, dur: u64) -> &'static str {
        const NUM_STEPS: usize = 60;

        lazy_static! {
            static ref PROGRESS_BAR: Vec<String> = {
                (0..NUM_STEPS + 1)
                    .map(|i| format!("[{:<60}]", str::repeat("■", i)))
                    .collect()
            };
        }

        let i = (NUM_STEPS as u64)
            .saturating_mul(ts)
            .checked_div(dur)
            .unwrap_or(0)
            .clamp(0, NUM_STEPS as u64);

        &PROGRESS_BAR[i as usize]
    }

    // Multiple print! calls would need to be made to print the progress, so instead, only lock
    // stdout once and use write! rather then print!.
    let stdout = std::io::stdout();
    let mut output = stdout.lock();

    if let Some(tb) = tb {
        let t = tb.calc_time(ts);

        let hours = t.seconds / (60 * 60);
        let mins = (t.seconds % (60 * 60)) / 60;
        let secs = f64::from((t.seconds % 60) as u32) + t.frac;

        write!(
            output,
            "\r\u{25b6}\u{fe0f}  {}:{:0>2}:{:0>4.1}",
            hours, mins, secs
        )
        .unwrap();

        if let Some(dur) = dur {
            let d = tb.calc_time(dur.saturating_sub(ts));

            let hours = d.seconds / (60 * 60);
            let mins = (d.seconds % (60 * 60)) / 60;
            let secs = f64::from((d.seconds % 60) as u32) + d.frac;

            write!(
                output,
                " {} -{}:{:0>2}:{:0>4.1}",
                progress_bar(ts, dur),
                hours,
                mins,
                secs
            )
            .unwrap();
        }
    } else {
        write!(output, "\r\u{25b6}\u{fe0f}  {}", ts).unwrap();
    }

    // This extra space is a workaround for Konsole to correctly erase the previous line.
    write!(output, " ").unwrap();

    // Flush immediately since stdout is buffered.
    output.flush().unwrap();
}
