use ebi_source::error::SourceError;
use scraper::Html;

mod selectors {
    use scraper::Selector;

    pub fn script_selector() -> Selector {
        Selector::parse("#leitor-opex > strong > script").unwrap()
    }
}

pub fn chapter_page_list(chapter_page_body: &str) -> Result<Vec<String>, SourceError> {
    let page = Html::parse_document(chapter_page_body);

    let script_selector = selectors::script_selector();
    let script_elem = page
        .select(&script_selector)
        .next()
        .ok_or(SourceError::Serialize)?;

    let page_list_json = script_elem.inner_html();
    let page_list_json = page_list_json
        .split("paginasLista = ")
        .nth(1)
        .ok_or(SourceError::Serialize)?;

    let page_list_json = page_list_json
        .split(";")
        .next()
        .ok_or(SourceError::Serialize)?;

    let page_list_json =
        serde_json::from_str::<String>(page_list_json).map_err(|_| SourceError::Serialize)?;
    let page_list_json: serde_json::Value =
        serde_json::from_str(page_list_json.as_str()).map_err(|_| SourceError::Serialize)?;
    let page_list_json = page_list_json.as_object().ok_or(SourceError::Serialize)?;

    let mut page_list = page_list_json
        .iter()
        .map(|(key, value)| {
            Ok((
                key.parse::<usize>().map_err(|_| SourceError::Serialize)?,
                value.as_str().unwrap().to_owned(),
            ))
        })
        .collect::<Result<Vec<(usize, String)>, SourceError>>()?;
    page_list.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(page_list
        .iter()
        .map(|(_, value)| format!("{}/{}", crate::client::BASE_URL, value))
        .collect())
}
