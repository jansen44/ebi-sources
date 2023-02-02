use opex::manga_list;

use opex::client::opex_html_page;
use opex::parser::manga as manga_parser;

fn main() {
    for manga in manga_list().iter() {
        let manga_page = opex_html_page(&manga.url).unwrap();
        let chapter_list = manga_parser::chapter_list(&manga.identifier, &manga_page).unwrap();

        chapter_list
            .iter()
            .for_each(|chapter| println!("{:?}", chapter));
    }
}
