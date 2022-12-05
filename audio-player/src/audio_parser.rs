use crate::symphonia_wrapper;
use std::path::Path;

pub trait MetadataParser {
    /// Print audio tags
    fn print_tags(&self, audio_path: &Path);
    /// Print audio file metadata
    fn print_metadata(&self, audio_path: &Path);
    /// Print audio file thumbnail
    fn print_visuals(&self, audio_path: &Path);
}

pub mod metadata_parser_builder {
    use crate::audio_parser::SymphoniaWrapper;
    use crate::MetadataParser;
    use std::path::Path;

    /// Build a MetadataParser from current crate used
    pub fn build() -> MetadataParserWrapper {
        MetadataParserWrapper {
            wrapped: Box::new(SymphoniaWrapper {}),
        }
    }
    pub struct MetadataParserWrapper {
        wrapped: Box<dyn MetadataParser>,
    }

    impl MetadataParserWrapper {
        pub fn test() {
            println!("coucou");
        }
    }

    impl MetadataParser for MetadataParserWrapper {
        fn print_metadata(&self, audio_path: &Path) {
            self.wrapped.print_metadata(audio_path);
        }
        fn print_tags(&self, audio_path: &Path) {
            self.wrapped.print_tags(audio_path);
        }
        fn print_visuals(&self, audio_path: &Path) {
            self.wrapped.print_visuals(audio_path);
        }
    }
}

/// Symphonia lib wrapper
pub struct SymphoniaWrapper;
impl MetadataParser for SymphoniaWrapper {
    fn print_metadata(&self, audio_path: &Path) {
        symphonia_wrapper::parseMetadata(audio_path);
    }
    fn print_tags(&self, _audio_path: &Path) {
        //   symphonia_wrapper::print_tags(audio_path);
    }
    fn print_visuals(&self, _audio_path: &Path) {
        //   symphonia_wrapper::print_visuals(audio_path);
    }
}
