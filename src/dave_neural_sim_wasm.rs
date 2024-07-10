use rand::prelude::*;
use wasm_bindgen::{
	JsValue,
	prelude::wasm_bindgen,
};
use crate::dave_neural_sim;

//
//	Animal
//
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Animal {
	pub x: f32,
	pub y: f32,
	pub rotation: f32,
	#[wasm_bindgen(getter_with_clone)]
	pub vision: Vec<f32>,
}

impl From<&dave_neural_sim::Animal> for Animal {
	fn from(animal: &dave_neural_sim::Animal) -> Self {
		Self {
			x: animal.position().x,
			y: animal.position().y,
			rotation: animal.rotation().angle(),
			vision: animal.vision().to_owned(),
		}
	}
}

//
//	Food
//
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Food {
	pub x: f32,
	pub y: f32,
}

impl From<&dave_neural_sim::Food> for Food {
	fn from(food: &dave_neural_sim::Food) -> Self {
		Self {
			x: food.position().x,
			y: food.position().y,
		}
	}
}

//
//	World
//
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
	#[wasm_bindgen(getter_with_clone)]
	pub animals: Vec<Animal>,
	#[wasm_bindgen(getter_with_clone)]
	pub foods: Vec<Food>,
}

impl From<&dave_neural_sim::World> for World {
	fn from(world: &dave_neural_sim::World) -> Self {
		let animals = world.animals().iter().map(Animal::from).collect();
		let foods = world.foods().iter().map(Food::from).collect();

		Self { animals, foods }
	}
}

//
//	Simulation
//
#[wasm_bindgen]
pub struct Simulation {
	rng: ThreadRng,
	sim: dave_neural_sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
	#[wasm_bindgen(constructor)]
	pub fn new(config: JsValue) -> Self {
		let config: dave_neural_sim::Config = serde_wasm_bindgen::from_value(config).unwrap();
		let mut rng = thread_rng();
		let sim = dave_neural_sim::Simulation::random(config, &mut rng);

		Self { rng, sim }
	}

	pub fn default_config() -> JsValue {
		serde_wasm_bindgen::to_value(&dave_neural_sim::Config::default()).unwrap()
	}

	pub fn config(&self) -> JsValue {
		serde_wasm_bindgen::to_value(self.sim.config()).unwrap()
	}

	pub fn world(&self) -> World {
		World::from(self.sim.world())
	}

	pub fn step(&mut self) -> Option<String> {
		self.sim.step(&mut self.rng).map(|stats| stats.to_string())
	}

	pub fn train(&mut self) -> String {
		self.sim.train(&mut self.rng).to_string()
	}
}
