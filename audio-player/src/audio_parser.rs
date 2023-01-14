use crate::symphonia_wrapper;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

pub trait MetadataParserWrapper {
    /// DEBUG : Print audio tags
    fn print_tags(&self, audio_path: &Path);
    /// DEBUG : Print audio file metadata
    fn print_metadata(&self, audio_path: &Path);
    /// Get a file target metadata
    fn get_metadata_string(
        &self,
        audio_path: &Path,
        target_metadata: &String,
    ) -> Result<String, Box<dyn Error>>;
    /// DEBUG : Print audio file thumbnail
    fn print_visuals(&self, audio_path: &Path);
    /// TODO : try to extract this to a proper Wrapper
    fn get_file_samples(&self, audio_path: &Path) -> Option<Box<Vec<f32>>>;
    fn get_live_sample(
        &self,
        audio_path: &Path,
        live_sample: Sender<(usize, usize, Vec<f32>)>,
        live_sample_written: &mut Rc<usize>,
    ) -> JoinHandle<()>;
}

pub mod metadata_parser_builder {
    use crate::audio_parser::SymphoniaWrapper;
    use crate::MetadataParserWrapper;
    use std::error::Error;
    use std::path::Path;
    use std::rc::Rc;
    use std::sync::mpsc::Sender;
    use std::thread::JoinHandle;

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
        fn get_metadata_string(
            &self,
            audio_path: &Path,
            target_metadata: &String,
        ) -> Result<String, Box<dyn Error>> {
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
        fn get_file_samples(&self, audio_path: &Path) -> Option<Box<Vec<f32>>> {
            self.wrapped.get_file_samples(audio_path)
        }

        fn get_live_sample(
            &self,
            audio_path: &Path,
            live_sample: Sender<(usize, usize, Vec<f32>)>,
            live_sample_written: &mut Rc<usize>,
        ) -> JoinHandle<()> {
            self.wrapped
                .get_live_sample(audio_path, live_sample, live_sample_written)
        }
    }
}

/// Symphonia lib wrapper
pub struct SymphoniaWrapper;
impl MetadataParserWrapper for SymphoniaWrapper {
    fn get_metadata_string(
        &self,
        audio_path: &Path,
        target_metadata: &String,
    ) -> Result<String, Box<dyn Error>> {
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
    fn get_file_samples(&self, audio_path: &Path) -> Option<Box<Vec<f32>>> {
        symphonia_wrapper::get_file_samples(audio_path)
    }
    fn get_live_sample(
        &self,
        audio_path: &Path,
        live_sample: Sender<(usize, usize, Vec<f32>)>,
        live_sample_written: &mut Rc<usize>,
    ) -> JoinHandle<()> {
        symphonia_wrapper::get_live_sample(audio_path, live_sample, live_sample_written)
    }
}
