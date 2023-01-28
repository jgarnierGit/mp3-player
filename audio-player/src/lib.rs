mod audio_parser;
mod symphonia_wrapper;
pub use audio_parser::metadata_parser_builder as MetadataParserBuilder;
pub use audio_parser::MetadataParserWrapper;
pub use symphonia_wrapper::playTrack;
pub mod audio_tags;
pub use audio_parser::TagsResult;
pub use audio_tags::AudioTag;
