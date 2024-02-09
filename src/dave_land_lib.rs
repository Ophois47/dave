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
	Help,
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
			Command::Help => write!(f, "help"),
			Command::Unknown(_) => write!(f, "unknown"),
		}
	}
}

pub struct Object {
	pub labels: Vec<String>,
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

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum AmbiguousOption<T> {
	None,
	Some(T),
	Ambiguous,
}

const LOC_COMM_CENTER: usize = 0;
const LOC_ARMORY: usize = 1;
const LOC_LANDING_PAD: usize = 2;
const LOC_PLAYER: usize = 3;
// const LOC_PHOTO: usize = 4;
const LOC_PANTS: usize = 5;
// const LOC_RIFLE: usize = 6;
const LOC_COPILOT: usize = 7;
// const NORTH_TO_COMMAND_CENTER: usize = 8;
// const SOUTH_TO_LANDING_PAD: usize = 9;
// const WEST_TO_ARMORY: usize = 10;

pub struct World {
	pub objects: Vec<Object>,
}

impl World {
	pub fn new() -> Self {
		World {
			objects: vec![
				Object {
					labels: vec!["Command Center".into()],
					description: "the command center".into(),
					location: None,
					destination: None,
				},
				Object {
					labels: vec!["Armory".into()],
					description: "the armory".into(),
					location: None,
					destination: None,
				},
				Object {
					labels: vec!["Landing Pad".into()],
					description: "the landing pad".into(),
					location: None,
					destination: None,
				},
				Object {
					labels: vec!["Yourself".into()],
					description: "yourself".into(),
					location: Some(LOC_COMM_CENTER),
					destination: None,
				},
				Object {
					labels: vec!["Glossy Photo".into(), "Photo".into(), "Picture".into()],
					description: "a glossy picture of a family. They look familiar ...".into(),
					location: Some(LOC_COMM_CENTER),
					destination: None,
				},
				Object {
					labels: vec!["Wrinkled Photo".into(), "Photo".into(), "Picture".into()],
					description: "a wrinkled picture of a woman. She is crying".into(),
					location: Some(LOC_COPILOT),
					destination: None,
				},
				Object {
					labels: vec!["the M41A Pulse Rifle".into(), "Rifle".into()],
					description: "a single M41A Pulse Rifle with an ammo counter that says 56".into(),
					location: Some(LOC_ARMORY),
					destination: None,
				},
				Object {
					labels: vec!["Copilot".into()],
					description: "your copilot".into(),
					location: Some(LOC_LANDING_PAD),
					destination: None,
				},
				Object {
					labels: vec!["Pen".into()],
					description: "a pen".into(),
					location: Some(LOC_COPILOT),
					destination: None,
				},
				Object {
					labels: vec!["Tater".into(), "Tot".into(), "Tater Tot".into()],
					description: "a cold tater tot".into(),
					location: Some(LOC_PANTS),
					destination: None,
				},
				Object {
					labels: vec!["Camo Pants".into()],
					description: "a pair of black, white and grey camouflaged pants".into(),
					location: Some(LOC_ARMORY),
					destination: None,
				},
				Object {
					labels: vec!["South".into()],
					description: "a passage south to the armory".into(),
					location: Some(LOC_COMM_CENTER),
					destination: Some(LOC_ARMORY),
				},
				Object {
					labels: vec!["North".into()],
					description: "a passage north to the comm center".into(),
					location: Some(LOC_ARMORY),
					destination: Some(LOC_COMM_CENTER),
				},
				Object {
					labels: vec!["South".into()],
					description: "a passage south to the landing pad".into(),
					location: Some(LOC_ARMORY),
					destination: Some(LOC_LANDING_PAD),
				},
				Object {
					labels: vec!["North".into()],
					description: "a passage north to the armory".into(),
					location: Some(LOC_LANDING_PAD),
					destination: Some(LOC_ARMORY),
				},
				Object {
					labels: vec!["North".into(), "East".into(), "West".into()],
					description: "a bulkhead covered in blinking screens, switch-panels, gauges and communications technology".into(),
					location: Some(LOC_COMM_CENTER),
					destination: None,
				},
				Object {
					labels: vec!["East".into(), "West".into()],
					description: "an empty wall where many rifles were once stored and displayed".into(),
					location: Some(LOC_ARMORY),
					destination: None,
				},
				Object {
					labels: vec!["South".into(), "East".into(), "West".into()],
					description: "a large landing pad with a UD-4 'Cheyenne' Dropship nestled in the middle of it. The ramp is extended. Hopefully it flies".into(),
					location: Some(LOC_LANDING_PAD),
					destination: None,
				},
			],
		}
	}

	fn object_has_label(&self, object: &Object, noun: &str) -> bool {
		let mut result: bool = false;
		for (_, label) in object.labels.iter().enumerate() {
			if label.to_lowercase() == noun {
				result = true;
				break;
			}
		}
		result
	}

	fn get_object_index(&self, noun: &str, from: Option<usize>, max_distance: Distance) -> AmbiguousOption<usize> {
		let mut result: AmbiguousOption<usize> = AmbiguousOption::None;
		for (position, object) in self.objects.iter().enumerate() {
			if self.object_has_label(object, noun) && self.get_distance(from, Some(position)) <= max_distance {
				if result == AmbiguousOption::None {
					result = AmbiguousOption::Some(position);
				} else {
					result = AmbiguousOption::Ambiguous;
				}
			}
		}
		result
	}

