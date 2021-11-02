mod definition;
mod walk;
mod utils;

use std::process::{Command, exit};
use definition::{Pandoc, Block, Block::CodeBlock};
use definition::Attr;
use walk::Walkable;

extern crate serde;
extern crate serde_json;
extern crate tera;

fn main() {
//    let format = &std::env::args().collect::<Vec<_>>()[1];

    let mut json = String::new();
    std::io::stdin().read_line(&mut json).unwrap();

    let mut res = serde_json::from_str::<Pandoc>(&json)
        .map(|it| it.walk(&mut utils::concat_map(&mut include)))
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

pub(crate) fn include(block: Block) -> Vec<Block> {
    match block {
        CodeBlock(Attr { id, classes, attributes }, content) if !classes.is_empty() && classes[0] == "include" => {
            let lines = content.trim().split("\n");
            let mut imports = Vec::new();
            for line in lines {
                if !line.is_empty() && !line.starts_with("#") {
                    let output = Command::new("mdtex")
                        .arg(line)
                        .arg("include")
                        .output()
                        .unwrap();

                    let json = String::from_utf8(output.stdout).unwrap();

                    if let Ok(mut pandoc_ast) = serde_json::from_str::<Pandoc>(&json) {
                        imports.append(&mut pandoc_ast.blocks)
                    } else {
                        return vec![CodeBlock(Attr { id, classes, attributes}, content)];
                    }
                }
            }
            imports
        }
        _ => vec![block]
    }
}