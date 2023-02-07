pub mod client;
pub mod parser;

use ebi_source::error::SourceError;
use ebi_source::prelude::*;
use ebi_source::{locale, Chapter, Manga, Source};
use ebi_source_macros::ebi_plugin;

const SOURCE_IDENTIFIER: &str = "opex";
const SOURCE_TITLE: &str = "One Piece Ex";
const SOURCE_DESCRIPTION: &str = "One Piece Ex | De fã para fã";

#[ebi_plugin]
pub fn source() -> Result<Source, SourceError> {
    Ok(Source {
        identifier: SOURCE_IDENTIFIER.to_owned(),
        title: SOURCE_TITLE.to_owned(),
        description: SOURCE_DESCRIPTION.to_owned(),
        locale: locale::Locale::PtBr,
    })
}

#[ebi_plugin]
pub fn manga_list() -> Result<Vec<Manga>, SourceError> {
    let main = Manga {
        identifier: String::from("main"),
        title: String::from("One Piece"),
        cover: String::from("https://onepieceex.net/mangareader/sbs/capa/preview/Volume_1.jpg"),
        url: String::from("/mangas"),
        genres: vec![String::from("shounen"), String::from("fantasy")],
        description: None,
        source_identifier: SOURCE_IDENTIFIER.to_string(),
    };

    let cover = Manga {
        identifier: String::from("covers"),
        title: String::from("One Piece - Histórias de Capa"),
        cover: String::from("https://onepieceex.net/mangareader/mangas/428/00_c.jpg"),
        url: String::from("/historias-de-capa"),
        genres: vec![String::from("shounen"), String::from("fantasy")],
        description: None,
        source_identifier: SOURCE_IDENTIFIER.to_string(),
    };

    let sbs = Manga {
        identifier: String::from("sbs"),
        title: String::from("One Piece - SBS"),
        cover: String::from("https://onepieceex.net/mangareader/sbs/capa/preview/nao.jpg"),
        url: String::from("/sbs"),
        genres: vec![String::from("shounen"), String::from("fantasy")],
        description: None,
        source_identifier: SOURCE_IDENTIFIER.to_string(),
    };

    Ok(vec![main.into(), cover.into(), sbs.into()])
}

#[ebi_plugin]
pub fn chapter_list(
    manga_identifier: String,
    manga_url: String,
) -> Result<Vec<Chapter>, SourceError> {
    let manga_page = client::opex_html_page(&manga_url)?;
    parser::manga::chapter_list(&manga_identifier, &manga_page)
}
