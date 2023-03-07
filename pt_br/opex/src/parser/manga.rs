use ebi_source::{error::SourceError, Chapter};

use scraper::{ElementRef, Html};

mod selectors {
    use scraper::Selector;

    pub fn span_selector() -> Selector {
        Selector::parse("span").unwrap()
    }

    pub fn online_anchor_selector() -> Selector {
        Selector::parse("a.online").unwrap()
    }

    fn main_manga_selector() -> Selector {
        Selector::parse("#volumes div.capitulos li.volume-capitulo").unwrap()
    }

    fn sbs_manga_selector() -> Selector {
        Selector::parse("#conteudo #post > a.text-uppercase.sombra-clara.bnt-lista-horizontal")
            .unwrap()
    }

    fn cover_manga_selector() -> Selector {
        Selector::parse("#post div.volume.text-uppercase div.capitulos li.volume-capitulo").unwrap()
    }

    pub fn chapter_list_selector(identifier: &str) -> Option<Selector> {
        match identifier {
            "main" => Some(main_manga_selector()),
            "sbs" => Some(sbs_manga_selector()),
            "covers" => Some(cover_manga_selector()),
            _ => None,
        }
    }
}

mod attrs {
    use scraper::{ElementRef, Selector};

    pub fn href_from_anchor(element: ElementRef) -> anyhow::Result<String> {
        let element = element
            .value()
            .attr("href")
            .ok_or(anyhow::anyhow!("MISSING_ELEMENT::href"))?;
        Ok(element.to_owned())
    }

    pub fn inner_from_child_selector(
        element: ElementRef,
        selector: Selector,
    ) -> anyhow::Result<String> {
        let element = element
            .select(&selector)
            .next()
            .ok_or(anyhow::anyhow!("COULDNT_GET_INNER_HTML"))?;
        Ok(element.inner_html())
    }

    pub fn href_from_child_selector(
        element: ElementRef,
        selector: Selector,
    ) -> anyhow::Result<String> {
        let url = element
            .select(&selector)
            .next()
            .ok_or(anyhow::anyhow!("COULDNT_GET_ONLINE_ANCHOR"))?;
        href_from_anchor(url)
    }
}

type ChapterInfo = (Option<u32>, String, String);

fn chapter_and_title_from_raw_title(base_title: &str) -> anyhow::Result<(u32, String)> {
    let mut id = "";
    let mut title = "";

    for (i, &item) in base_title.as_bytes().iter().enumerate() {
        if item == b'.' {
            id = &base_title[0..i];
            title = &base_title[i + 2..];
            break;
        }
    }

    let id = id.parse()?;
    Ok((id, String::from(title)))
}

fn main_chapter_info_from_element(element: ElementRef) -> anyhow::Result<ChapterInfo> {
    let base_title = attrs::inner_from_child_selector(element, selectors::span_selector())?;

    let anchor_selector = selectors::online_anchor_selector();
    let url = attrs::href_from_child_selector(element, anchor_selector)?;

    let (id, title) = chapter_and_title_from_raw_title(base_title.as_str())?;
    Ok((Some(id), title, url))
}

fn sbs_chapter_info_from_element(element: ElementRef) -> anyhow::Result<ChapterInfo> {
    let title = attrs::inner_from_child_selector(element, selectors::span_selector())?;
    let url = attrs::href_from_anchor(element)?;
    Ok((None, title, url))
}

fn covers_chapter_info_from_element(element: ElementRef) -> anyhow::Result<ChapterInfo> {
    let title = element
        .text()
        .next()
        .ok_or(anyhow::anyhow!("COULDNT_GET_TEXT"))?;
    let title = (&title[..title.len() - 1]).to_owned();

    let anchor_selector = selectors::online_anchor_selector();
    let anchor = element
        .select(&anchor_selector)
        .next()
        .ok_or(anyhow::anyhow!("COULDNT_GET_ONLINE_ANCHOR"))?;
    let url = attrs::href_from_anchor(anchor)?;

    Ok((None, title, url))
}

fn chapter_from_element(
    identifier: &str,
    element: ElementRef,
    idx: usize,
) -> Result<Chapter, SourceError> {
    let info = match identifier {
        "main" => Ok(main_chapter_info_from_element(element)),
        "sbs" => Ok(sbs_chapter_info_from_element(element)),
        "covers" => Ok(covers_chapter_info_from_element(element)),
        _ => Err(SourceError::InvalidIdentifier),
    }?;

    let (id, title, url) = info.map_err(|_| SourceError::Serialize)?;
    let chapter = match id {
        Some(id) => id,
        None => idx as u32,
    };

    Ok(Chapter {
        chapter,
        title,
        url,
        manga: String::from(identifier),
        source: crate::source_info().unwrap().identifier,
    })
}

pub fn chapter_list(identifier: &str, html: &str) -> Result<Vec<Chapter>, SourceError> {
    let html = Html::parse_document(html);

    let selector =
        selectors::chapter_list_selector(identifier).ok_or(SourceError::InvalidIdentifier)?;

    html.select(&selector)
        .into_iter()
        .enumerate()
        .map(|(idx, element)| chapter_from_element(identifier, element, idx))
        .collect()
}
