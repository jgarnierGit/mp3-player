use audio_manager::audio_library::{metadata_aggregator, visitor};
use audio_player::{MetadataParserBuilder, MetadataParserWrapper};
use std::fs::DirEntry;
use std::path::Path;
use std::rc::Rc;
mod args;
use args::Cli;
use clap::Parser;
use log::error;

fn main() {
    let args = Cli::parse();

    let code = match run(&args) {
        Ok(code) => code,
        Err(err) => {
            error!("{}", err.to_string().to_lowercase());
            -1
        }
    };
    std::process::exit(code)
}

fn run(args: &Cli) -> Result<i32, Box<dyn std::error::Error>> {
    let music_folder_path = Path::new(&args.input);
    let metadata_parser = MetadataParserBuilder::build();

    if args.count {
        count_music(music_folder_path);
    }
    if let Some(tag_agg) = args.aggregate_tag.as_deref() {
        process_aggregation(music_folder_path, &metadata_parser, tag_agg);
    }
    if let (Some(tag_filter), Some(tag_value)) =
        (args.filter_tag.as_deref(), args.filter_value.as_deref())
    {
        process_filter(music_folder_path, &metadata_parser, tag_filter, tag_value);
    }
    Ok(1)
}

fn process_aggregation(
    music_folder_path: &Path,
    metadata_parser: &Box<dyn MetadataParserWrapper>,
    tag_agg: &str,
) {
    let tag_agg_string = String::from(tag_agg); // FIXME change signature to &str
    let (res_metadata_aggr, errs) =
        metadata_aggregator::aggregate_by(music_folder_path, &metadata_parser, &tag_agg_string);
    println!("metadatas aggregated {:?}", res_metadata_aggr);
    println!("errors {:?}", errs);
}
fn process_filter(
    music_folder_path: &Path,
    metadata_parser: &Box<dyn MetadataParserWrapper>,
    tag_filter: &str,
    tag_value: &str,
) {
    let tag_agg_string = String::from(tag_filter); // FIXME change signature to &str

    let (res_metadata_filter, errs) = metadata_aggregator::filter_by(
        music_folder_path,
        &metadata_parser,
        &tag_agg_string,
        tag_value,
    );
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
