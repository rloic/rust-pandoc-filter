mod definition;
mod walk;
mod utils;

use std::process::exit;
use tera::{Context, Tera};
use Block::CodeBlock;
use definition::{Attr, AttrList, Block, Inline, Meta, Pandoc};
use utils::{latex_block, None};
use definition::MetaValue::{MetaBlocks, MetaInlines, MetaList, MetaMap};
use walk::Walkable;
use serde::{Serialize, Deserialize};
use utils::RemoveByKey;

extern crate serde;
extern crate serde_json;
extern crate tera;

fn main() {
    let mut json = String::new();
    std::io::stdin().read_line(&mut json).unwrap();

    let mut res = serde_json::from_str::<Pandoc>(&json)
        .map(|it| it.walk(&mut minted))
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

const TEMPLATE_NAME: &str = "minted";

const TEMPLATE: &str = "\
{% set L_CB = '{' %}\
{% set R_CB = '}' %}\
\\begin{listing}
\\begin{minted}[{{ attributes }}]{{ L_CB }}{{ language }}{{ R_CB }}
{{ content }}
\\end{minted}
{% if label %}\
  \\label{{ L_CB }}{{ label | safe }}{{ R_CB }}
{% endif %}\
{% if caption %}\
  \\caption{{ L_CB }}{{ caption | safe }}{{ R_CB }}
{% endif %}\
\\end{listing}
";

pub fn minted(pandoc: Pandoc) -> Pandoc {
    let mut tera = Tera::default();
    tera.add_raw_template(TEMPLATE_NAME, TEMPLATE).unwrap();

    let Pandoc { version, mut meta, blocks } = pandoc;
    let Settings{ default_language, theme} = unpack_metadata(&meta);

    if let Some(theme) = theme {
        meta = install_theme(theme, meta);
    }

    Pandoc { version, meta, blocks: blocks.walk(&mut |block: Block| { transform_block(block, &default_language, &tera) }) }
}

fn transform_block(block: Block, default_language: &String, tera: &Tera) -> Block {
    match block {
        CodeBlock(Attr { id, classes, attributes }, contents) if classes.iter().none(|it| it.starts_with('=')) => {
            let label = if id.is_empty() { None } else { Some(id) };
            let code = unpack_code(label, classes, attributes, contents, default_language.clone());
            let context = Context::from_serialize(&code).unwrap();
            latex_block(tera.render(TEMPLATE_NAME, &context).unwrap())
        }
        _ => block
    }
}

fn install_theme(theme: String, mut meta: Meta) -> Meta {
    let includes = meta.entry(String::from("header-includes")).or_insert(MetaList(Vec::new()));
    if let MetaList(includes) = includes {
        includes.push(MetaBlocks(vec![latex_block(format!("\\usemintedstyle{{{}}}", theme))]));
    }
    meta
}

struct Settings {
    default_language: String,
    theme: Option<String>
}

fn unpack_metadata(meta: &Meta) -> Settings {
    let empty_map = MetaMap(Meta::new());

    let mut user_theme = None;

    let settings = meta.get("pandoc-minted")
        .unwrap_or(&empty_map);

    if let MetaMap(settings) = settings {
        let language = settings.get("default-language")
            .unwrap_or(&empty_map);

        let theme = settings.get("theme")
            .unwrap_or(&empty_map);

        if let MetaInlines(theme) = theme {
            if let Inline::Str(theme) = &theme[0] {
                user_theme = Some(theme.clone());
            }
        }

        if let MetaInlines(language) = language {
            if let Inline::Str(language) = &language[0] {
                return Settings { default_language: language.clone(), theme: user_theme };
            }
        }
    }

    Settings { default_language: String::from("text"), theme: user_theme }
}

#[derive(Serialize, Deserialize)]
struct Code {
    content: String,
    language: String,
    attributes: String,
    label: Option<String>,
    caption: Option<String>,
}

fn unpack_code(label: Option<String>, classes: Vec<String>, mut attributes: AttrList, content: String, default_language: String) -> Code {
    let language = classes.first()
        .unwrap_or(&default_language)
        .to_owned();

    let caption = attributes.remove_by_key("caption");

    let attributes = attributes.iter()
        .map(join("=", |value| value.is_empty()))
        .collect::<Vec<_>>()
        .join(", ");

    Code {
        content,
        language,
        attributes,
        label,
        caption
    }
}

fn join<S: Into<String>, F>(delimiter: S, ignore_value: F) -> impl Fn(&(String, String)) -> String
    where F: Fn(&String) -> bool {
    let d = delimiter.into();
    move |(k, v): &(String, String)| {
        if ignore_value(v) {
            k.to_string()
        } else {
            format!("{}{}{}", k, d, v).to_string()
        }
    }
}