use symphonia::core::{
    codecs::CodecParameters,
    formats::{Cue, Track},
    meta::{ColorMode, StandardTagKey, Tag, Value, Visual},
    probe::ProbeResult,
    units::TimeBase,
};

use crate::{audio_parser::TagsResult, audio_tags::AudioTag};

use super::commons;

use log::info;
use std::error::Error;
use std::path::Path;

pub fn parse(music_path: &Path) {
    let mut probed = match commons::get_probe(music_path) {
        Ok(probe) => probe,
        Err(err) => panic!("Unsupported format {}", err),
    };
    print_format(&mut probed);
}

/// # Returns tags content list ordered as input
pub fn get_metadata_string(audio_path: &Path, target: &Vec<AudioTag>) -> TagsResult {
    let mut probed = commons::get_probe(audio_path)?;
    // Prefer metadata that's provided in the container format, over other tags found during the
    // probe operation.

    let mut content_list: Vec<Option<String>> = Vec::new();

    for tag in target {
        let tag_content: Option<String>;
        // try find target metadata in format :
        if let Some(format_item) = get_tracks_string(probed.format.tracks(), tag) {
            content_list.push(Some(format_item));
            continue;
        }

        // try finding target metadata in tags :
        if let Some(metadata_rev) = probed.format.metadata().current() {
            tag_content = get_tag_string(metadata_rev.tags(), tag);
        } else if let Some(metadata_rev) = probed.metadata.get().as_ref().and_then(|m| m.current())
        {
            tag_content = get_tag_string(metadata_rev.tags(), tag);
        } else {
            tag_content = None;
        }
        content_list.push(tag_content);
    }

    Ok(content_list)
}

fn get_tag_string(tags: &[Tag], target: &AudioTag) -> Option<String> {
    if !tags.is_empty() {
        // Print tags with a standard tag key first, these are the most common tags.
        for tag in tags.iter().filter(|tag| tag.is_known()) {
            if let Some(matchin_result) = get_matching_tag(tag, target) {
                return Some(matchin_result.to_string());
            }
        }

        // Print the remaining tags with keys truncated to 26 characters.
        for tag in tags.iter().filter(|tag| !tag.is_known()) {
            if let Some(matchin_result) = get_matching_tag(tag, target) {
                return Some(matchin_result.to_string());
            }
        }
    }
    None
}

fn get_matching_tag<'a>(tag: &'a Tag, target: &AudioTag) -> Option<&'a Value> {
    if let Some(std_key) = tag.std_key {
        if let Some(target_key) = get_stantard_tag_key(target) {
            if std_key == target_key {
                return Some(&tag.value);
            }
        }
    }
    None
}

fn get_stantard_tag_key(target: &AudioTag) -> Option<StandardTagKey> {
    match target {
        AudioTag::Artist => Some(StandardTagKey::Artist),
        AudioTag::Album => Some(StandardTagKey::Album),
        AudioTag::Bpm => Some(StandardTagKey::Bpm),
        AudioTag::Date => Some(StandardTagKey::Date),
        AudioTag::Genre => Some(StandardTagKey::Genre),
        AudioTag::Lyrics => Some(StandardTagKey::Lyrics),
        AudioTag::TrackNumber => Some(StandardTagKey::TrackNumber),
        AudioTag::TrackName => Some(StandardTagKey::TrackTitle),
        _ => None,
    }
}

fn print_format(probed: &mut ProbeResult) {
    print_tracks(probed.format.tracks());

    // Prefer metadata that's provided in the container format, over other tags found during the
    // probe operation.
    if let Some(metadata_rev) = probed.format.metadata().current() {
        print_tags(metadata_rev.tags());
        print_visuals(metadata_rev.visuals());

        // Warn that certain tags are preferred.
        if probed.metadata.get().as_ref().is_some() {
            info!("tags that are part of the container format are preferentially printed.");
            info!("not printing additional tags that were found while probing.");
        }
    } else if let Some(metadata_rev) = probed.metadata.get().as_ref().and_then(|m| m.current()) {
        print_tags(metadata_rev.tags());
        print_visuals(metadata_rev.visuals());
    }

    print_cues(probed.format.cues());
    println!(":");
    println!();
}

