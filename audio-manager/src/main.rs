use audio_manager::fs_manager::audio_visitor;
use audio_player::MetadataParser;
use audio_player::MetadataParserBuilder;
use std::collections::HashMap;
use std::fs::DirEntry;
use std::path::Path;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
    let path = Path::new("D:/Documents/prog/rust/mp3Player/audio-project/audio-manager/assets");
    let metadata_parser = MetadataParserBuilder::build();
    audio_visitor::visit(path, &|_dir: &DirEntry, _audio_path: &Path| say_hello()).unwrap();
    audio_visitor::visit(path, &|_dir: &DirEntry, audio_path: &Path| {
        metadata_parser.print_metadata(audio_path)
    })
    .unwrap();
    count_music(path);

    //  aggregate_by_sample_rate(path);
}

pub fn say_hello() {
    println!("hello");
}

pub fn count_music(path: &Path) {
    let mut counter = Rc::new(0);
    let mut closure_counter = {
        let test_mut = Rc::get_mut(&mut counter).unwrap();
        move |_dir: &DirEntry, _audio_path: &Path| {
            *test_mut += 1;
        }
    };
    audio_visitor::visit_mut(path, &mut closure_counter).unwrap();
    //visit_dirs(path, &mut closure_counter).unwrap();
    println!("total {:?}", counter.as_ref());
}

pub fn aggregate_by_sample_rate(path: &Path) {
    let mut sample_aggr: Rc<HashMap<String, usize>> = Rc::new(HashMap::new());
    let mut closure_sample_aggr = {
        let mut_map = Rc::get_mut(&mut sample_aggr).unwrap();
        move |dir: &DirEntry, _audio_path: &Path| {}
    };
    audio_visitor::visit_mut(path, &mut closure_sample_aggr).unwrap();
}
