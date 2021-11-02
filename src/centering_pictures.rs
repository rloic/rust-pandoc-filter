mod definition;
mod walk;
mod utils;

use std::process::exit;
use definition::Inline;
use definition::Inline::Image;
use definition::Pandoc;
use utils::{latex_inline, RemoveBy};
use walk::Walkable;

extern crate serde;
extern crate serde_json;

fn main() {
//    let format = &std::env::args().collect::<Vec<_>>()[1];

    let mut json = String::new();
    std::io::stdin().read_line(&mut json).unwrap();

    let mut res = serde_json::from_str::<Pandoc>(&json)
        .map(|it| it.walk(&mut utils::concat_map(&mut centering_pictures)))
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

pub(crate) fn centering_pictures(inline: Inline) -> Vec<Inline> {
    match inline {
        Image(mut attr, inlines, target) if attr.classes.iter().any(|it| it == "center") => {
            attr.classes.remove_by(|it| it == &String::from("center"));
            vec![
                latex_inline("\\hfill\\break{\\centering"),
                Image(attr, inlines, target),
                latex_inline("\\par}")
            ]
        },
        _ => vec![inline]
    }
}