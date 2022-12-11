use crate::symphonia_wrapper;
use std::path::Path;

pub trait MetadataParserWrapper {
    /// DEBUG : Print audio tags
    fn print_tags(&self, audio_path: &Path);
    /// DEBUG : Print audio file metadata
    fn print_metadata(&self, audio_path: &Path);
    /// Get a file target metadata
    fn get_metadata_string(&self, audio_path: &Path, target_metadata: String) -> String;
    /// DEBUG : Print audio file thumbnail
    fn print_visuals(&self, audio_path: &Path);
}

pub mod metadata_parser_builder {
    use crate::audio_parser::SymphoniaWrapper;
    use crate::MetadataParserWrapper;
    use std::path::Path;

    /// Build a MetadataParser from current crate used
    pub fn build() -> Box<dyn MetadataParserWrapper> {
        Box::new(MetadataParser {
            wrapped: Box::new(SymphoniaWrapper {}),
        })
    }
    pub struct MetadataParser {
        wrapped: Box<dyn MetadataParserWrapper>,
    }

    impl MetadataParserWrapper for MetadataParser {
        fn get_metadata_string(&self, audio_path: &Path, target_metadata: String) -> String {
            self.wrapped
                .get_metadata_string(audio_path, target_metadata)
        }
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
impl MetadataParserWrapper for SymphoniaWrapper {
    fn get_metadata_string(&self, audio_path: &Path, target_metadata: String) -> String {
        symphonia_wrapper::get_metadata_string(audio_path, target_metadata)
    }
    fn print_metadata(&self, audio_path: &Path) {
        symphonia_wrapper::parse(audio_path);
    }
    fn print_tags(&self, _audio_path: &Path) {
        //   symphonia_wrapper::print_tags(audio_path);
    }
    fn print_visuals(&self, _audio_path: &Path) {
        //   symphonia_wrapper::print_visuals(audio_path);
    }
}
