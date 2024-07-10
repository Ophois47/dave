use rand::{
	Rng,
	RngCore,
};
use std::iter::once;

//
//	Neuron
//
#[derive(Clone, Debug)]
struct Neuron {
	bias: f32,
	weights: Vec<f32>,
}

impl Neuron {
	fn new(bias: f32, weights: Vec<f32>) -> Self {
		assert!(!weights.is_empty());

		Self { bias, weights }
	}

	// fn propagate(&self, inputs: &[f32]) -> Result<f32, String> {
	fn propagate(&self, inputs: &[f32]) -> f32 {
		/*if inputs.len() != self.weights.len() {
			return Err(format!(
				"Got {} inputs but {} inputs were expected",
				inputs.len(),
				self.weights.len(),
			));
		}*/

		assert_eq!(inputs.len(), self.weights.len());
		
		let output = inputs
			.iter()
			.zip(&self.weights)
			.map(|(input, weight)| input * weight)
			.sum::<f32>();

		(self.bias + output).max(0.0)
	}

	fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
		let bias = rng.gen_range(-1.0..=1.0);
		let weights = (0..input_size)
			.map(|_| rng.gen_range(-1.0..=1.0))
			.collect();

		Self { bias, weights }
	}

	fn from_weights(input_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
		let bias = weights.next().expect("Not enough weights received");
		let weights = (0..input_size)
			.map(|_| weights.next().expect("Not enough weights received"))
			.collect();

		Self::new(bias, weights)
	}
}

//
//	Layer
//
#[derive(Clone, Debug)]
struct Layer {
	neurons: Vec<Neuron>,
}

#[derive(Clone, Copy, Debug)]
pub struct LayerTopology {
	pub neurons: usize,
}

impl Layer {
	fn new(neurons: Vec<Neuron>) -> Self {
		assert!(!neurons.is_empty());
		assert!(neurons.iter().all(|neuron| neuron.weights.len() == neurons[0].weights.len()));

		Self { neurons }
	}

	fn from_weights(input_size: usize, output_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
		let neurons = (0..output_size)
			.map(|_| Neuron::from_weights(input_size, weights))
			.collect();

		Self::new(neurons)
	}

	fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
		self.neurons
			.iter()
			.map(|neuron| neuron.propagate(&inputs))
			.collect()
	}

	fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
		let neurons = (0..output_size)
			.map(|_| Neuron::random(rng, input_size))
			.collect();

		Self::new(neurons)
	}
}

//
//	Network
//
#[derive(Clone, Debug)]
pub struct Network {
	layers: Vec<Layer>,
}

impl Network {
	fn new(layers: Vec<Layer>) -> Self {
		Self { layers }
	}

	pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
		self.layers.iter().fold(inputs, |inputs, layer| layer.propagate(inputs))
	}

	pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
		assert!(layers.len() > 1);

		let layers = layers
			.windows(2)
			.map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
			.collect();

		Self::new(layers)
	}

	pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
		assert!(layers.len() > 1);

		let mut weights = weights.into_iter();
		let layers = layers
			.windows(2)
			.map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
			.collect();

		if weights.next().is_some() {
			panic!("Got too many weights");
		}

		Self::new(layers)
	}

	pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
		self.layers
			.iter()
			.flat_map(|layer| layer.neurons.iter())
			.flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
			.cloned()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::{
		assert_relative_eq,
		assert_relative_ne,
	};
	use rand::SeedableRng;
	use rand_chacha::ChaCha8Rng;

	#[test]
	fn neuron_random() {
		let mut rng = ChaCha8Rng::from_seed(Default::default());
		let neuron = Neuron::random(&mut rng, 4);

		assert_relative_eq!(neuron.bias, -0.6255188);
		assert_relative_eq!(
			neuron.weights.as_slice(),
			[0.67383957, 0.8181262, 0.26284897, 0.5238807].as_slice(),
		);
	}

	#[test]
	fn layer_random() {
		let mut rng = ChaCha8Rng::from_seed(Default::default());
		let layer = Layer::random(&mut rng, 3, 2);

		let actual_biases: Vec<_> = layer.neurons.iter().map(|neuron| neuron.bias).collect();
		let expected_biases = vec![-0.6255188, 0.5238807];

		let actual_weights: Vec<_> = layer
			.neurons
			.iter()
			.map(|neuron| neuron.weights.as_slice())
			.collect();

		let expected_weights: Vec<&[f32]> = vec![
			&[0.67383957, 0.8181262, 0.26284897],
            &[-0.53516835, 0.069369674, -0.7648182],
		];

		assert_relative_eq!(actual_biases.as_slice(), expected_biases.as_slice());
        assert_relative_eq!(actual_weights.as_slice(), expected_weights.as_slice());
	}

	#[test]
	fn network_random() {
		let mut rng = ChaCha8Rng::from_seed(Default::default());

        let network = Network::random(
            &mut rng,
            &[
                LayerTopology { neurons: 3 },
                LayerTopology { neurons: 2 },
                LayerTopology { neurons: 1 },
            ],
        );

        assert_eq!(network.layers.len(), 2);
        assert_eq!(network.layers[0].neurons.len(), 2);

        assert_relative_eq!(network.layers[0].neurons[0].bias, -0.6255188);

        assert_relative_eq!(
            network.layers[0].neurons[0].weights.as_slice(),
            &[0.67383957, 0.8181262, 0.26284897].as_slice()
        );

        assert_relative_eq!(network.layers[0].neurons[1].bias, 0.5238807);

        assert_relative_eq!(
            network.layers[0].neurons[1].weights.as_slice(),
            &[-0.5351684, 0.069369555, -0.7648182].as_slice()
        );

        assert_eq!(network.layers[1].neurons.len(), 1);

        assert_relative_eq!(
            network.layers[1].neurons[0].weights.as_slice(),
            &[-0.48879623, -0.19277143].as_slice()
        );
	}

	#[test]
	fn layer_propagate() {
		let neurons = (
            Neuron::new(0.0, vec![0.1, 0.2, 0.3]),
            Neuron::new(0.0, vec![0.4, 0.5, 0.6]),
        );

        let layer = Layer::new(vec![neurons.0.clone(), neurons.1.clone()]);
        let inputs = &[-0.5, 0.0, 0.5];

        let actual = layer.propagate(inputs.to_vec());
        let expected = vec![neurons.0.propagate(inputs), neurons.1.propagate(inputs)];

        assert_relative_eq!(actual.as_slice(), expected.as_slice());
	}

	#[test]
	fn network_propagate() {
		let layers = (
            Layer::new(vec![
                Neuron::new(0.0, vec![-0.5, -0.4, -0.3]),
                Neuron::new(0.0, vec![-0.2, -0.1, 0.0]),
            ]),
            Layer::new(vec![Neuron::new(0.0, vec![-0.5, 0.5])]),
        );
        let network = Network::new(vec![layers.0.clone(), layers.1.clone()]);

        let actual = network.propagate(vec![0.5, 0.6, 0.7]);
        let expected = layers.1.propagate(layers.0.propagate(vec![0.5, 0.6, 0.7]));

        assert_relative_eq!(actual.as_slice(), expected.as_slice());
	}

	mod propagate {
		use super::*;

		#[test]
		fn returns_propagated_input() {
			let actual = Neuron::new(0.1, vec![-0.3, 0.6, 0.9]).propagate(&[0.5, -0.6, 0.7]);
			let expected: f32 = 0.1 + (0.5 * -0.3) + (-0.6 * 0.6) + (0.7 * 0.9);

			approx::assert_relative_eq!(actual, expected.max(0.0));
		}

		#[test]
		fn restricts_output() {
			let neuron = Neuron::new(0.0, vec![0.5]);
			let v1 = neuron.propagate(&[-1.0]);
            let v2 = neuron.propagate(&[-0.5]);
            let v3 = neuron.propagate(&[0.0]);
            let v4 = neuron.propagate(&[0.5]);
            let v5 = neuron.propagate(&[1.0]);

            assert_relative_eq!(v1, v2);
            assert_relative_eq!(v2, v3);
            assert_relative_ne!(v3, v4);
            assert_relative_ne!(v4, v5);
		}
	}

	#[test]
	fn neuron_from_weights() {
		let actual = Neuron::from_weights(3, &mut vec![0.1, 0.2, 0.3, 0.4].into_iter());
		let expected = Neuron::new(0.1, vec![0.2, 0.3, 0.4]);

		assert_relative_eq!(actual.bias, expected.bias);
		assert_relative_eq!(actual.weights.as_slice(), expected.weights.as_slice());
	}

	#[test]
	fn layer_from_weights() {
		let layer = Layer::from_weights(
            3,
            2,
            &mut vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8].into_iter(),
        );

        let actual_biases: Vec<_> = layer.neurons.iter().map(|neuron| neuron.bias).collect();
        let expected_biases = vec![0.1, 0.5];

        let actual_weights: Vec<_> = layer
            .neurons
            .iter()
            .map(|neuron| neuron.weights.as_slice())
            .collect();
        let expected_weights: Vec<&[f32]> = vec![&[0.2, 0.3, 0.4], &[0.6, 0.7, 0.8]];

        assert_relative_eq!(actual_biases.as_slice(), expected_biases.as_slice());
        assert_relative_eq!(actual_weights.as_slice(), expected_weights.as_slice());
	}

	#[test]
	fn network_from_weights() {
		let layers = &[LayerTopology { neurons: 3 }, LayerTopology { neurons: 2 }];
        let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

        let actual: Vec<_> = Network::from_weights(layers, weights.clone())
            .weights()
            .collect();

        assert_relative_eq!(actual.as_slice(), weights.as_slice());
	}

	#[test]
	fn network_weights() {
		let network = Network::new(vec![
			Layer::new(vec![Neuron::new(0.1, vec![0.2, 0.3, 0.4])]),
			Layer::new(vec![Neuron::new(0.5, vec![0.6, 0.7, 0.8])]),
		]);

		let actual: Vec<_> = network.weights().collect();
		let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
		assert_relative_eq!(actual.as_slice(), expected.as_slice());
	}
}
