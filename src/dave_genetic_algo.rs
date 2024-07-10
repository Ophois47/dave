use std::cmp::Ordering;
use std::iter::FromIterator;
use std::ops::Index;
use rand::{
	Rng,
	RngCore,
	seq::SliceRandom,
};

//
//	Chromosome
//
#[derive(Clone, Debug)]
pub struct Chromosome {
	genes: Vec<f32>,
}

impl Chromosome {
	fn len(&self) -> usize {
		self.genes.len()
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn iter(&self) -> impl Iterator<Item = f32> + '_ {
		self.genes.iter().copied()
	}

	fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
		self.genes.iter_mut()
	}
}

impl Index<usize> for Chromosome {
	type Output = f32;
	fn index(&self, index: usize) -> &Self::Output {
		&self.genes[index]
	}
}

impl FromIterator<f32> for Chromosome {
	fn from_iter<T>(iter: T) -> Self
	where
		T: IntoIterator<Item = f32>,
	{
		Self {
			genes: iter.into_iter().collect(),
		}
	}
}

//
//	Crossover
//
#[derive(Clone, Debug, Default)]
pub struct UniformCrossover;

impl CrossoverMethod for UniformCrossover {
	fn crossover(
		&self,
		rng: &mut dyn RngCore,
		parent_a: &Chromosome,
		parent_b: &Chromosome,
	) -> Chromosome {
		assert_eq!(parent_a.len(), parent_b.len());

		let parent_a = parent_a.iter();
		let parent_b = parent_b.iter();

		parent_a
			.zip(parent_b)
			.map(|(a, b)| if rng.gen_bool(0.5) { a } else { b })
			.collect()
	}
}

pub trait CrossoverMethod {
	fn crossover(&self, rng: &mut dyn RngCore, parent_a: &Chromosome, parent_b: &Chromosome) -> Chromosome;
}

//
//	Gaussian Mutation
//
#[derive(Clone, Debug)]
pub struct GaussianMutation {
	chance: f32,
	coeff: f32,
}

impl GaussianMutation {
	pub fn new(chance: f32, coeff: f32) -> Self {
		assert!((0.0..=1.0).contains(&chance));

		Self { chance, coeff }
	}
}

impl MutationMethod for GaussianMutation {
	fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
		for gene in child.iter_mut() {
			let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };
			if rng.gen_bool(self.chance as _) {
				*gene += sign * self.coeff * rng.gen::<f32>();
			}
		}
	}
}

pub trait MutationMethod {
	fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}

//
//	Selection Roulette Wheel
//
#[derive(Clone, Debug, Default)]
pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
	fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
	where
		I: Individual,
	{
		population
			.choose_weighted(rng, |individual| individual.fitness().max(0.00001))
			.expect("Got an empty population")
	}
}

pub trait SelectionMethod {
	fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
	where
		I: Individual;
}

//
//	Individual
//
pub trait Individual {
	fn create(chromosome: Chromosome) -> Self;
	fn chromosome(&self) -> &Chromosome;
	fn fitness(&self) -> f32;
}
//
//	Statistics
//
#[derive(Clone, Debug)]
pub struct Statistics {
	min_fitness: f32,
	max_fitness: f32,
	avg_fitness: f32,
	median_fitness: f32,
}

impl Statistics {
	pub fn new<I>(population: &[I]) -> Self
	where
		I: Individual,
	{
		assert!(!population.is_empty());

		let len = population.len();
		let fitnesses = {
			let mut fitnesses: Vec<_> = population.iter().map(|i| i.fitness()).collect();
			fitnesses.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
			fitnesses
		};

		let min_fitness = fitnesses[0];
		let max_fitness = fitnesses[len - 1];
		let avg_fitness = fitnesses.iter().sum::<f32>() / (len as f32);

		let median_fitness = if len % 2 == 0 {
			(fitnesses[len / 2 - 1] + fitnesses[len / 2]) / 2.0
		} else {
			fitnesses[len / 2]
		};

		Self {
			min_fitness,
			max_fitness,
			avg_fitness,
			median_fitness,
		}
	}

