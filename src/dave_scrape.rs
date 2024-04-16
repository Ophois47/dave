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
		"https://www.digitalcombatsimulator.com/en/",
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

pub fn nba_scores_scraper() -> io::Result<()> {
	let current_local: DateTime<Local> = Local::now();
	let date_format = current_local.format("%m-%d-%Y");
	println!(
		"##==>> Checking Current Basketball Scores for {}\n",
		date_format,
	);

	let date_reqwest_format = current_local.format("%Y%m%d");
	let reqwest_string = format!("https://www.cbssports.com/nba/scoreboard/{}/", date_reqwest_format);
	let response = reqwest::blocking::get(reqwest_string)
		.unwrap()
		.text()
		.unwrap();

	let document = scraper::Html::parse_document(&response);
	let nba_game_selector = scraper::Selector::parse("div.in-progress-table.section").unwrap();
	let nba_game_results = document.select(&nba_game_selector).map(|x| x.inner_html());

	for game in nba_game_results {
		let game_fragment = scraper::Html::parse_fragment(&game);
		let game_selector = scraper::Selector::parse("table").unwrap();
		let game_status = game_fragment.select(&game_selector).next().unwrap().inner_html();
		let fragment = scraper::Html::parse_fragment(&game_status);

		let team_selector = scraper::Selector::parse("a.team-name-link").unwrap();
		let teams = fragment.select(&team_selector).map(|x| x.inner_html());
		let mut teams_vec = vec![];
		for team in teams.clone() {
			teams_vec.push(team);
		}
		println!("##==> The {} vs The {}", teams_vec[0], teams_vec[1]);

		// let score_selector = scraper::Selector::parse("td.total").unwrap();
		// let scores = fragment.select(&score_selector).map(|x| x.inner_html());
		/*let mut scores_vec = vec![];
		for score in scores {
			println!("SCORE: {:?}", score);
			scores_vec.push(score);
		}
		println!("SCORE_VEC: {:?}", scores_vec);*/

		println!();
	}
	Ok(())
}