fn get_tracks_string(tracks: &[Track], target: &AudioTag) -> Option<String> {
    if tracks.is_empty() {
        return None;
    }
    /*for (idx, track) in tracks.iter().enumerate() {
        let params = &track.codec_params;

    }*/
    let track_content = if let Some(track) = tracks.first() {
        let params = &track.codec_params;
        match target {
            AudioTag::Duration => get_duration(params),
            AudioTag::FrameRate => get_sample_rate(params),
            AudioTag::ChannelsNumber => get_channels(params),
            AudioTag::TotalFrames => get_frame_number(params),
            _ => None,
        }
    } else {
        None
    };
    track_content
}

/// Get total audio file frame count
fn get_frame_number(params: &CodecParameters) -> Option<String> {
    match params.n_frames {
        Some(rate) => Some(rate.to_string()),
        None => None,
    }
}

/// TODO returns numeric
fn get_sample_rate(params: &CodecParameters) -> Option<String> {
    match params.sample_rate {
        Some(rate) => Some(rate.to_string()),
        None => None,
    }
}
/// TODO returns numeric
fn get_channels(params: &CodecParameters) -> Option<String> {
    match params.channels {
        Some(info) => Some(info.count().to_string()),
        None => None,
    }
}

fn get_duration(params: &CodecParameters) -> Option<String> {
    if let Some(n_frames) = params.n_frames {
        if let Some(tb) = params.time_base {
            return Some(fmt_time(n_frames, tb));
        } else {
            return Some(n_frames.to_string());
        }
    }
    None
}

fn print_tracks(tracks: &[Track]) {
    if !tracks.is_empty() {
        println!("|");
        println!("| // Tracks //");

        for (idx, track) in tracks.iter().enumerate() {
            let params = &track.codec_params;

            print!("|     [{:0>2}] Codec:           ", idx + 1);

            if let Some(codec) = symphonia::default::get_codecs().get_codec(params.codec) {
                println!("{} ({})", codec.long_name, codec.short_name);
            } else {
                println!("Unknown (#{})", params.codec);
            }

            if let Some(sample_rate) = params.sample_rate {
                println!("|          Sample Rate:     {}", sample_rate);
            }
            if params.start_ts > 0 {
                if let Some(tb) = params.time_base {
                    println!(
                        "|          Start Time:      {} ({})",
                        fmt_time(params.start_ts, tb),
                        params.start_ts
                    );
                } else {
                    println!("|          Start Time:      {}", params.start_ts);
                }
            }
            if let Some(n_frames) = params.n_frames {
                if let Some(tb) = params.time_base {
                    println!(
                        "|          Duration:        {} ({})",
                        fmt_time(n_frames, tb),
                        n_frames
                    );
                } else {
                    println!("|          Frames:          {}", n_frames);
                }
            }
            if let Some(tb) = params.time_base {
                println!("|          Time Base:       {}", tb);
            }
            if let Some(padding) = params.delay {
                println!("|          Encoder Delay:   {}", padding);
            }
            if let Some(padding) = params.padding {
                println!("|          Encoder Padding: {}", padding);
            }
            if let Some(sample_format) = params.sample_format {
                println!("|          Sample Format:   {:?}", sample_format);
            }
            if let Some(bits_per_sample) = params.bits_per_sample {
                println!("|          Bits per Sample: {}", bits_per_sample);
            }
            if let Some(channels) = params.channels {
                println!("|          Channel(s):      {}", channels.count());
                println!("|          Channel Map:     {}", channels);
            }
            if let Some(channel_layout) = params.channel_layout {
                println!("|          Channel Layout:  {:?}", channel_layout);
            }
            if let Some(language) = &track.language {
                println!("|          Language:        {}", language);
            }
        }
    }
}

fn print_cues(cues: &[Cue]) {
    if !cues.is_empty() {
        println!("|");
        println!("| // Cues //");

        for (idx, cue) in cues.iter().enumerate() {
            println!("|     [{:0>2}] Track:      {}", idx + 1, cue.index);
            println!("|          Timestamp:  {}", cue.start_ts);

            // Print tags associated with the Cue.
            if !cue.tags.is_empty() {
                println!("|          Tags:");

                for (tidx, tag) in cue.tags.iter().enumerate() {
                    if let Some(std_key) = tag.std_key {
                        println!(
                            "{}",
                            print_tag_item(tidx + 1, &format!("{:?}", std_key), &tag.value, 21)
                        );
                    } else {
                        println!("{}", print_tag_item(tidx + 1, &tag.key, &tag.value, 21));
                    }
                }
            }

            // Print any sub-cues.
            if !cue.points.is_empty() {
                println!("|          Sub-Cues:");

                for (ptidx, pt) in cue.points.iter().enumerate() {
                    println!(
                        "|                      [{:0>2}] Offset:    {:?}",
                        ptidx + 1,
                        pt.start_offset_ts
                    );

                    // Start the number of sub-cue tags, but don't print them.
                    if !pt.tags.is_empty() {
                        println!(
                            "|                           Sub-Tags:  {} (not listed)",
                            pt.tags.len()
                        );
                    }
                }
            }
        }
    }
}

