use std::fmt;
use std::io::{self, Write};

pub enum Command {
	Ask(String),
	Drop(String),
	Get(String),
	Give(String),
	Go(String),
	Inventory,
	Look(String),
	Quit,
	Unknown(String),
}

impl fmt::Display for Command {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Command::Ask(_) => write!(f, "ask"),
			Command::Drop(_) => write!(f, "drop"),
			Command::Get(_) => write!(f, "get"),
			Command::Give(_) => write!(f, "give"),
			Command::Go(_) => write!(f, "go"),
			Command::Inventory => write!(f, "inventory"),
			Command::Look(_) => write!(f, "look"),
			Command::Quit => write!(f, "quit"),
			Command::Unknown(_) => write!(f, "unknown"),
		}
	}
}

pub struct Object {
	pub name: String,
	pub description: String,
	pub location: Option<usize>,
	pub destination: Option<usize>,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum Distance {
	Me,
	Held,
	HeldContained,
	Location,
	Here,
	HereContained,
	OverThere,
	NotHere,
	UnknownObject,
}

const LOC_COM_CENTER: usize = 0;
const LOC_MESS_HALL: usize = 1;
const LOC_LANDING_PAD: usize = 2;
const LOC_ARMORY: usize = 3;
const LOC_PLAYER: usize = 4;
// const LOC_PHOTO: usize = 5;
const LOC_PANTS: usize = 6;
// const LOC_RIFLE: usize = 7;
const LOC_COPILOT: usize = 8;
// const NORTH_TO_COMMAND_CENTER: usize = 9;
// const SOUTH_TO_ARMORY: usize = 10;
// const EAST_TO_MESS_HALL: usize = 11;
// const WEST_TO_LANDING_PAD: usize = 12;
const LOC_YOUR_ROOM: usize = 13;

pub struct World {
	pub objects: Vec<Object>,
}

impl World {
	pub fn new() -> Self {
		World {
			objects: vec![
				Object {
					name: "Command Center".to_string(),
					description: "the command center".to_string(),
					location: None,
					destination: None,
				},
				Object {
					name: "Mess Hall".to_string(),
					description: "the mess hall".to_string(),
					location: None,
					destination: None,
				},
				Object {
					name: "Landing Pad".to_string(),
					description: "the landing pad".to_string(),
					location: None,
					destination: None,
				},
				Object {
					name: "Armory".to_string(),
					description: "the armory".to_string(),
					location: None,
					destination: None,
				},
				Object {
					name: "Your Room".to_string(),
					description: "your room".to_string(),
					location: None,
					destination: None,
				},
				Object {
					name: "Yourself".to_string(),
					description: "yourself".to_string(),
					location: Some(LOC_COM_CENTER),
					destination: None,
				},
				Object {
					name: "Photo".to_string(),
					description: "a picture of a family. They look familiar ...".to_string(),
					location: Some(LOC_COM_CENTER),
					destination: None,
				},
				Object {
					name: "Rifle".to_string(),
					description: "the M41A Pulse Rifle".to_string(),
					location: Some(LOC_ARMORY),
					destination: None,
				},
				Object {
					name: "Copilot".to_string(),
					description: "your copilot, dead on the floor".to_string(),
					location: Some(LOC_MESS_HALL),
					destination: None,
				},
				Object {
					name: "Pen".to_string(),
					description: "a pen".to_string(),
					location: Some(LOC_COPILOT),
					destination: None,
				},
				Object {
					name: "Tater Tot".to_string(),
					description: "a cold tater tot".to_string(),
					location: Some(LOC_PANTS),
					destination: None,
				},
				Object {
					name: "North".to_string(),
					description: "north leads to the command center".to_string(),
					location: Some(LOC_YOUR_ROOM),
					destination: Some(LOC_COM_CENTER),
				},
				Object {
					name: "South".to_string(),
					description: "south leads to the armory".to_string(),
					location: Some(LOC_COM_CENTER),
					destination: Some(LOC_ARMORY),
				},
				Object {
					name: "East".to_string(),
					description: "east leads to the mess hall".to_string(),
					location: Some(LOC_COM_CENTER),
					destination: Some(LOC_MESS_HALL),
				},
				Object {
					name: "West".to_string(),
					description: "west leads to the landing pad".to_string(),
					location: Some(LOC_COM_CENTER),
					destination: Some(LOC_LANDING_PAD),
				},
			],
		}
	}

