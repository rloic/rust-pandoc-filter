mod definition;
mod walk;
mod utils;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::process::exit;
use definition::{Inline, Inline::Span, Meta, MetaValue, Pandoc};
use utils::{latex_inline, concat_map, RemoveBy};
use walk::Walkable;


extern crate serde;
extern crate serde_json;

fn main() {
    let mut json = String::new();
    std::io::stdin().read_line(&mut json).unwrap();

    let mut res = serde_json::from_str::<Pandoc>(&json)
        .map(|it| it.walk(&mut acronyms))
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

pub fn acronyms(pandoc: Pandoc) -> Pandoc {
    let Pandoc { version, mut meta, mut blocks } = pandoc;

    let acronyms = if let Ok(file) = File::open("metadata.json") {
        let reader = BufReader::new(file);
        let mut acronyms = serde_json::from_reader(reader).unwrap();
        let mut_ref = &mut acronyms;
        let mut insert = move |inline: Inline| { apply_acronyms(inline, mut_ref) };
        blocks = blocks.walk(&mut concat_map(&mut insert));
        acronyms
    } else {
        let mut acronyms = HashMap::new();
        let mut record_acronyms = |inline: Inline| { record_acronyms(inline, &mut acronyms) };
        blocks = blocks.walk(&mut record_acronyms);
        acronyms
    };

    {
        let tmp = File::create("metadata.json").unwrap();
        let writer = BufWriter::new(tmp);
        serde_json::to_writer(writer, &acronyms).unwrap();
    }

    let pandoc = Pandoc { version, meta, blocks };
    pandoc
}

fn record_acronyms(inline: Inline, acronyms: &mut HashMap<String, bool>) -> Inline {
    match inline {
        Span(mut attr, mut content) if attr.classes.first().map(|it| it == &String::from("acronym")).unwrap_or(false) => {
            attr.classes.remove_by(|it| it == &String::from("acronym"));
            acronyms.insert(attr.id.clone(), false);
            content.insert(0, latex_inline(format!("\\hypertarget{{{}}}{{\\textbf{{{}}}}}", &attr.id, &attr.id)));
            content.insert(1, Inline::Str(String::from(" (")));
            content.push(Inline::Str(String::from(")")));
            content.push(latex_inline(format!("\\\\")));
            attr.id = String::new();
            Span(attr, content)
        }
        _ => inline
    }
}

fn apply_acronyms(inline: Inline, acronyms: &mut HashMap<String, bool>) -> Vec<Inline> {
    match inline {
        Inline::Str(s) if acronyms.keys().any(|it| s.contains(it)) => {
            let mut builder = Vec::<Inline>::new();

            let mut prev = 0;
            for (acronym, is_declared) in acronyms.iter_mut() {
                if let Some(pos) = s.find(acronym) {
                    //  *is_declared {
                        builder.push(Inline::Str(String::from(&s[prev..pos])));
                        builder.push(latex_inline(format!("\\hyperlink{{{}}}{{{}}}", acronym, acronym)));
                        prev = pos + acronym.len();
                    /*} else {
                        *is_declared = true;
                        change = true;
                    }*/
                }
            }

            builder.push(Inline::Str(String::from(&s[prev..])));

            builder
        }
        _ => vec![inline]
    }
}