	fn get_visible(&self, message: &str, noun: &str) -> (String, Option<usize>) {
		let obj_over_there = self.get_object_index(noun, Some(LOC_PLAYER), Distance::OverThere);
		let obj_not_here = self.get_object_index(noun, Some(LOC_PLAYER), Distance::NotHere);

		match (obj_over_there, obj_not_here) {
			(AmbiguousOption::None, AmbiguousOption::None) => (format!("I don't understand {}.\n", message), None),
			(AmbiguousOption::None, AmbiguousOption::Some(_)) => (format!("You don't see any '{}' here.\n", noun), None),
			(AmbiguousOption::Ambiguous, _) | (AmbiguousOption::None, AmbiguousOption::Ambiguous) => (
				format!("Please be more specific about which {} you mean.\n", noun),
				None,
			),
			(AmbiguousOption::Some(index), _) => (String::new(), Some(index)),
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
			Command::Help => format!(
				"You can:\n \
				Look - see what is around you. Try typing \"look around\"\n \
				Go - travel to a nearby location. Try typing \"go south\" or \"go north\"\n \
				Get - pickup an item. Try typing \"get <item name>\"\n \
				Give - give an item to someone or something else\n \
				Drop - drop an item currently in your inventory. Try typing \"drop <item name>\"\n \
				Inventory - Check your current inventory\n \
				Ask - speak with someone nearby. Try typing \"ask <entity>\"\n \
				Help - prints this screen\n \
				Quit - leave the game"
			),
			Command::Unknown(input_str) => format!("What do you mean by '{}'?", input_str),
		}
	}

	pub fn do_look(&self, noun: &str) -> String {
        match noun {
            "around" | "" => {
                let (list_string, _) = self.list_objects_at_location(self.objects[LOC_PLAYER].location.unwrap());
                format!(
                    "{}\nYou are in {}.\n",
                    self.objects[self.objects[LOC_PLAYER].location.unwrap()].labels[0],
                    self.objects[self.objects[LOC_PLAYER].location.unwrap()].description
                ) + list_string.as_str()
            }
            _ => format!("I don't understand what you want to look at.\n"),
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
					"You are not able to go further in that direction.\n".to_string()
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
			(Distance::OverThere, _) => output_vis + "Too far away, move closer.\n",
			(Distance::UnknownObject, _) => output_vis,
			_ => {
				let obj_loc = obj_opt.and_then(|a| self.objects[a].location);

				if obj_loc == Some(LOC_COPILOT) {
					output_vis + &format!("You should ask {} nicely.\n", self.objects[LOC_COPILOT].labels[0])
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
				format!("You drop {}.\n", self.objects[obj_opt_idx].labels[0])
			}
			(Some(obj_opt_idx), _, Some(to_idx), _) if to_idx != LOC_PLAYER => {
				if to_idx == LOC_COPILOT {
					format!("You give {} to {}.\n", self.objects[obj_opt_idx].labels[0], self.objects[to_idx].labels[0])
				} else {
					format!("You put {} in {}.\n", self.objects[obj_opt_idx].labels[0], self.objects[to_idx].labels[0])
				}
			}
			(Some(obj_opt_idx), Some(obj_loc_idx), _, Some(player_loc_idx)) if obj_loc_idx == player_loc_idx => {
				format!("You pick up {}.\n", self.objects[obj_opt_idx].labels[0])
			}
			(Some(obj_opt_idx), Some(obj_loc_idx), _, _) => format!(
				"You get {} from {}.\n",
				self.objects[obj_opt_idx].labels[0],
				self.objects[obj_loc_idx].labels[0],
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
            (Some(_), AmbiguousOption::None, AmbiguousOption::None) => (
                format!("I don't understand what you want to {}.\n", command),
                None,
            ),
            (Some(from_idx), AmbiguousOption::None, _) if from_idx == LOC_PLAYER => {
                (format!("You are not holding any {}.\n", noun), None)
            }
            (Some(from_idx), AmbiguousOption::None, _) => (
                format!("There appears to be no {} you can get from {}.\n", noun, self.objects[from_idx].labels[0]),
                None,
            ),
            (Some(from_idx), AmbiguousOption::Some(object_held_idx), _) if object_held_idx == from_idx => {
                (format!("You should not be doing that to {}.\n", self.objects[object_held_idx].labels[0]),
                    None,
                )
            }
            (Some(_), AmbiguousOption::Ambiguous, _) => (
                format!("Please be more specific about which {} you want to {}.\n", noun, command),
                None,
            ),
            (Some(_), AmbiguousOption::Some(object_held_idx), _) => {
                ("".to_string(), Some(object_held_idx))
            }
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

	fn get_passage_index(&self, from_opt: Option<usize>, to_opt: Option<usize>) -> Option<usize> {
        let mut result: Option<usize> = None;

        if from_opt.is_some() && to_opt.is_some() {
        	for (position, object) in self.objects.iter().enumerate() {
        		if self.is_holding(from_opt, Some(position)) && object.destination == to_opt {
        			result = Some(position);
        			break;
        		}
        	}
        	result
        } else {
        	result
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
    let mut split_input_iter = lc_input_str.split_whitespace();
 
    let verb = split_input_iter.next().unwrap_or_default().to_string();
    let noun = split_input_iter.fold("".to_string(), |accum, item| {
        if accum.is_empty() {
            accum + item
        } else {
            accum + " " + item
        }
    });
 
    match verb.as_str() {
        "ask" => Command::Ask(noun),
        "drop" => Command::Drop(noun),
        "get" => Command::Get(noun),
        "give" => Command::Give(noun),
        "go" => Command::Go(noun),
        "inventory" => Command::Inventory,
        "look" => Command::Look(noun),
        "help" => Command::Help,
        "quit" => Command::Quit,
        _ => Command::Unknown(input_str.trim().to_string()),
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
