use nalgebra::{
	distance,
	Point2,
	Rotation2,
	Vector2,
	wrap,
};
use rand::{
	Rng,
	RngCore,
};
use serde::{
	Deserialize,
	Serialize,
};
use std::f32::consts::*;
use std::fmt;
use crate::dave_genetic_algo::*;
use crate::dave_neural_net::*;

//
//	Animal
//
#[derive(Debug)]
pub struct Animal {
	position: Point2<f32>,
	rotation: Rotation2<f32>,
	vision: Vec<f32>,
	speed: f32,
	eye: Eye,
	brain: Brain,
	satiation: usize,
}

impl Animal {
	pub fn position(&self) -> Point2<f32> {
		self.position
	}

	pub fn rotation(&self) -> Rotation2<f32> {
		self.rotation
	}

	pub fn vision(&self) -> &[f32] {
		&self.vision
	}
}

impl Animal {
	fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
		let brain = Brain::random(config, rng);
		Self::new(config, rng, brain)
	}

	fn from_chromosome(
		config: &Config,
		rng: &mut dyn RngCore,
		chromosome: Chromosome,
	) -> Self {
		let brain = Brain::from_chromosome(config, chromosome);
		Self::new(config, rng, brain)
	}

	fn as_chromosome(&self) -> Chromosome {
		self.brain.as_chromosome()
	}

	fn process_brain(&mut self, config: &Config, foods: &[Food]) {
		self.vision = self.eye.process_vision(self.position, self.rotation, foods);
		let (speed, rotation) = self.brain.propagate(self.vision.clone());

		self.speed = (self.speed + speed).clamp(config.sim_speed_min, config.sim_speed_max);
		self.rotation = Rotation2::new(self.rotation.angle() + rotation);
	}

	fn process_movement(&mut self) {
		self.position += self.rotation * Vector2::new(0.0, self.speed);
		self.position.x = wrap(self.position.x, 0.0, 1.0);
		self.position.y = wrap(self.position.y, 0.0, 1.0);
	}
}

impl Animal {
	fn new(config: &Config, rng: &mut dyn RngCore, brain: Brain) -> Self {
		Self {
			position: rng.gen(),
			rotation: rng.gen(),
			vision: vec![0.0; config.eye_cells],
			speed: config.sim_speed_max,
			eye: Eye::new(config),
			brain,
			satiation: 0,
		}
	}
}

//
//	Animal Individual
//
struct AnimalIndividual {
	fitness: f32,
	chromosome: Chromosome,
}

impl AnimalIndividual {
	fn from_animal(animal: &Animal) -> Self {
		Self {
			fitness: animal.satiation as f32,
			chromosome: animal.as_chromosome(),
		}
	}

	fn into_animal(self, config: &Config, rng: &mut dyn RngCore) -> Animal {
		Animal::from_chromosome(config, rng, self.chromosome)
	}
}

impl Individual for AnimalIndividual {
	fn create(chromosome: Chromosome) -> Self {
		Self {
			fitness: 0.0,
			chromosome,
		}
	}

	fn chromosome(&self) -> &Chromosome {
		&self.chromosome
	}

	fn fitness(&self) -> f32 {
		self.fitness
	}
}

//
//	Brain
//
#[derive(Debug)]
struct Brain {
	speed_accel: f32,
	rotation_accel: f32,
	nn: Network,
}

impl Brain {
	fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
		let nn = Network::random(rng, &Self::topology(config));
		Self::new(config, nn)
	}

	fn from_chromosome(config: &Config, chromosome: Chromosome) -> Self {
		let nn = Network::from_weights(&Self::topology(config), chromosome.iter());
		Self::new(config, nn)
	}

	fn as_chromosome(&self) -> Chromosome {
		self.nn.weights().collect()
	}

	fn propagate(&self, vision: Vec<f32>) -> (f32, f32) {
		let response = self.nn.propagate(vision);
		let r0 = response[0].clamp(0.0, 1.0) - 0.5;
		let r1 = response[1].clamp(0.0, 1.0) - 0.5;
		let speed = (r0 + r1).clamp(-self.speed_accel, self.speed_accel);
		let rotation = (r0 - r1).clamp(-self.rotation_accel, self.rotation_accel);

		(speed, rotation)
	}
}

