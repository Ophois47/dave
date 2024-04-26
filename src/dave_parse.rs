use std::fs;
use std::io;
use std::path::PathBuf;
use file_format::FileFormat;
use indexmap::IndexMap;
use nom::{
	self,
	IResult,
	branch::alt,
	bytes::complete::{is_not, tag, take, take_until},
	character::complete::{char, multispace0},
	combinator::{map, value},
	multi::separated_list0,
	number::complete::{be_u16, double},
	sequence::{delimited, separated_pair},
};

// JSON Parser
#[derive(Debug, PartialEq, Clone)]
enum JsonNode<'a> {
	Object(Box<IndexMap<&'a str, JsonNode<'a>>>),
	Array(Vec<JsonNode<'a>>),
	String(&'a str),
	Number(f64),
	Boolean(bool),
	Null,
}

fn parse_null(json: &str) -> IResult<&str, JsonNode> {
	value(JsonNode::Null, tag("null"))(json)
}

fn parse_boolean(json: &str) -> IResult<&str, JsonNode> {
	alt((
		value(JsonNode::Boolean(true), tag("true")),
		value(JsonNode::Boolean(false), tag("false")),
	))(json)
}

// Parse String and Wrap Into JsonNode
fn parse_string(json:&str) -> IResult<&str, JsonNode> {
	map(parse_string_inner, JsonNode::String)(json)
}

// Parse String and Return "Raw" Without Building JsonNode
fn parse_string_inner(json: &str) -> IResult<&str, &str> {
	delimited(tag("\""), take_until("\""), tag("\""))(json)
}

fn parse_number(json: &str) -> IResult<&str, JsonNode> {
	map(double, JsonNode::Number)(json)
}

fn parse_array(json: &str) -> IResult<&str, JsonNode> {
	map(
		// An Array is Delimited by '[]'
		delimited(
			tag("["),
			// Entries Seperated by ',' But Optionally Empty
			separated_list0(
				delimited(multispace0, tag(","), multispace0),
				parse_json
			),
			tag("]"),
		),
		JsonNode::Array,
	)(json)
}

fn parse_object(json: &str) -> IResult<&str, JsonNode> {
	map(
		// Object Delimited By '{}'
		delimited(
			tag("{"),
			separated_list0(
				tag(","),
				separated_pair(
					delimited(multispace0, parse_string_inner, multispace0),
					tag(":"),
					delimited(multispace0, parse_json, multispace0),
				),
			),
			tag("}"),
		),
		|v| JsonNode::Object(Box::new(v.into_iter().collect())),
	)(json)
}

fn parse_json(json: &str) -> IResult<&str, JsonNode> {
	alt((
		parse_object,
		parse_array,
		parse_number,
		parse_string,
		parse_boolean,
		parse_null,
	))(json)
}

#[allow(dead_code)]
fn length_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
	let (input, length) = be_u16(input)?;
	take(length)(input)
}

#[allow(dead_code)]
fn parens(input: &str) -> IResult<&str, &str> {
	delimited(char('('), is_not(")"), char(')'))(input)
}

fn parse_file(file: PathBuf) -> io::Result<()> {
	let fmt = FileFormat::from_file(file.clone())?;
	println!("##==> File: '{}'", file.display());
	println!("##==>> {} : {}", fmt.name(), fmt.short_name().unwrap_or(" "));
	match fs::read_to_string(file.clone()) {
		Ok(contents) => {
			println!("##==>> Contents of File:\n");
			println!("-----------------------------------------------------------------------------------------");
			println!("{}", contents);
			println!("-----------------------------------------------------------------------------------------\n");
			match fmt.short_name() {
				Some("BIN") => {
					let sanitized_contents = contents
						.replace("\n", "")
						.replace("\t", "")
						.replace(" ", "");
					match parse_json(&sanitized_contents) {
						Ok(result) => {
							println!("##==> Valid JSON Format Found. Running JSON Parser ...");
							println!("##==>> JSON Parsed Results: {:#?}", result);
						},
						Err(error) => eprintln!("##==>>>> ERROR: {}", error),
					};
				},
				_ => println!("##==>>>> Some Other Format"),
			};
		},
		_ => return Ok(()),
	}
	Ok(())
}

