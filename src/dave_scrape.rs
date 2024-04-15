use std::io;

pub fn imdb_top100_scraper() -> io::Result<()> {
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

pub fn bball_scraper() -> io::Result<()> {
	let html_content = reqwest::blocking::get(
		"https://www.espn.com/nba/scoreboard",
	)
	.unwrap()
	.text()
	.unwrap();
	let document = scraper::Html::parse_document(&html_content);
	let html_score_selector = scraper::Selector::parse("ul.ScoreboardScoreCell__Competitors").unwrap();
	let html_scores = document.select(&html_score_selector);

	for html_score in html_scores {
		println!("HTML SCORE: {:?}", html_score);
		let winner = html_score
			.select(&scraper::Selector::parse("li").unwrap())
			.next()
			.and_then(|a| a.value().attr("ScoreboardScoreCell__Item--winner"))
			.map(str::to_owned);
		println!("WINNER: {:?}", winner);

		let loser = html_score
			.select(&scraper::Selector::parse("li").unwrap())
			.next()
			.and_then(|a| a.value().attr("ScoreboardScoreCell__Item--loser"))
			.map(str::to_owned);
		println!("LOSER: {:?}", loser);
	}
	
	Ok(())
}
