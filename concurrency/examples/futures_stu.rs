use trpl::{Either, Html};

fn main() {
    let urls = ["http://www.rust-lang.org", "http://www.baidu.com"];

    trpl::run(async {
        let title_fut_1 = page_title(urls[0]);
        let title_fut_2 = page_title(urls[1]);

        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };
        println!("{url} returned first!");
        match maybe_title {
            Some(title) => println!("{} has title: {}", url, title),
            None => println!("{} has no title", url),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}
