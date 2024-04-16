use std::io;
use chrono::{DateTime, Local};

pub fn imdb_top100_scraper() -> io::Result<()> {
	let current_local: DateTime<Local> = Local::now();
	let date_format = current_local.format("%m-%d-%Y");
	println!(
		"##==>> Current IMDB Top 100 as of {}\n",
		date_format,
	);

	let response = reqwest::blocking::get(
		"https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
	)
	.unwrap()
	.text()
	.unwrap();
	let document = scraper::Html::parse_document(&response);
	let title_selector = scraper::Selector::parse("h3.ipc-title__text").unwrap();
	let titles = document.select(&title_selector).map(|x| x.inner_html());

	titles.zip(1..101).for_each(|(item, _number)| println!("{}", item));
	Ok(())
}

pub fn dcs_news_scraper() -> io::Result<()> {
	let current_local: DateTime<Local> = Local::now();
	let date_format = current_local.format("%m-%d-%Y");
	println!(
		"##==>> Current News Stories Posted on 'https://www.digitalcombatsimulator.com/en/' as of {}\n",
		date_format,
	);

	let response = reqwest::blocking::get(
		"https://www.digitalcombatsimulator.com/en/"
	)
	.unwrap()
	.text()
	.unwrap();
	let document = scraper::Html::parse_document(&response);
	let news_selector = scraper::Selector::parse("div.news-item").unwrap();
	let news_posts = document.select(&news_selector).map(|x| x.inner_html());

	for post in news_posts {
		let fragment = scraper::Html::parse_fragment(&post);
		let date_selector = scraper::Selector::parse("div.date").unwrap();
		let post_selector = scraper::Selector::parse("a").unwrap();

		let post_input = fragment.select(&post_selector).next().unwrap().inner_html();
		println!("##==> Post Title: {:?}", post_input);
		let date_input = fragment.select(&date_selector).next().unwrap().inner_html();
		println!("##==> Date Posted: {:?}", date_input);
		println!();
	}

	Ok(())
}
