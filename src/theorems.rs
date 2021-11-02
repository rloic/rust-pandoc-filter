mod definition;
mod walk;
mod utils;

use std::process::exit;
use definition::{Block, Pandoc};
use utils::{latex_block};
use utils::{RemoveBy, RemoveByKey};
use definition::{Attr, MetaValue};
use walk::Walkable;

extern crate serde;
extern crate serde_json;

fn main() {
//    let format = &std::env::args().collect::<Vec<_>>()[1];

    let mut json = String::new();
    std::io::stdin().read_line(&mut json).unwrap();

    let mut res = serde_json::from_str::<Pandoc>(&json)
        .map(|it| it.walk(&mut theorems))
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

const ENVIRONMENTS: [&str; 4] = [
    "definition", "proof", "theorem", "lemma"
];

pub(crate) fn theorems(mut p: Pandoc) -> Pandoc {
    let header_includes = p.meta
        .entry(String::from("header-includes"))
        .or_insert(MetaValue::MetaList(Vec::new()));

    match header_includes {
        MetaValue::MetaList(inlines) => {
            if !is_init(inlines) {
                inlines.push(MetaValue::MetaBlocks(vec![
                    latex_block(String::from("\\usepackage{amsthm}")),
                    latex_block(String::from("\\newtheorem{definition}{Definition}")),
                    // latex_block(String::from("\\newtheorem{proof}{Proof}")),
                    latex_block(String::from("\\newtheorem{theorem}{Theorem}")),
                    latex_block(String::from("\\newtheorem{lemma}{Lemma}")),
                ]));
            }
        }
        _ => {}
    };

    p.walk(&mut th)
}

fn th(block: Block) -> Block {
    match block {
        Block::Div(Attr { id, mut classes, mut attributes }, mut content) if ENVIRONMENTS.iter().any(|env| classes.contains(&String::from(*env))) => {
            let env = classes.remove_by(|it| ENVIRONMENTS.contains(&it.as_str())).unwrap();
            let name = attributes.remove_by_key("name")
                .unwrap_or(String::new());
            let mut new_content = Vec::with_capacity(content.len() + 2);
            if !id.is_empty() && !name.is_empty() {
                new_content.push(latex_block(format!("\\begin{{{}}}[\\hypertarget{{{}}}{{{}}}]", env, id, name)));
            } else if !id.is_empty() {
                new_content.push(latex_block(format!("\\begin{{{}}}", env)));
                new_content.push(latex_block(format!("\\label{{{}}}", id)));
            } else {
                new_content.push(latex_block(format!("\\begin{{{}}}", env)));
            }
            new_content.append(&mut content);
            new_content.push(latex_block(format!("\\end{{{}}}", env)));

            Block::Div(Attr { id: String::new(), classes, attributes }, new_content)
        }
        _ => block
    }
}

fn is_init(inlines: &Vec<MetaValue>) -> bool {
    for inline in inlines {
        let is_valid = match inline {
            MetaValue::MetaString(s) => s.find("amsthm").is_some(),
            MetaValue::MetaBlocks(child) => child.iter().any(is_init_block),
            _ => false
        };
        if is_valid { return true; }
    }
    false
}

fn is_init_block(block: &Block) -> bool {
    match block {
        Block::RawBlock(f, s) if f == &String::from("latex") || f == &String::from("tex") => s.find("amsthm").is_some(),
        _ => false
    }
}