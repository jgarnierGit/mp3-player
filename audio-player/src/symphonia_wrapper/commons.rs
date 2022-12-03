use std::{fs::File, path::Path};
use symphonia::core::errors::Result;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::{Hint, ProbeResult};

pub fn get_probe(music_path: &Path) -> Result<ProbeResult> {
    // Create a hint to help the format registry guess what format reader is appropriate.
    let mut hint = Hint::new();
    if let Some(ext) = music_path.extension() {
        if let Some(ext_str) = ext.to_str() {
            hint.with_extension(ext_str);
            println!("hint is {:?}", hint);
        } else {
            // TODO add to a errorlog
            panic!("invalid format {:?}", ext);
        }
    } else {
        // TODO add to a errorlog
        panic!("no file extension for {:?}", music_path);
    }
    let source = Box::new(File::open(music_path)?);

    // Create the media source stream using the boxed media source from above.
    let mss = MediaSourceStream::new(source, Default::default());

    // Use the default options for metadata and format readers.
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    // Probe the media source stream for metadata and get the format reader.
    symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)
}
