use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::path::Path;
use xml::reader::{EventReader, XmlEvent::Characters};

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        // trim whitespaces from left
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_alphabetic() {
            let mut n = 0;
            while n < self.content.len() && self.content[n].is_alphanumeric() {
                n += 1;
            }

            let token = &self.content[0..n];
            self.content = &self.content[n..];
            return Some(token);
        }

        todo!("not implemented")
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn read_xml_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let file = File::open(file_path)?;
    let xml_event_reader = EventReader::new(file);
    let mut content = String::new();
    for event in xml_event_reader {
        if let Characters(text) = event.expect("TODO") {
            content.push_str(&text);
        }
    }
    Ok(content)
}

// results -> {"hello": 32, "world": 120, "hi": 20, "mom": 320}
fn index_file(_file_content: &str) -> HashMap<String, usize> {
    todo!("not implemented");
}

fn main() -> io::Result<()> {
    let content = read_xml_file("docs.gl/gl4/glVertexAttribDivisor.xhtml")?
        .chars()
        .collect::<Vec<_>>();

    let lexer = Lexer::new(&content);

    for token in lexer {
        println!("{token:?}");
    }

    // let all_documents = HashMap::<&str, HashMap<String, usize>>::new();

    // let dir_path = "docs.gl/gl4";
    // let dir = fs::read_dir(dir_path)?;
    // for file in dir {
    //     let file_path = file?.path();
    //     let content = read_xml_file(&file_path).unwrap();
    //     println!("{file_path:?} => {size}", size = content.len());
    // }

    Ok(())
}
