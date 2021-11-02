mod definition;
mod walk;
mod utils;

use std::process::exit;
use definition::{Pandoc, Citation, Inline};
use utils::latex_inline;
use walk::Walkable;

extern crate serde;
extern crate serde_json;

fn main() {
//    let format = &std::env::args().collect::<Vec<_>>()[1];

    let mut json = String::new();
    std::io::stdin().read_line(&mut json).unwrap();

    let mut res = serde_json::from_str::<Pandoc>(&json)
        .map(|it| it.walk(&mut cross_reference))
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

const PREFIX: [&str; 7] = ["fig:", "lst:", "eqn:", "tbl:", "sec:", "algo:", "thm:"];

pub(crate) fn cross_reference(inline: Inline) -> Inline {
    match inline {
        Inline::Cite( citations, .. ) if citations.iter().all(is_cross_reference) => {
            let citations = citations.iter().map(|it| it.citation_id.clone())
                .collect::<Vec<_>>()
                .join(", ");

            latex_inline(format!("\\ref{{{}}}", citations))
        },
        Inline::Cite( citations, .. ) if citations.iter().all(is_source_citation) => {
            let citations = citations.iter().map(|it| it.citation_id.clone())
                .collect::<Vec<_>>()
                .join(", ");

            latex_inline(format!("\\cite{{{}}}", citations))
        },
        _ => inline
    }
}

fn is_cross_reference(citation: &Citation) -> bool {
    PREFIX.iter().any(|prefix| citation.citation_id.starts_with(prefix))
}

fn is_source_citation(citation: &Citation) -> bool {
    !is_cross_reference(citation)
}

