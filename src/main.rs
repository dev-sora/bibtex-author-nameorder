use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // new empty string
    let mut new_contents = String::new();
    // lookup each line
    for line in contents.lines() {
        // if line contains 'author'
        if line.contains("author") {
            // split by '='
            let author_line: Vec<&str> = line.split("=").collect();
            // get author
            let author = author_line[1].trim();
            let author = &author[1..author.len() - 2];


            // split by 'and'
            let authors: Vec<&str> = author.split(" and ").collect();
            let mut new_authors = String::new();
            // if the name of author in authors divided by comma, last name first
            // if not, do nothing
            for name in authors {
                if name.contains(", ") {
                    let name = name.split(", ").collect::<Vec<&str>>();
                    // add to new string to new authors
                    new_authors.push_str(name[1]);
                    new_authors.push_str(" ");
                    new_authors.push_str(name[0]);
                    new_authors.push_str(" and ");
                } else {
                    // add to new string to new authors
                    new_authors.push_str(name);
                    new_authors.push_str(" and ");
                }
            }
            // remove last 'and'
            new_authors.pop();
            new_authors.pop();
            // add new authors to new string
            new_contents.push_str("author = {");
            new_contents.push_str(&new_authors);
            new_contents.push_str("},\n");
        } else {
            // add line to new string
            new_contents.push_str(line);
            new_contents.push_str("\n");
        }
    }

    // write new string to new file
    let mut new_filename = String::from("./new_");
    new_filename.push_str(filename);
    let mut new_file = File::create(new_filename).expect("Unable to create file");
    new_file
        .write_all(new_contents.as_bytes())
        .expect("Unable to write file");

}