	pub fn min_fitness(&self) -> f32 {
		self.min_fitness
	}

	pub fn max_fitness(&self) -> f32 {
		self.max_fitness
	}

	pub fn avg_fitness(&self) -> f32 {
		self.avg_fitness
	}

	pub fn median_fitness(&self) -> f32 {
		self.median_fitness
	}
}

//
//	Genetic Algorithm
//
pub struct GeneticAlgorithm<S> {
	selection_method: S,
	crossover_method: Box<dyn CrossoverMethod>,
	mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
	S: SelectionMethod,
{
	pub fn new(
		selection_method: S,
		crossover_method: impl CrossoverMethod + 'static,
		mutation_method: impl MutationMethod + 'static,
	) -> Self {
		Self {
			selection_method,
			crossover_method: Box::new(crossover_method),
			mutation_method: Box::new(mutation_method),
		}
	}

	pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> (Vec<I>, Statistics)
	where
		I: Individual,
	{
		assert!(!population.is_empty());

		let new_population = (0..population.len())
			.map(|_| {
				let parent_a = self.selection_method.select(rng, population).chromosome();
				let parent_b = self.selection_method.select(rng, population).chromosome();
				let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

				self.mutation_method.mutate(rng, &mut child);
				I::create(child)
			})
			.collect();

		(new_population, Statistics::new(population))
	}
}

//
//	Tests
//
#[cfg(test)]
impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        approx::relative_eq!(self.genes.as_slice(), other.genes.as_slice())
    }
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub enum TestIndividual {
    WithChromosome { chromosome: Chromosome },
    WithFitness { fitness: f32 },
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self::WithFitness { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn create(chromosome: Chromosome) -> Self {
        Self::WithChromosome { chromosome }
    }

    fn chromosome(&self) -> &Chromosome {
        match self {
            Self::WithChromosome { chromosome } => chromosome,
            Self::WithFitness { .. } => panic!("not supported for TestIndividual::WithFitness"),
        }
    }

    fn fitness(&self) -> f32 {
        match self {
            Self::WithChromosome { chromosome } => chromosome.iter().sum(),
            Self::WithFitness { fitness } => *fitness,
        }
    }
}

#[cfg(test)]
mod tests {
	use super::*;
    use std::collections::BTreeMap;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn crossover_test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a: Chromosome = (1..=100).map(|n| n as f32).collect();
        let parent_b: Chromosome = (1..=100).map(|n| -n as f32).collect();

        let child = UniformCrossover.crossover(&mut rng, &parent_a, &parent_b);

        // Number of genes different between `child` and `parent_a`
        let diff_a = child
            .iter()
            .zip(parent_a.iter())
            .filter(|(c, p)| c != p)
            .count();

        // Number of genes different between `child` and `parent_b`
        let diff_b = child
            .iter()
            .zip(parent_b.iter())
            .filter(|(c, p)| c != p)
            .count();

        // Roughly looks like 50%, which proves that chance for picking either
        // gene is 50%
        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }

    fn actual(chance: f32, coeff: f32) -> Vec<f32> {
        let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);

        child.iter().collect()
    }

    mod given_zero_chance {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.0, coeff)
        }

        mod and_zero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.5);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_fifty_fifty_chance {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.5, coeff)
        }

        mod and_zero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            use super::*;

            #[test]
            fn slightly_changes_the_original_chromosome() {
                let actual = actual(0.5);
                let expected = vec![1.0, 1.7756249, 3.0, 4.1596804, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_max_chance {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(1.0, coeff)
        }

        mod and_zero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            use super::*;

            #[test]
            fn entirely_changes_the_original_chromosome() {
                let actual = actual(0.5);
                let expected = vec![1.4545316, 2.1162078, 2.7756248, 3.9505124, 4.638691];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    #[test]
    fn roulette_selection_test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let actual_histogram = (0..1000)
            .map(|_| RouletteWheelSelection.select(&mut rng, &population))
            .fold(BTreeMap::default(), |mut histogram, individual| {
                *histogram.entry(individual.fitness() as i32).or_default() += 1;
                histogram
            });

        let expected_histogram = maplit::btreemap! {
            // fitness => how many times this fitness has been chosen
            1 => 98,
            2 => 202,
            3 => 278,
            4 => 422,
        };

        assert_eq!(actual_histogram, expected_histogram);
    }

    fn chromosome() -> Chromosome {
        Chromosome {
            genes: vec![3.0, 1.0, 2.0],
        }
    }

    #[test]
    fn len() {
        assert_eq!(chromosome().len(), 3);
    }

    #[test]
    fn iter() {
        let chromosome = chromosome();
        let genes: Vec<_> = chromosome.iter().collect();

        assert_eq!(genes.len(), 3);
        assert_eq!(genes[0], 3.0);
        assert_eq!(genes[1], 1.0);
        assert_eq!(genes[2], 2.0);
    }

    #[test]
    fn iter_mut() {
        let mut chromosome = chromosome();

        chromosome.iter_mut().for_each(|gene| {
            *gene *= 10.0;
        });

        let genes: Vec<_> = chromosome.iter().collect();

        assert_eq!(genes.len(), 3);
        assert_eq!(genes[0], 30.0);
        assert_eq!(genes[1], 10.0);
        assert_eq!(genes[2], 20.0);
    }

    #[test]
    fn index() {
        let chromosome = chromosome();

        assert_eq!(chromosome[0], 3.0);
        assert_eq!(chromosome[1], 1.0);
        assert_eq!(chromosome[2], 2.0);
    }

    #[test]
    fn from_iterator() {
        let chromosome: Chromosome = chromosome().iter().collect();

        assert_eq!(chromosome[0], 3.0);
        assert_eq!(chromosome[1], 1.0);
        assert_eq!(chromosome[2], 2.0);
    }

    #[test]
    fn test_even() {
        let stats = Statistics::new(&[
            TestIndividual::new(30.0),
            TestIndividual::new(10.0),
            TestIndividual::new(20.0),
            TestIndividual::new(40.0),
        ]);

        approx::assert_relative_eq!(stats.min_fitness(), 10.0);
        approx::assert_relative_eq!(stats.max_fitness(), 40.0);
        approx::assert_relative_eq!(stats.avg_fitness(), (10.0 + 20.0 + 30.0 + 40.0) / 4.0);
        approx::assert_relative_eq!(stats.median_fitness(), (20.0 + 30.0) / 2.0);
    }

    #[test]
    fn test_odd() {
        let stats = Statistics::new(&[
            TestIndividual::new(30.0),
            TestIndividual::new(20.0),
            TestIndividual::new(40.0),
        ]);

        approx::assert_relative_eq!(stats.min_fitness(), 20.0);
        approx::assert_relative_eq!(stats.max_fitness(), 40.0);
        approx::assert_relative_eq!(stats.avg_fitness(), (20.0 + 30.0 + 40.0) / 3.0);
        approx::assert_relative_eq!(stats.median_fitness(), 30.0);
    }

    fn individual(genes: &[f32]) -> TestIndividual {
        TestIndividual::create(genes.iter().cloned().collect())
    }

    #[allow(clippy::excessive_precision)] // formatting the numbers differently would make the test less readable
    #[test]
    fn gen_algo_test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection,
            UniformCrossover,
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 1.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[1.0, 2.0, 4.0]),
        ];

        for _ in 0..10 {
            population = ga.evolve(&mut rng, &population).0;
        }

        let expected_population = vec![
            individual(&[0.44769490, 2.0648358, 4.3058133]),
            individual(&[1.21268670, 1.5538777, 2.8869110]),
            individual(&[1.06176780, 2.2657390, 4.4287640]),
            individual(&[0.95909685, 2.4618788, 4.0247330]),
        ];

        assert_eq!(population, expected_population);
    }
}