impl Brain {
	fn new(config: &Config, nn: Network) -> Self {
		Self {
			speed_accel: config.sim_speed_accel,
			rotation_accel: config.sim_rotation_accel,
			nn,
		}
	}

	fn topology(config: &Config) -> [LayerTopology; 3] {
		[
			LayerTopology {
				neurons: config.eye_cells,
			},
			LayerTopology {
				neurons: config.brain_neurons,
			},
			LayerTopology { neurons: 2 },
		]
	}
}

//
//	Config
//
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
	brain_neurons: usize,
	eye_fov_range: f32,
	eye_fov_angle: f32,
	eye_cells: usize,
	food_size: f32,
	ga_reverse: usize,
	ga_mut_chance: f32,
	ga_mut_coeff: f32,
	sim_speed_min: f32,
	sim_speed_max: f32,
	sim_speed_accel: f32,
	sim_rotation_accel: f32,
	sim_generation_length: usize,
	world_animals: usize,
	world_foods: usize,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			brain_neurons: 9,
            eye_fov_range: 0.25,
            eye_fov_angle: PI + FRAC_PI_4,
            eye_cells: 9,
            food_size: 0.01,
            ga_reverse: 0,
            ga_mut_chance: 0.01,
            ga_mut_coeff: 0.3,
            sim_speed_min: 0.001,
            sim_speed_max: 0.005,
            sim_speed_accel: 0.2,
            sim_rotation_accel: FRAC_PI_2,
            sim_generation_length: 2500,
            world_animals: 40,
            world_foods: 60,
		}
	}
}

//
//	Eye
//
#[derive(Debug)]
struct Eye {
	fov_range: f32,
	fov_angle: f32,
	cells: usize,
}

impl Eye {
	fn new(config: &Config) -> Self {
		Self::new_ex(config.eye_fov_range, config.eye_fov_angle, config.eye_cells)
	}

	fn process_vision(
		&self,
		position: Point2<f32>,
		rotation: Rotation2<f32>,
		foods: &[Food],
	) -> Vec<f32> {
		let mut cells = vec![0.0; self.cells];

		for food in foods {
			let vec = food.position - position;
			let dist = vec.norm();

			if dist > self.fov_range {
				continue;
			}

			let angle = Rotation2::rotation_between(&Vector2::y(), &vec).angle();
			let angle = angle - rotation.angle();
			let angle = wrap(angle, -PI, PI);

			if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
				continue;
			}

			let angle = angle + self.fov_angle / 2.0;
			let cell = angle / self.fov_angle * (self.cells as f32);
			let cell = (cell as usize).min(cells.len() - 1);

			cells[cell] += (self.fov_range - dist) / self.fov_range;
		}

		cells
	}
}

impl Eye {
	fn new_ex(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
		assert!(fov_range > 0.0);
		assert!(fov_angle > 0.0);
		assert!(cells > 0);

		Self {
			fov_range,
			fov_angle,
			cells,
		}
	}
}

//
//	Food
//
#[derive(Debug)]
pub struct Food {
	position: Point2<f32>,
}

impl Food {
	pub fn position(&self) -> Point2<f32> {
		self.position
	}
}

impl Food {
	fn random(rng: &mut dyn RngCore) -> Self {
		Self {
			position: rng.gen(),
		}
	}
}

//
//	Statistics
//
#[derive(Clone, Debug)]
pub struct Statistics {
	generation: usize,
	ga: crate::dave_genetic_algo::Statistics,
}

impl fmt::Display for Statistics {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "Generation {}:", self.generation)?;
		write!(
			f,
			"Min[{:.2}] Max[{:.2}] Avg[{:.2}] Median[{:.2}]",
			self.ga.min_fitness(),
			self.ga.max_fitness(),
			self.ga.avg_fitness(),
			self.ga.median_fitness(),
		)
	}
}

//
//	World
//
#[derive(Debug)]
pub struct World {
	animals: Vec<Animal>,
	foods: Vec<Food>,
}

impl World {
	pub fn animals(&self) -> &[Animal] {
		&self.animals
	}

	pub fn foods(&self) -> &[Food] {
		&self.foods
	}
}

impl World {
	fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
		let animals = (0..config.world_animals)
			.map(|_| Animal::random(config, rng))
			.collect();