	fn object_has_name(&self, object: &Object, noun: &str) -> bool {
		*noun == object.name.to_lowercase()
	}

	fn get_object_index(&self, noun: &str, from: Option<usize>, max_distance: Distance) -> Option<usize> {
		let mut result: Option<usize> = None;
		for (position, object) in self.objects.iter().enumerate() {
			if self.object_has_name(&object, noun) && self.get_distance(from, Some(position)) <= max_distance {
				result = Some(position);
				break;
			}
		}
		result
	}

	fn get_visible(&self, message: &str, noun: &str) -> (String, Option<usize>) {
		let obj_over_there = self.get_object_index(noun, Some(LOC_PLAYER), Distance::OverThere);
		let obj_not_here = self.get_object_index(noun, Some(LOC_PLAYER), Distance::NotHere);

		match (obj_over_there, obj_not_here) {
			(None, None) => (format!("I don't understand {}.\n", message), None),
			(None, Some(_)) => (format!("You don't see any '{}' here.\n", noun), None),
			_ => (String::new(), obj_over_there),
		}
	}

	pub fn list_objects_at_location(&self, location: usize) -> (String, i32) {
		let mut output = String::new();
		let mut count: i32 = 0;

		for (position, object) in self.objects.iter().enumerate() {
			if position != LOC_PLAYER && self.is_holding(Some(location), Some(position)) {
				if count == 0 {
					output += "You see:\n";
				}
				count += 1;
				output = output + &format!("{}\n", object.description);
			}
		}
		(output, count)
	}

	pub fn update_state(&mut self, command: &Command) -> String {
		match command {
			Command::Ask(noun) => self.do_ask(noun),
			Command::Drop(noun) => self.do_drop(noun),
			Command::Get(noun) => self.do_get(noun),
			Command::Give(noun) => self.do_give(noun),
			Command::Go(noun) => self.do_go(noun),
			Command::Inventory => self.do_inventory(),
			Command::Look(noun) => self.do_look(noun),
			Command::Quit => format!("Quitting ...\nThank you for playing!"),
			Command::Unknown(input_str) => format!("I don't even know how to '{}'.", input_str),
		}
	}

	pub fn do_look(&self, noun: &str) -> String {
        match noun {
            "around" | "" => {
                let (list_string, _) = self.list_objects_at_location(self.objects[LOC_PLAYER].location.unwrap());
                format!(
                    "{}\nYou are in {}.\n",
                    self.objects[self.objects[LOC_PLAYER].location.unwrap()].name,
                    self.objects[self.objects[LOC_PLAYER].location.unwrap()].description
                ) + list_string.as_str()
            }
            _ => format!("I don't understand what you want to see.\n"),
        }
    }

	pub fn do_go(&mut self, noun: &str) -> String {
		let (output_vis, obj_opt) = self.get_visible("where you want to go", noun);

		match self.get_distance(Some(LOC_PLAYER), obj_opt) {
			Distance::OverThere => {
				self.objects[LOC_PLAYER].location = obj_opt;
				"OK.\n\n".to_string() + &self.do_look("around")
			}
			Distance::NotHere => {
				format!("You don't see any {} here.\n", noun)
			}
			Distance::UnknownObject => output_vis,
			_ => {
				let obj_dst = obj_opt.and_then(|a| self.objects[a].destination);
				if obj_dst.is_some() {
					self.objects[LOC_PLAYER].location = obj_dst;
					"OK.\n\n".to_string() + &self.do_look("around")
				} else {
					"You can't get much closer than this.\n".to_string()
				}
			}
		}
	}

	pub fn do_ask(&mut self, noun: &String) -> String {
		let actor_loc = self.actor_here();
		let (output, object_idx) = self.get_possession(actor_loc, Command::Ask("ask".to_string()), noun);
		output + self.move_object(object_idx, Some(LOC_PLAYER)).as_str()
	}

