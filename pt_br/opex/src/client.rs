use anyhow::Result;

const BASE_URL: &str = "https://onepieceex.net";

const ACCEPT_HEADER: &str = "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9";
const REFERER_HEADER: &str = "https://onepieceex.net/";
const ACCEPT_LANGUAGE_HEADER: &str = "pt-BR,pt;q=0.9,en-US;q=0.8,en;q=0.7,es;q=0.6,gl;q=0.5";

pub fn opex_html_page(page_url: &str) -> Result<String> {
    let url = format!("{}{}", BASE_URL, page_url);
    let body = ureq::get(url.as_str())
        .set("accept", ACCEPT_HEADER)
        .set("referer", REFERER_HEADER)
        .set("accept-language", ACCEPT_LANGUAGE_HEADER)
        .call()?
        .into_string()?;
    Ok(body)
}