fn print_tags(tags: &[Tag]) {
    if !tags.is_empty() {
        println!("|");
        println!("| // Tags //");

        let mut idx = 1;

        // Print tags with a standard tag key first, these are the most common tags.
        for tag in tags.iter().filter(|tag| tag.is_known()) {
            if let Some(std_key) = tag.std_key {
                println!(
                    "{}",
                    print_tag_item(idx, &format!("{:?}", std_key), &tag.value, 4)
                );
            }
            idx += 1;
        }

        // Print the remaining tags with keys truncated to 26 characters.
        for tag in tags.iter().filter(|tag| !tag.is_known()) {
            println!("{}", print_tag_item(idx, &tag.key, &tag.value, 4));
            idx += 1;
        }
    }
}

fn print_visuals(visuals: &[Visual]) {
    if !visuals.is_empty() {
        println!("|");
        println!("| // Visuals //");

        for (idx, visual) in visuals.iter().enumerate() {
            if let Some(usage) = visual.usage {
                println!("|     [{:0>2}] Usage:      {:?}", idx + 1, usage);
                println!("|          Media Type: {}", visual.media_type);
            } else {
                println!("|     [{:0>2}] Media Type: {}", idx + 1, visual.media_type);
            }
            if let Some(dimensions) = visual.dimensions {
                println!(
                    "|          Dimensions: {} px x {} px",
                    dimensions.width, dimensions.height
                );
            }
            if let Some(bpp) = visual.bits_per_pixel {
                println!("|          Bits/Pixel: {}", bpp);
            }
            if let Some(ColorMode::Indexed(colors)) = visual.color_mode {
                println!("|          Palette:    {} colors", colors);
            }
            println!("|          Size:       {} bytes", visual.data.len());

            // Print out tags similar to how regular tags are printed.
            if !visual.tags.is_empty() {
                println!("|          Tags:");
            }

            for (tidx, tag) in visual.tags.iter().enumerate() {
                if let Some(std_key) = tag.std_key {
                    println!(
                        "{}",
                        print_tag_item(tidx + 1, &format!("{:?}", std_key), &tag.value, 21)
                    );
                } else {
                    println!("{}", print_tag_item(tidx + 1, &tag.key, &tag.value, 21));
                }
            }
        }
    }
}

fn print_tag_item(idx: usize, key: &str, value: &Value, indent: usize) -> String {
    let key_str = match key.len() {
        0..=28 => format!("| {:w$}[{:0>2}] {:<28} : ", "", idx, key, w = indent),
        _ => format!(
            "| {:w$}[{:0>2}] {:.<28} : ",
            "",
            idx,
            key.split_at(26).0,
            w = indent
        ),
    };

    let line_prefix = format!("\n| {:w$} : ", "", w = indent + 4 + 28 + 1);
    let line_wrap_prefix = format!("\n| {:w$}   ", "", w = indent + 4 + 28 + 1);

    let mut out = String::new();

    out.push_str(&key_str);

    for (wrapped, line) in value.to_string().lines().enumerate() {
        if wrapped > 0 {
            out.push_str(&line_prefix);
        }

        let mut chars = line.chars();
        let split = (0..)
            .map(|_| chars.by_ref().take(72).collect::<String>())
            .take_while(|s| !s.is_empty())
            .collect::<Vec<_>>();

        out.push_str(&split.join(&line_wrap_prefix));
    }

    out
}

fn fmt_time(ts: u64, tb: TimeBase) -> String {
    let time = tb.calc_time(ts);

    let hours = time.seconds / (60 * 60);
    let mins = (time.seconds % (60 * 60)) / 60;
    let secs = f64::from((time.seconds % 60) as u32) + time.frac;

    format!("{}:{:0>2}:{:0>6.3}", hours, mins, secs)
}
