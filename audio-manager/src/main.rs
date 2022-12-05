use audio_player::MetadataParser;
use audio_player::MetadataParserBuilder;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
    let path = Path::new("D:/Documents/prog/rust/mp3Player/audio-project/audio-manager/assets");
    let metadata_parser = MetadataParserBuilder::build();
    visit_dirs_path(path, &move |path| metadata_parser.print_metadata(path)).unwrap();
    let mut counter = Rc::new(0);
    let mut closure_counter = {
        let test_mut = Rc::get_mut(&mut counter).unwrap();
        move |entry: &DirEntry| {
            if let Ok(ftype) = entry.file_type() {
                println!("{:?}", ftype);
            }
            *test_mut += 1;
        }
    };
    visit_dirs(path, &mut closure_counter).unwrap();
    println!("total {:?}", counter.as_ref());
    visit_dirs(path, &mut |_dir| println!("nothing special")).unwrap();
}

fn visit_dirs_path<T>(path: &Path, cb: &T) -> io::Result<()>
where
    T: Fn(&Path),
{
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs_path(&path, cb)?;
            } else {
                cb(&path);
            }
        }
    }
    Ok(())
}

fn visit_dirs<T>(dir: &Path, cb: &mut T) -> io::Result<()>
where
    T: FnMut(&DirEntry),
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
