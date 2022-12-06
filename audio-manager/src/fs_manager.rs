pub mod audio_visitor {
    use std::{
        fs::{self, DirEntry},
        io,
        path::Path,
    };

    /// Symphonia can handle more types but I'm just lazy to write them in this enum
    enum AudioFormat {
        MP3,
        FLAC,
    }

    fn match_audio_type(extension: &str) -> Option<AudioFormat> {
        match extension {
            "mp3" => Some(AudioFormat::MP3),
            "flac" => Some(AudioFormat::FLAC),
            _ => None,
        }
    }

    fn check_audio_format(path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                if let Some(_) = match_audio_type(ext_str) {
                    return true;
                }
            }
        }
        false
    }

    /// Visitor for valid audio files
    pub fn visit<T>(path: &Path, cb: &T) -> io::Result<()>
    where
        T: Fn(&DirEntry, &Path),
    {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit(&path, cb)?;
                } else if check_audio_format(&path) {
                    cb(&entry, &path);
                }
            }
        }
        Ok(())
    }

    /// Mutable state visitor for valid audio files
    pub fn visit_mut<T>(dir: &Path, cb: &mut T) -> io::Result<()>
    where
        T: FnMut(&DirEntry, &Path),
    {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_mut(&path, cb)?;
                } else if check_audio_format(&path) {
                    cb(&entry, &path);
                }
            }
        }
        Ok(())
    }
}
