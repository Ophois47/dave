use std::fs::File;
use std::io;
use linfa::traits::{Fit, Predict};
use linfa::DatasetBase;
use linfa_clustering::KMeans;
use linfa_datasets::generate;
use linfa_nn::distance::LInfDist;
use ndarray::{array, Axis};
use ndarray_npy::write_npy;

// Routine K-Means Task:
// Build Synthetic Dataset, Fit Algorithm On It
// Save Training Data and Predictions to Disk
pub fn kmeans_task() -> io::Result<()> {
	// Get Random Value
	let mut rng = rand::thread_rng();

	// For Each Expected Centroid, Generate 'n' Data Points
	// Around it in a Blob
	let expected_centroids = array![[10., 10.], [1., 12.], [20., 30.], [-20., 30.],];
	let n = 10000;
	let dataset = DatasetBase::from(generate::blobs(n, &expected_centroids, &mut rng));

	// Configure Training Algorithm
	let n_clusters = expected_centroids.len_of(Axis(0));
	let model = KMeans::params_with(n_clusters, rng, LInfDist)
		.max_n_iterations(200)
		.tolerance(1e-5)
		.fit(&dataset)
		.expect("KMeans Fitted");

	// Assign Each Point to a Cluster Using Set of Centroids
	// Found Using 'Fit'
	let dataset = model.predict(dataset);
	let DatasetBase {
		records, targets, ..
	} = dataset;

	// Save Dataset to Disk and Cluster Label Assigned to Each
	// Observation. Using 'npy' Format for Compatibility with NumPy
	let dataset_file_string = "./dave_conf/var/daves_machines/clustered_dataset.npy";
	File::create(dataset_file_string)?;
	let memberships_file_string = "./dave_conf/var/daves_machines/clustered_memberships.npy";
	File::create(memberships_file_string)?;

	println!("##==> Writing Records to Dataset File ...");
	write_npy(
		dataset_file_string,
		&records,
	).expect("Failed to write clustered_dataset.npy file");
	println!("##==> INFO! Dataset File Written Successfully\n");

	println!("##==> Writing Targets to Memberships File ...");
	write_npy(
		memberships_file_string,
		&targets.map(|&x| x as u64),
	).expect("Failed to write to clustered_memberships.npy file");
	println!("##==> INFO! Memberships File Written Successfully");

	Ok(())
}
