use std::error::Error;
use std::{collections::HashMap, fs::DirEntry, path::Path, rc::Rc};

use audio_player::MetadataParserWrapper;

use crate::audio_library::visitor;

pub fn aggregate_by(
    path: &Path,
    metadata_parser: &Box<dyn MetadataParserWrapper>,
    tag: &String,
) -> (Rc<HashMap<String, Rc<usize>>>, Rc<Vec<Box<dyn Error>>>) {
    let mut sample_aggr: Rc<HashMap<String, Rc<usize>>> = Rc::new(HashMap::new());
    let mut errors: Rc<Vec<Box<dyn Error>>> = Rc::new(Vec::new());

    let mut closure_sample_aggr = {
        let mut_map = Rc::get_mut(&mut sample_aggr).unwrap();
        let mut_errors = Rc::get_mut(&mut errors).unwrap();

        move |_dir: &DirEntry, audio_path: &Path| {
            match metadata_parser.get_metadata_string(audio_path, tag) {
                Ok(metadata) => {
                    if let Some(counting) = mut_map.get_mut(&metadata) {
                        let mut_counting = Rc::get_mut(counting).unwrap();
                        *mut_counting += 1;
                    } else {
                        mut_map.insert(metadata, Rc::new(1));
                    }
                }
                Err(error) => {
                    // TODO aggregate audio_path in order to identify file in error.
                    // TODO try to fix lib if possible
                    mut_errors.push(error);
                }
            };
        }
    };
    visitor::visit_mut(path, &mut closure_sample_aggr).unwrap();
    (sample_aggr, errors)
}