	pub fn do_give(&mut self, noun: &String) -> String {
		let actor_loc = self.actor_here();
		let (output, object_idx) = self.get_possession(Some(LOC_PLAYER), Command::Give("give".to_string()), noun);
		output + self.move_object(object_idx, actor_loc).as_str()
	}

	pub fn do_drop(&mut self, noun: &String) -> String {
		let (output, object_idx) = self.get_possession(Some(LOC_PLAYER), Command::Drop("drop".to_string()), noun);
		let player_loc = self.objects[LOC_PLAYER].location;

		output + self.move_object(object_idx, player_loc).as_str()
	}

	pub fn do_get(&mut self, noun: &String) -> String {
		let (output_vis, obj_opt) = self.get_visible("where do you want to go", noun);
		let player_to_obj = self.get_distance(Some(LOC_PLAYER), obj_opt);

		match (player_to_obj, obj_opt) {
			(Distance::Me, _) => output_vis + "You should not be doing that to yourself.\n",
			(Distance::Held, Some(object_idx)) => {
				output_vis + &format!("You already have {}.\n", self.objects[object_idx].description)
			}
			(Distance::OverThere, _) => output_vis + "Too far away, move closer please.\n",
			(Distance::UnknownObject, _) => output_vis,
			_ => {
				let obj_loc = obj_opt.and_then(|a| self.objects[a].location);

				if obj_loc == Some(LOC_COPILOT) {
					output_vis + &format!("You should ask {} nicely.\n", self.objects[LOC_COPILOT].name)
				} else {
					self.move_object(obj_opt, Some(LOC_PLAYER))
				}
			}
		}
	}

	pub fn do_inventory(&self) -> String {
		let (list_string, count) = self.list_objects_at_location(LOC_PLAYER);
		if count == 0 {
			format!("You have nothing in your inventory.\n")
		} else {
			list_string
		}
	}

	pub fn describe_move(&self, obj_opt: Option<usize>, to: Option<usize>) -> String {
		let obj_loc = obj_opt.and_then(|a| self.objects[a].location);
		let player_loc = self.objects[LOC_PLAYER].location;

		match (obj_opt, obj_loc, to, player_loc) {
			(Some(obj_opt_idx), _, Some(to_idx), Some(player_loc_idx)) if to_idx == player_loc_idx => {
				format!("You drop {}.\n", self.objects[obj_opt_idx].name)
			}
			(Some(obj_opt_idx), _, Some(to_idx), _) if to_idx != LOC_PLAYER => {
				if to_idx == LOC_COPILOT {
					format!("You give {} to {}.\n", self.objects[obj_opt_idx].name, self.objects[to_idx].name)
				} else {
					format!("You put {} in {}.\n", self.objects[obj_opt_idx].name, self.objects[to_idx].name)
				}
			}
			(Some(obj_opt_idx), Some(obj_loc_idx), _, Some(player_loc_idx)) if obj_loc_idx == player_loc_idx => {
				format!("You pick up {}.\n", self.objects[obj_opt_idx].name)
			}
			(Some(obj_opt_idx), Some(obj_loc_idx), _, _) => format!(
				"You get {} from {}.\n",
				self.objects[obj_opt_idx].name,
				self.objects[obj_loc_idx].name,
			),
			// This arm should never be hit
			(None, _, _, _) | (_, None, _, _) => format!("How can you drop nothing?\n"),
		}
	}

	pub fn move_object(&mut self, obj_opt: Option<usize>, to: Option<usize>) -> String {
		let obj_loc = obj_opt.and_then(|a| self.objects[a].location);

		match (obj_opt, obj_loc, to) {
			(None, _, _) => format!(""),
			(Some(_), _, None) => format!("There is nobody to give that to.\n"),
			(Some(_), None, Some(_)) => format!("This is way too heavy.\n"),
			(Some(obj_idx), Some(_), Some(to_idx)) => {
				let output = self.describe_move(obj_opt, to);
				self.objects[obj_idx].location = Some(to_idx);
				output
			}
		}
	}

