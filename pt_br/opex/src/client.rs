use anyhow::Result;

use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::Client;

const BASE_URL: &str = "https://onepieceex.net";

const ACCEPT_HEADER: &str = "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9";
const REFERER_HEADER: &str = "https://onepieceex.net/";
const ACCEPT_LANGUAGE_HEADER: &str = "pt-BR,pt;q=0.9,en-US;q=0.8,en;q=0.7,es;q=0.6,gl;q=0.5";

pub async fn opex_html_page(page_url: &str) -> Result<String> {
    let url = format!("{}{}", BASE_URL, page_url);

    let mut headers = HeaderMap::new();
    headers.insert(header::ACCEPT, ACCEPT_HEADER.parse()?);
    headers.insert(header::REFERER, REFERER_HEADER.parse()?);
    headers.insert(header::ACCEPT_LANGUAGE, ACCEPT_LANGUAGE_HEADER.parse()?);

    let client = Client::builder().default_headers(headers).build()?;
    let body = client.get(url).send().await?.text().await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::opex_html_page;

    #[tokio::test]
    async fn manga_page() {
        let page = opex_html_page("/mangas").await;
        assert!(page.is_ok());

        let page = page.unwrap();
        assert!(!page.contains("<h2>Erro 404</h2>"));
        assert!(page.contains("<h1>Mangá</h1>"));
        assert!(page.contains("<h2>Todos os Volumes de One Piece e algo mais =)</h2>"));
    }

    #[tokio::test]
    async fn chapter_page() {
        let page = opex_html_page("/mangas/leitor/1047").await;
        assert!(page.is_ok());

        let page = page.unwrap();
        println!("{}", page);
        assert!(!page.contains("<h2>Erro 404</h2>"));
        assert!(page.contains("paginasLista = \""));
    }

    #[tokio::test]
    async fn invalid_page() {
        let page = opex_html_page("/invalid-page-that-doesnt-exist").await;
        assert!(page.is_ok());

        let page = page.unwrap();
        assert!(page.contains("<h2>Erro 404</h2>"));
    }
}
