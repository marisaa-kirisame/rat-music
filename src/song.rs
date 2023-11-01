use std::ops::Deref;
use std::path::Path;
use id3::{Tag, TagLike};

pub struct Song {
    title: String,
    author: Option<String>
}


impl Song {
    pub fn new(title: String, author: Option<String>) -> Song {
        Song {
            title,
            author
        }
    }

    pub fn new_from_file(path: Box<Path>) -> Song {
        let tag = Tag::read_from_path(path.clone());
        match tag {
            Ok(tag_content) => {
                Song {
                    title: tag_content.title().expect("Unknown").to_owned(),
                    author: Some(tag_content.artist().expect("Unknown").to_owned())
                }
            },
            Err(_) => {
                Song {
                    title: String::from(path.file_name().unwrap().to_str().unwrap()),
                    author: None
                }
            }
        }

    }


    pub fn as_str(&self) -> String {
        match self.author.clone() {
            Some(a) => { format!("{} - {}", a.clone(), self.title.clone()) },
            None => { format!("{}", self.title.clone()) }
        }

    }
}