/// By default, filters and get music path.
pub fn filter_by(
    path: &Path,
    metadata_parser: &Box<dyn MetadataParserWrapper>,
    tag: &String,
    value: &str,
) -> (
    Rc<HashMap<String, Rc<Vec<String>>>>,
    Rc<Vec<Box<dyn Error>>>,
) {
    let mut filtered: Rc<HashMap<String, Rc<Vec<String>>>> = Rc::new(HashMap::new());
    let mut errors: Rc<Vec<Box<dyn Error>>> = Rc::new(Vec::new());

    let mut closure_sample_filter = {
        let mut_map = Rc::get_mut(&mut filtered).unwrap();
        let mut_errors = Rc::get_mut(&mut errors).unwrap();
        move |_dir: &DirEntry, audio_path: &Path| {
            match metadata_parser.get_metadata_string(audio_path, tag) {
                Ok(metadata) if metadata.as_str() == value => {
                    if let Some(filtered_sound) = mut_map.get_mut(&metadata) {
                        let mut_filtered_sound = Rc::get_mut(filtered_sound).unwrap();
                        mut_filtered_sound.push(String::from(audio_path.to_str().unwrap()));
                    } else {
                        mut_map.insert(
                            metadata,
                            Rc::new(vec![String::from(audio_path.to_str().unwrap())]),
                        );
                    }
                }
                Ok(_) => (),
                Err(error) => {
                    // TODO aggregate audio_path in order to identify file in error.
                    // TODO try to fix lib if possible
                    mut_errors.push(error);
                }
            };
        }
    };
    visitor::visit_mut(path, &mut closure_sample_filter).unwrap();
    (filtered, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use audio_player::MetadataParserBuilder;
    use std::error::Error;
    use std::fs::{self, File};
    use std::io::Read;
    use std::io::Seek;
    use std::io::SeekFrom;
    use std::io::Write;
    use std::path::Path;
    use std::path::PathBuf;
    use std::sync::mpsc::Sender;
    use std::thread::{self, JoinHandle};
    use tempfile::Builder;
    use tempfile::NamedTempFile;
    use tempfile::TempPath;

    struct MetadataParserMock {}

    impl MetadataParserWrapper for MetadataParserMock {
        fn get_metadata_string(
            &self,
            audio_path: &Path,
            _target_metadata: &String,
        ) -> Result<String, Box<dyn Error>> {
            let mut buffer: String = String::new();
            println!("reading test file {:?}", audio_path);
            let mut audio = File::open(audio_path).unwrap();
            audio.read_to_string(&mut buffer).unwrap();
            Ok(buffer)
        }

        fn print_metadata(&self, _audio_path: &Path) {}
        fn print_tags(&self, _audio_path: &Path) {}
        fn print_visuals(&self, _audio_path: &Path) {}
        fn get_file_samples(&self, _audio_path: &Path) -> Option<Box<Vec<f32>>> {
            None
        }
        fn get_live_sample(
            &self,
            _audio_path: &Path,
            _live_sample: Sender<(usize, usize, Vec<f32>)>,
            _live_sample_written: &mut Rc<usize>,
        ) -> JoinHandle<()> {
            thread::spawn(|| {})
        }
    }

    fn build_metadata_parser_mock() -> Box<dyn MetadataParserWrapper> {
        Box::new(MetadataParserMock {})
    }

    /// TODO extract all this logic into a common tester package.
    ///
    /// create_sub_dir persists folder on disk, so don't forget to :
    /// ```
    /// drop(dir)
    /// ```
    /// before leaving
    ///
    /// # Returns
    /// (temporary file created, current directory)
    fn create_temp_file(dir: &Path, create_sub_dir: bool, content: &str) -> (TempPath, PathBuf) {
        let mut temp_file: NamedTempFile;
        let current_dir: PathBuf;
        if create_sub_dir {
            let sub_dir = Builder::new().tempdir_in(dir).unwrap();
            let persis_sub_dir = sub_dir.into_path();
            temp_file = Builder::new()
                .suffix(".mp3")
                .tempfile_in(persis_sub_dir.clone())
                .unwrap();
            current_dir = persis_sub_dir;
        } else {
            temp_file = Builder::new()
                .suffix(".mp3")
                .tempfile_in(dir.clone())
                .unwrap();
            current_dir = dir.to_path_buf();
        }

        temp_file.write_all(content.as_bytes()).unwrap();
        temp_file.seek(SeekFrom::Start(0)).unwrap();
        (temp_file.into_temp_path(), current_dir)
    }

    fn drop_temp_dir(dir: PathBuf) {
        let dir_log = dir.clone();
        let dir_path = dir_log.as_os_str();
        println!("cleaning temp directory : {:?}", dir_path);
        fs::remove_dir(dir)
            .expect(format!("Could not remove temp directory {:?}", dir_path).as_str());
    }

    #[test]
    #[ignore]
    fn it_with_temp_files() {
        let root_content = "Metal";
        let sub_content_1 = "Rock";
        let root_dir = Builder::new().tempdir_in("./").unwrap();
        let root_path = root_dir.into_path();
        let (root_audio, root_dir) = create_temp_file(&root_path, false, root_content);
        let (sub_audio, sub_dir2) = create_temp_file(&root_path, true, sub_content_1);
        let mut root_buffer: String = String::new();
        let mut sub_buffer: String = String::new();

        let mut root_audio = File::open(root_audio).unwrap();
        let mut sub_audio = File::open(sub_audio).unwrap();
        root_audio.read_to_string(&mut root_buffer).unwrap();
        sub_audio.read_to_string(&mut sub_buffer).unwrap();
        assert_eq!(root_buffer, root_content);
        assert_eq!(sub_buffer, sub_content_1);
        //TODO add "finally" behavior for drop_temp_dir if assert fails
        drop_temp_dir(sub_dir2);
        drop_temp_dir(root_dir);
    }

    #[test]
    fn it_aggregate_with_mock() {
        let metal_content = "Metal";
        let rock_content = "Rock";
        let empty_content = "Ska";
        let tag = String::from("who cares");
        let root_dir = Builder::new().tempdir_in("./").unwrap();
        let root_path = root_dir.into_path();
        let (root_audio, root_dir) = create_temp_file(&root_path, false, metal_content);
        let (root_audio2, _) = create_temp_file(&root_path, false, metal_content);
        let (sub_audio, sub_dir) = create_temp_file(&root_path, true, rock_content);
        //  let clone_sub_dir = sub_dir.unwrap();
        // let sub_sub_dir = clone_sub_dir.clone().as_path();
        let (sub_audio2, sub_dir2) = create_temp_file(&root_path, true, empty_content);
        let metadata_parser = build_metadata_parser_mock();
        let (result_aggr_genre, _) = aggregate_by(&root_path, &metadata_parser, &tag);
        assert_eq!(result_aggr_genre.len(), 3);

        // FIXME for some reason I need to force drop audio file before removing dir in this test case. but not in it_with_temp_files
        // investigate this behavior. (not related to aggregate_by_sample_rate)
        drop(root_audio);
        drop(root_audio2);
        drop(sub_audio);
        drop(sub_audio2);
        drop_temp_dir(sub_dir);
        drop_temp_dir(sub_dir2);
        drop_temp_dir(root_dir);
    }

    #[test]
    fn it_filters_with_mock() {
        let metal_content = "Metal";
        let rock_content = "Rock";
        let empty_content = "Ska";
        let tag = String::from("who cares");
        let root_dir = Builder::new().tempdir_in("./").unwrap();
        let root_path = root_dir.into_path();
        let (root_audio, root_dir) = create_temp_file(&root_path, false, metal_content);
        let (root_audio2, _) = create_temp_file(&root_path, false, metal_content);
        let (sub_audio, sub_dir) = create_temp_file(&root_path, true, rock_content);
        //  let clone_sub_dir = sub_dir.unwrap();
        // let sub_sub_dir = clone_sub_dir.clone().as_path();
        let (sub_audio2, sub_dir2) = create_temp_file(&root_path, true, empty_content);
        let metadata_parser = build_metadata_parser_mock();
        let (result_aggr_genre, _) = filter_by(&root_path, &metadata_parser, &tag, metal_content);
        assert_eq!(result_aggr_genre.len(), 1);
        assert_eq!(result_aggr_genre.get(metal_content).unwrap().len(), 2);

        // FIXME for some reason I need to force drop audio file before removing dir in this test case. but not in it_with_temp_files
        // investigate this behavior. (not related to aggregate_by_sample_rate)
        drop(root_audio);
        drop(root_audio2);
        drop(sub_audio);
        drop(sub_audio2);
        drop_temp_dir(sub_dir);
        drop_temp_dir(sub_dir2);
        drop_temp_dir(root_dir);
    }

    #[test]
    #[ignore]
    fn it_aggregate_genre() {
        let tag = String::from("who cares");
        //  let mut tmpfile: File = tempfile::tempfile().unwrap();
        let path = Path::new("D:/Documents/prog/rust/mp3Player/audio-project/audio-manager/assets");
        let metadata_parser = MetadataParserBuilder::build();
        let (result_aggr_genre, _) = aggregate_by(path, &metadata_parser, &tag);
        assert_eq!(result_aggr_genre.len(), 3)
    }
}