	pub fn get_possession(
		&mut self,
		from: Option<usize>,
		command: Command,
		noun: &String,
	) -> (String, Option<usize>) {
		let object_held = self.get_object_index(noun, from, Distance::HeldContained);
		let object_not_here = self.get_object_index(noun, from, Distance::NotHere);

		match (from, object_held, object_not_here) {
            (None, _, _) => (
                format!("I don't understand what you want to {}.\n", command),
                None,
            ),
            (Some(_), None, None) => (
                format!("I don't understand what you want to {}.\n", command),
                None,
            ),
            (Some(from_idx), None, Some(_)) if from_idx == LOC_PLAYER => {
                (format!("You are not holding any {}.\n", noun), None)
            }
            (Some(from_idx), None, Some(_)) => (
                format!("There appears to be no {} you can get from {}.\n", noun, self.objects[from_idx].name),
                None,
            ),
            (Some(from_idx), Some(object_held_idx), _) if object_held_idx == from_idx => (
                format!("You should not be doing that to {}.\n", self.objects[object_held_idx].name),
                None,
            ),
            _ => ("".to_string(), object_held),
        }
	}

	pub fn actor_here(&self) -> Option<usize> {
		let mut actor_loc: Option<usize> = None;

		for (position, _) in self.objects.iter().enumerate() {
			if self.is_holding(self.objects[LOC_PLAYER].location, Some(position)) && position == LOC_COPILOT {
				actor_loc = Some(position);
			}
		}
		actor_loc
	}

	fn get_passage_index(&self, from: Option<usize>, to: Option<usize>) -> Option<usize> {
        let mut result: Option<usize> = None;

        match (from, to) {
            (Some(from), Some(to)) => {
                for (pos, object) in self.objects.iter().enumerate() {
                    let obj_loc = object.location;
                    let obj_dest = object.destination;
                    match (obj_loc, obj_dest) {
                        (Some(location), Some(destination)) if location == from && destination == to => {
                            result = Some(pos);
                            break;
                        }
                        _ => continue,
                    }
                }
                result
            }
            _ => result,
        }
    }

    pub fn is_holding(&self, container: Option<usize>, object: Option<usize>) -> bool {
    	object.is_some() && (object.and_then(|a| self.objects[a].location) == container)
    }

    pub fn get_distance(&self, from: Option<usize>, to: Option<usize>) -> Distance {
    	let from_loc = from.and_then(|a| self.objects[a].location);
    	let to_loc = to.and_then(|a| self.objects[a].location);

    	if to.is_none() {
    		Distance::UnknownObject
    	} else if to == from {
    		Distance::Me
    	} else if self.is_holding(from, to) {
    		Distance::Held
    	} else if self.is_holding(to, from) {
    		Distance::Location
    	} else if from_loc.is_some() && self.is_holding(from_loc, to) {
    		Distance::Here
    	} else if self.is_holding(from, to_loc) {
    		Distance::HeldContained
    	} else if self.is_holding(to_loc, from) {
    		Distance::HereContained
    	} else if self.get_passage_index(from_loc, to).is_some() {
    		Distance::OverThere
    	} else {
    		Distance::NotHere
    	}
    }
}

pub fn parse(input_str: String) -> Command {
	let lc_input_str = input_str.to_lowercase();
	let mut split_input_iter = lc_input_str.trim().split_whitespace();

	let verb = split_input_iter.next().unwrap_or_default().to_string();
	let noun = split_input_iter.next().unwrap_or_default().to_string();

	match verb.as_str() {
		"ask"  		=> Command::Ask(noun),
		"drop" 		=> Command::Drop(noun),
		"get"  		=> Command::Get(noun),
		"give" 		=> Command::Give(noun),
		"go"   		=> Command::Go(noun),
		"inventory" => Command::Inventory,
		"look" 		=> Command::Look(noun),
		"quit" 		=> Command::Quit,
		_      		=> Command::Unknown(input_str.trim().to_string()),
	}
}

pub fn get_input() -> Command {
	// Prompt
	println!("");
	print!("> ");
	io::stdout().flush().unwrap();

	let mut input_str = String::new();

	io::stdin()
		.read_line(&mut input_str)
		.expect("Failed to Read Your Command");
	println!("");

	// Parse and Return
	parse(input_str)
}

pub fn update_screen(output: String) {
	println!("{}", output);
}
