#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioTag {
    // Tags
    Artist,
    Album,
    Bpm,
    Date,
    Genre,
    Lyrics,
    TrackNumber,
    TrackName,
    // Metadatas
    Duration,
    FrameRate,
    ChannelsNumber,
    /// Total frames count
    TotalFrames,
    Unknown,
}

impl From<&str> for AudioTag {
    fn from(value: &str) -> AudioTag {
        from_str_to_audio_tag(value)
    }
}

impl From<&String> for AudioTag {
    fn from(value: &String) -> Self {
        from_str_to_audio_tag(value.as_str())
    }
}

fn from_str_to_audio_tag(value: &str) -> AudioTag {
    match value {
        "artist" => AudioTag::Artist,
        "album" => AudioTag::Album,
        "bpm" => AudioTag::Bpm,
        "date" => AudioTag::Date,
        "genre" => AudioTag::Genre,
        "lyrics" => AudioTag::Lyrics,
        "trackNumber" => AudioTag::TrackNumber,
        "trackName" => AudioTag::TrackName,
        // Metadatas
        "duration" => AudioTag::Duration,
        "frameRate" => AudioTag::FrameRate,
        "channelsNumber" => AudioTag::ChannelsNumber,
        /// Total frames count
        "totalFrames" => AudioTag::TotalFrames,
        _ => AudioTag::Unknown,
    }
}
