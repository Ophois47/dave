use std::io;

pub fn imdb_top100_scraper() -> io::Result<()> {
	let response = reqwest::blocking::get(
		"https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
	)
	.unwrap()
	.text()
	.unwrap();
	let document = scraper::Html::parse_document(&response);
	let title_selector = scraper::Selector::parse("h3.ipc-title ipc-title--base ipc-title--title ipc-title-link-no-icon ipc-title--on-textPrimary sc-b189961a-9 iALATN dli-title>a").unwrap();
	let titles = document.select(&title_selector).map(|x| x.inner_html());
	titles.zip(1..101).for_each(|(item, number)| println!("{}. {}", number, item));

	Ok(())
}
