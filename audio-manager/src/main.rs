use audio_player::MetadataParser;
use audio_player::MetadataParserBuilder;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
static mut COUNTER: u32 = 0;

fn main() {
    println!("Hello, world!");
    let path = Path::new("D:/Documents/prog/rust/mp3Player/audio-project/audio-manager/assets");
    let metadata_parser = MetadataParserBuilder::build();
    //let path = Path::new("D:/mp3");
    /*
    visit_dirs(path, &|e| {
          println!("{:?} ", e.file_name()) //, e.metadata().unwrap()
      })
      .unwrap();
    */
    // TODO visit_dirs_path(path, print_metadatas);
    visit_dirs(path, &count).unwrap();
    unsafe {
        println!("total {}", COUNTER);
    }
}
fn print_metadatas() {}

fn count(entry: &DirEntry) {
    if let Ok(ftype) = entry.file_type() {
        println!("{:?}", ftype);
    }
    unsafe {
        COUNTER += 1;
    }
}
/*
fn visit_dirs_path<T>(dir: &Path, cb: &T) -> io::Result<()>
where
    T: Fn(&Path),
{
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}*/

fn visit_dirs<T>(dir: &Path, cb: &T) -> io::Result<()>
where
    T: Fn(&DirEntry),
{
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