		let foods = (0..config.world_foods).map(|_| Food::random(rng)).collect();

		Self { animals, foods }
	}
}

//
//	Simulation
//
pub struct Simulation {
	config: Config,
	world: World,
	age: usize,
	generation: usize,
}

impl Simulation {
	pub fn random(config: Config, rng: &mut dyn RngCore) -> Self {
		let world = World::random(&config, rng);
		Self {
			config,
			world,
			age: 0,
			generation: 0,
		}
	}

	pub fn config(&self) -> &Config {
		&self.config
	}

	pub fn world(&self) -> &World {
		&self.world
	}

	pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
		self.process_collisions(rng);
		self.process_brains();
		self.process_movements();
		self.try_evolving(rng)
	}

	pub fn train(&mut self, rng: &mut dyn RngCore) -> Statistics {
		loop {
			if let Some(statistics) = self.step(rng) {
				return statistics
			}
		}
	}
}

impl Simulation {
	fn process_collisions(&mut self, rng: &mut dyn RngCore) {
		for animal in &mut self.world.animals {
			for food in &mut self.world.foods {
				let distance = distance(&animal.position, &food.position);
				if distance <= self.config.food_size {
					animal.satiation += 1;
					food.position = rng.gen();
				}
			}
		}
	}

	fn process_brains(&mut self) {
		for animal in &mut self.world.animals {
			animal.process_brain(&self.config, &self.world.foods);
		}
	}

	fn process_movements(&mut self) {
		for animal in &mut self.world.animals {
			animal.process_movement();
		}
	}

	fn try_evolving(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
		self.age += 1;
		if self.age > self.config.sim_generation_length {
			Some(self.evolve(rng))
		} else {
			None
		}
	}

	fn evolve(&mut self, rng: &mut dyn RngCore) -> Statistics {
		self.age = 0;
		self.generation += 1;

		let mut individuals: Vec<_> = self
			.world
			.animals
			.iter()
			.map(AnimalIndividual::from_animal)
			.collect();

		if self.config.ga_reverse == 1 {
			let max_satiation = self
				.world
				.animals
				.iter()
				.map(|animal| animal.satiation)
				.max()
				.unwrap_or_default();

			for individual in &mut individuals {
				individual.fitness = (max_satiation as f32) - individual.fitness;
			}
		}

		let ga = GeneticAlgorithm::new(
			RouletteWheelSelection,
			UniformCrossover,
			GaussianMutation::new(self.config.ga_mut_chance, self.config.ga_mut_coeff),
		);

		let (individuals, statistics) = ga.evolve(rng, &individuals);

		self.world.animals = individuals
			.into_iter()
			.map(|i| i.into_animal(&self.config, rng))
			.collect();

		for food in &mut self.world.foods {
			food.position = rng.gen();
		}

		Statistics {
			generation: self.generation - 1,
			ga: statistics,
		}
	}
}

