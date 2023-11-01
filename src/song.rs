use std::path::Path;
use id3::{Tag, TagLike};

pub struct Song {
    title: String,
    author: String
}


impl Song {
    pub fn new(title: String, author: String) -> Song {
        Song {
            title,
            author
        }
    }

    pub fn new_from_file(path: Box<Path>) -> Song {
        let tag = Tag::read_from_path(path).unwrap(); // TODO: This panics on most files!!!!!!! Hahahahahaha fix this silly
        Song {
            title: tag.title().expect("Unknown").to_owned(),
            author: tag.artist().expect("Unknown").to_owned()
        }
    }


    pub fn as_str(&self) -> String {
        format!("{} - {}", self.author.clone(), self.title.clone())
    }
}
