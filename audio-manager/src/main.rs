use audio_manager::audio_library::{metadata_aggregator, visitor};
use audio_player::MetadataParserBuilder;
use std::fs::DirEntry;
use std::path::Path;
use std::rc::Rc;

fn main() {
    let path = Path::new("D:/Documents/prog/rust/mp3Player/audio-project/audio-manager/assets");
    // let path = Path::new("D:/mp3");
    let metadata_parser = MetadataParserBuilder::build();
    visitor::visit(path, &|_dir: &DirEntry, audio_path: &Path| {
        metadata_parser.print_metadata(audio_path)
    })
    .unwrap();
    count_music(path);
    let tag = String::from("genre");
    let frame_rate_tag = String::from("frameRate");
    let channel_tag = String::from("channels");
    let (res_metadata_aggr, errs) =
        metadata_aggregator::aggregate_by(path, &metadata_parser, &channel_tag);
    println!("metadatas aggregated {:?}", res_metadata_aggr);
    println!("errors {:?}", errs);
    //get songs with 1 channels only
    let value_to_filter = "1";
    let (res_metadata_filter, errs) =
        metadata_aggregator::filter_by(path, &metadata_parser, &channel_tag, value_to_filter);
    // TODO add an iterator layer logic for haevy results.
    println!("metadatas filtered {:?}", res_metadata_filter);
    println!("errors {:?}", errs);
}

pub fn count_music(path: &Path) {
    let mut counter = Rc::new(0);
    let mut closure_counter = {
        let test_mut = Rc::get_mut(&mut counter).unwrap();
        move |_dir: &DirEntry, _audio_path: &Path| {
            *test_mut += 1;
        }
    };
    visitor::visit_mut(path, &mut closure_counter).unwrap();
    println!("total {:?}", counter.as_ref());
}