//
//	Tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use test_case::test_case;

    const TEST_EYE_CELLS: usize = 13;

    fn food(x: f32, y: f32) -> Food {
        Food {
            position: Point2::new(x, y),
        }
    }

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new_ex(self.fov_range, self.fov_angle, TEST_EYE_CELLS);

            let actual = eye.process_vision(
                Point2::new(self.x, self.y),
                Rotation2::new(self.rot),
                &self.foods,
            );

            let actual = actual
                .into_iter()
                .map(|cell| {
                    if cell >= 0.7 {
                        "#"
                    } else if cell >= 0.3 {
                        "+"
                    } else if cell > 0.0 {
                        "."
                    } else {
                        " "
                    }
                })
                .collect::<Vec<_>>()
                .join("");

            assert_eq!(actual, self.expected);
        }
    }

    #[test_case(1.0, "      +      ")]
    #[test_case(0.9, "      +      ")]
    #[test_case(0.8, "      +      ")]
    #[test_case(0.7, "      .      ")]
    #[test_case(0.6, "      .      ")]
    #[test_case(0.5, "             ")]
    #[test_case(0.4, "             ")]
    #[test_case(0.3, "             ")]
    #[test_case(0.2, "             ")]
    #[test_case(0.1, "             ")]
    fn different_fov_ranges(fov_range: f32, expected: &'static str) {
        TestCase {
            foods: vec![food(0.5, 1.0)],
            fov_angle: FRAC_PI_2,
            x: 0.5,
            y: 0.5,
            rot: 0.0,
            fov_range,
            expected,
        }
        .run()
    }

    #[test_case(0.25 * PI, " +         + ")]
    #[test_case(0.50 * PI, ".  +     +  .")]
    #[test_case(0.75 * PI, "  . +   + .  ")]
    #[test_case(1.00 * PI, "   . + + .   ")]
    #[test_case(1.25 * PI, "   . + + .   ")]
    #[test_case(1.50 * PI, ".   .+ +.   .")]
    #[test_case(1.75 * PI, ".   .+ +.   .")]
    #[test_case(2.00 * PI, "+.  .+ +.  .+")]
    fn different_fov_angles(fov_angle: f32, expected: &'static str) {
        TestCase {
            foods: vec![
                food(0.0, 0.0),
                food(0.0, 0.33),
                food(0.0, 0.66),
                food(0.0, 1.0),
                food(1.0, 0.0),
                food(1.0, 0.33),
                food(1.0, 0.66),
                food(1.0, 1.0),
            ],
            fov_range: 1.0,
            x: 0.5,
            y: 0.5,
            rot: 3.0 * FRAC_PI_2,
            fov_angle,
            expected,
        }
        .run()
    }

    // Checking the X axis:
    #[test_case(0.9, 0.5, "#           #")]
    #[test_case(0.8, 0.5, "  #       #  ")]
    #[test_case(0.7, 0.5, "   +     +   ")]
    #[test_case(0.6, 0.5, "    +   +    ")]
    #[test_case(0.5, 0.5, "    +   +    ")]
    #[test_case(0.4, 0.5, "     + +     ")]
    #[test_case(0.3, 0.5, "     . .     ")]
    #[test_case(0.2, 0.5, "     . .     ")]
    #[test_case(0.1, 0.5, "     . .     ")]
    #[test_case(0.0, 0.5, "             ")]
    //
    // Checking the Y axis:
    #[test_case(0.5, 0.0, "            +")]
    #[test_case(0.5, 0.1, "          + .")]
    #[test_case(0.5, 0.2, "         +  +")]
    #[test_case(0.5, 0.3, "        + +  ")]
    #[test_case(0.5, 0.4, "      +  +   ")]
    #[test_case(0.5, 0.6, "   +  +      ")]
    #[test_case(0.5, 0.7, "  + +        ")]
    #[test_case(0.5, 0.8, "+  +         ")]
    #[test_case(0.5, 0.9, ". +          ")]
    #[test_case(0.5, 1.0, "+            ")]
    fn different_positions(x: f32, y: f32, expected: &'static str) {
        TestCase {
            foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
            fov_range: 1.0,
            fov_angle: FRAC_PI_2,
            rot: 3.0 * FRAC_PI_2,
            x,
            y,
            expected,
        }
        .run()
    }

    #[test_case(0.00 * PI, "         +   ")]
    #[test_case(0.25 * PI, "        +    ")]
    #[test_case(0.50 * PI, "      +      ")]
    #[test_case(0.75 * PI, "    +        ")]
    #[test_case(1.00 * PI, "   +         ")]
    #[test_case(1.25 * PI, " +           ")]
    #[test_case(1.50 * PI, "            +")]
    #[test_case(1.75 * PI, "           + ")]
    #[test_case(2.00 * PI, "         +   ")]
    #[test_case(2.25 * PI, "        +    ")]
    #[test_case(2.50 * PI, "      +      ")]
    fn different_rotations(rot: f32, expected: &'static str) {
        TestCase {
            foods: vec![food(0.0, 0.5)],
            fov_range: 1.0,
            fov_angle: 2.0 * PI,
            x: 0.5,
            y: 0.5,
            rot,
            expected,
        }
        .run()
    }

    #[test]
    #[ignore]
    fn simulation_test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut sim = Simulation::random(Default::default(), &mut rng);

        let avg_fitness = (0..10)
            .map(|_| sim.train(&mut rng).ga.avg_fitness())
            .sum::<f32>()
            / 10.0;

        approx::assert_relative_eq!(31.944998, avg_fitness);
    }
}
