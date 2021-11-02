mod definition;
mod walk;
mod utils;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::{Command, exit};
use definition::{Block};
use utils::{latex_block};
use definition::{Attr, Meta, MetaValue, Pandoc};
use walk::Walkable;

extern crate serde;
extern crate serde_json;

fn main() {
//    let format = &std::env::args().collect::<Vec<_>>()[1];

    let mut json = String::new();
    std::io::stdin().read_line(&mut json).unwrap();

    let mut res = serde_json::from_str::<Pandoc>(&json)
        .map(|it| it.walk(&mut input))
        .map_err(|e| e.to_string());

    match res {
        Ok(pandoc_ast) => println!("{}", serde_json::to_string(&pandoc_ast).unwrap()),
        Err(e) => {
            eprintln!("{}", e);
            println!("{}", json);
            exit(1);
        }
    }
}

pub(crate) fn input(block: Block) -> Block {
    match block {
        Block::CodeBlock(Attr { classes, .. }, content) if !classes.is_empty() && classes[0] == "input" => {
            let lines = content.trim().split("\n");

            let mut used_lines = Vec::new();

            for line in lines {

                if !line.is_empty() && !line.starts_with("#") {
                    let output = Command::new("mdtex")
                        .arg(line)
                        .arg("input")
                        .output()
                        .unwrap();

                    let mut content = String::from_utf8(output.stdout).unwrap();

                    if let Some(start_idx) = content.find("\\mainmatter") {
                        if let Some(end_idx) = content.find("\\backmatter") {
                            content = String::from(&content[start_idx + 11..end_idx]);
                        }
                    }

                    let tex_file = line.replace(".md", ".tex");

                    let file = File::create(&tex_file);
                    let mut writer = BufWriter::new(file.unwrap());

                    writer.write_all(content.as_bytes()).unwrap();

                    used_lines.push(format!("\\input{{{}}}", &tex_file));
                }
            }

            latex_block(used_lines.join("\n"))
        }
        _ => block
    }
}