pub fn parse_handle_file(filename: PathBuf) -> io::Result<()> {
	let canonical_file = filename.canonicalize()?.clone();
	let metadata = fs::metadata(canonical_file)?;
	if metadata.is_file() {
		if let Err(error) = parse_file(filename) {
			eprintln!("##==>>>> ERROR: {}", error);
		}
	} else if metadata.is_dir() {
		println!("##==> Directory: '{}'", filename.display());
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use indexmap::IndexMap;
	use nom::error::make_error;
	use crate::dave_parse::{
		parse_array,
		parse_boolean,
		parse_null,
		parse_number,
		parse_object,
		parse_string,
		JsonNode,
	};

	#[test]
	fn can_parse_null() {
		assert_eq!(Ok(("", JsonNode::Null)), parse_null("null"));
		assert_eq!(
			parse_null("something"),
			Err(nom::Err::Error(make_error(
				"something",
				nom::error::ErrorKind::Tag
			)))
		);
	}

	#[test]
	fn can_parse_boolean() {
		assert_eq!(Ok(("", JsonNode::Boolean(true))), parse_boolean("true"));
		assert_eq!(Ok(("", JsonNode::Boolean(false))), parse_boolean("false"));
		assert_eq!(
			parse_boolean("something"),
			Err(nom::Err::Error(make_error(
				"something",
				nom::error::ErrorKind::Tag
			)))
		);
	}

	#[test]
	fn can_parse_string() {
		assert_eq!(Ok(("", JsonNode::String("abc"))), parse_string("\"abc\""));
		assert_eq!(
			parse_string("something"),
			Err(nom::Err::Error(make_error(
				"something",
				nom::error::ErrorKind::Tag
			)))
		);
	}

	#[test]
	fn can_parse_numbers() {
		assert_eq!(Ok(("", JsonNode::Number(42f64))), parse_number("42"));
		assert_eq!(Ok(("", JsonNode::Number(1.2f64))), parse_number("1.2"));
		assert_eq!(Ok(("", JsonNode::Number(1.3e4f64))), parse_number("1.3e4"));
		assert_eq!(Ok(("", JsonNode::Number(0.14f64))), parse_number(".14"));
		assert_eq!(
			parse_number("something"),
			Err(nom::Err::Error(make_error(
				"something",
				nom::error::ErrorKind::Float
			)))
		);
	}

	#[test]
	fn can_parse_array() {
		assert_eq!(Ok(("", JsonNode::Array(Vec::new()))), parse_array("[]"));
		assert_eq!(
			Ok(("", JsonNode::Array(vec![JsonNode::Boolean(true)]))),
			parse_array("[true]")
		);
		assert_eq!(
			Ok((
				"",
				JsonNode::Array(vec![
					JsonNode::Boolean(false),
					JsonNode::Null,
					JsonNode::Boolean(false),
				])
			)),
			parse_array("[false, null, false]")
		);
		assert_eq!(
			parse_array("something"),
			Err(nom::Err::Error(make_error(
				"something",
				nom::error::ErrorKind::Tag
			)))
		);
	}

	#[test]
	fn can_parse_objects() {
		assert_eq!(
			Ok(("", JsonNode::Object(Box::new(IndexMap::new())))),
			parse_object("{}"),
		);
		assert_eq!(
			Ok((
				"",
				JsonNode::Object(Box::new(
					vec![("b", JsonNode::Boolean(false))].into_iter().collect()
				))
			)),
			parse_object("{\"b\": false}")
		);
		assert_eq!(
			Ok((
				"",
				JsonNode::Object(Box::new(
					vec![("a", JsonNode::String("x")), ("b", JsonNode::Boolean(true)),]
						.into_iter()
						.collect()
				))
			)),
			parse_object("{\"a\": \"x\", \"b\": true}")
		);
		assert_eq!(
			Ok((
				"",
				JsonNode::Object(Box::new(
					vec![("color", JsonNode::String("red")), ("value", JsonNode::String("#f00"))]
						.into_iter()
						.collect()
				))
			)),
			parse_object("{color:\"red\",value:\"#f00\"}")
		);
		assert_eq!(
			parse_object("something"),
			Err(nom::Err::Error(make_error(
				"something",
				nom::error::ErrorKind::Tag
			)))
		);
	}
}
