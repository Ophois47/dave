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

pub fn scores_scraper(chosen_sport: String) -> io::Result<()> {
	let current_local: DateTime<Local> = Local::now();
	let date_format = current_local.format("%m-%d-%Y");
	println!(
		"##==>> Checking Current {} Scores for {} ...\n",
		chosen_sport,
		date_format,
	);

	// Format Current Date String to Match CBS-Sports URL Style
	let date_reqwest_format = current_local.format("%Y%m%d");
	let mut sport = "";
	if chosen_sport == "NBA" {
		sport = "nba";
	} else if chosen_sport == "NHL" {
		sport = "nhl";
	} else if chosen_sport == "MLB" {
		sport = "mlb";
	} else if chosen_sport == "MLS" {
		sport = "mls";
	} else if chosen_sport == "NFL" {
		sport = "nfl";
	} else if chosen_sport == "WNBA" {
		sport = "wnba";
	} else if chosen_sport == "NCAA-FB" {
		sport = "college-football";
	} else if chosen_sport == "NCAA-BB" {
		sport = "college-basketball";
	}

	// Look Up Scoreboard Based on User Chosen Sport
	let reqwest_string = format!("https://www.cbssports.com/{}/scoreboard/{}/", sport, date_reqwest_format);
	let response = reqwest::blocking::get(reqwest_string.clone())
		.unwrap()
		.text()
		.unwrap();

	// Keep Track of How Many Games Are/Were Scheduled
	let mut game_number = 0;
	let document = scraper::Html::parse_document(&response);
	let nba_game_selector = scraper::Selector::parse("div.live-update").unwrap();
	let nba_game_results = document.select(&nba_game_selector).map(|x| x.inner_html());
	let mut is_live = false;

	for game in nba_game_results {
		game_number += 1;
		let game_fragment = scraper::Html::parse_fragment(&game);
		let game_selector = scraper::Selector::parse("table").unwrap();
		let game_status = game_fragment.select(&game_selector).next().unwrap().inner_html();
		let fragment = scraper::Html::parse_fragment(&game_status);

		// Get Team Names
		let team_selector = scraper::Selector::parse("a.team-name-link").unwrap();
		let teams = fragment.select(&team_selector).map(|x| x.inner_html());
		let mut teams_vec = vec![];
		for team in teams.clone() {
			teams_vec.push(team);
		}
		println!("##==> -------------------------------------------");
		println!("##==> Game #{}", game_number);
		println!("##==> {} vs {}", teams_vec[0], teams_vec[1]);
		println!("##==> -------------------------------------------");

		// Get Odds For Future Games When Applicable
		let odds_selector = scraper::Selector::parse("td.in-progress-odds").unwrap();
		let odds = game_fragment.select(&odds_selector).map(|x| x.inner_html());
		let mut odds_vec = vec![];
		for odd in odds {
			odds_vec.push(odd);
		}
		// Only Display Odds When They Are Found
		if odds_vec.len() > 0 {
			println!("##==> Home Team {} Points Spread: {}", teams_vec[1], odds_vec[1]);
			println!("##==> Away Team {} Odds: {}", teams_vec[0], odds_vec[0]);
		}

		// Get Scores for Games That Happened
		let score_selector = scraper::Selector::parse("td.total").unwrap();
		let scores = game_fragment.select(&score_selector).map(|x| x.inner_html());
		let mut scores_vec = vec![];
		for score in scores {
			scores_vec.push(score);
		}
		// Only Display Scores When They Are Found
		if scores_vec.len() > 0 {
			println!("##==> The {} Scored {} pts", teams_vec[0], scores_vec[0]);
			println!("##==> The {} Scored {} pts", teams_vec[1], scores_vec[1]);
		}

		// Find What Network is Broadcasting the Game
		let broadcaster_selector = scraper::Selector::parse("div.broadcaster").unwrap();
		let broadcasters = game_fragment.select(&broadcaster_selector).map(|x| x.inner_html());
		let mut broadcaster_vec = vec![];
		for broadcaster in broadcasters {
			broadcaster_vec.push(broadcaster);
		}
		// Find Game Time
		let browser = headless_chrome::Browser::default().unwrap();
		let tab = browser.new_tab().unwrap();
		tab.navigate_to(&reqwest_string.as_str()).unwrap();
		let game_times = tab.wait_for_elements("span.formatter").unwrap();
		let mut times_vec = vec![];
		for game_time in game_times {
			times_vec.push(game_time.get_inner_text().unwrap());
		}

		// Find How Many Matches Are In Progress
		let live_matches_selector = scraper::Selector::parse("div.game-status.ingame").unwrap();
		let live_matches = game_fragment.select(&live_matches_selector).map(|x| x.inner_html());
		let mut live_matches_vec = vec![];
		for live_match in live_matches {
			live_matches_vec.push(live_match);
		}

		// TODO: Handle When Live Games Are Currently Happening
		// For Now It Wont Print Game Times Or Networks When 
		// Live Games Are Happening
		if live_matches_vec.len() > 0 {
			is_live = true;
		}

		// Print Information For Future Games
		if !is_live {
			if broadcaster_vec[0] != "" && times_vec.len() > 0 {
				println!(
					"##==> This Match Will Be Shown On {} at {}",
					broadcaster_vec[0],
					times_vec[game_number - 1],
				);
			} else if broadcaster_vec[0] == "" && times_vec.len() > 0 {
				println!("##==> This Match Will Be Shown at {}", times_vec[game_number - 1]);
			}
		}
		println!("##==> -------------------------------------------");
		println!();
	}

	if game_number == 0 {
		println!("##==>> There are no {} games today!", chosen_sport);
	}
	Ok(())
}
