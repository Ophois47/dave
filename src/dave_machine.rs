use std::fs::File;
use std::io;
use linfa::dataset::{Labels, Records};
use linfa::metrics::SilhouetteScore;
use linfa::traits::{Fit, Predict, Transformer};
use linfa::DatasetBase;
use linfa_clustering::{Dbscan, KMeans};
use linfa_datasets::generate;
use linfa_nn::distance::LInfDist;
use ndarray::{array, Axis};
use ndarray_npy::write_npy;

// Routine K-Means Task:
// Build Synthetic Dataset, Fit Algorithm On It
// Save Training Data and Predictions to Disk
pub fn kmeans_task() -> io::Result<()> {
	// Get Random Value Generator
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
	let dataset_file_string = "./dave_conf/var/daves_machines/kmeans_clustered_dataset.npy";
	File::create(dataset_file_string)?;
	let memberships_file_string = "./dave_conf/var/daves_machines/kmeans_clustered_memberships.npy";
	File::create(memberships_file_string)?;

	println!("---------------------------------------------------------");
	println!("##==> Writing KMeans Records to Dataset File ...");
	write_npy(
		dataset_file_string,
		&records,
	).expect("Failed to write kmeans_clustered_dataset.npy file");
	println!("##==> INFO! KMeans Dataset File Written Successfully\n");

	println!("##==> Writing KMeans Targets to Memberships File ...");
	write_npy(
		memberships_file_string,
		&targets.map(|&x| x as u64),
	).expect("Failed to write to kmeans_clustered_memberships.npy file");
	println!("##==> INFO! KMeans Memberships File Written Successfully");
	println!("---------------------------------------------------------");

	Ok(())
}

// Routine DBScan Task:
// Build Synthetic Dataset
// Predict Clusters For It
// Save Training Data + Predictions to Disk
pub fn dbscan_task() -> io::Result<()> {
	// Get Random Value Generator
	let mut rng = rand::thread_rng();

	// For Each Expected Centroid, Generate 'n' Data Points
	// Around it in a Blob
	let expected_centroids = array![[10., 10.], [1., 12.], [20., 30.], [-20., 30.],];
	let n = 100;
	let dataset: DatasetBase<_, _> = generate::blobs(n, &expected_centroids, &mut rng).into();

	// Configure Training Algorithm
	let min_points = 3;

	println!(
		"##==> Clustering #{} Data Points Grouped In 4 Clusters Of {} Points Each",
		dataset.nsamples(),
		n,
	);

	// Infer An Optimal Set Of Centroids Based On Training Data Distribution
	let cluster_memberships = Dbscan::params(min_points)
		.tolerance(1.)
		.transform(dataset)
		.unwrap();

	// Single Target Dataset
	let label_count = cluster_memberships.label_count().remove(0);

	println!();
	println!("##==>> Result: ");
	for (label, count) in label_count {
		match label {
			None => println!(" - {} Noise Points", count),
			Some(i) => println!(" - {} Points In Cluster {}", count, i),
		}
	}
	println!();

	let silhouette_score = cluster_memberships.silhouette_score().unwrap();
	println!("##==> Silhouette Score: {}\n", silhouette_score);

	// Save Dataset + Cluster Label Assigned To Each Observation
	// To Disk Using 'npy' Format For Compatibility With NumPy
	let dataset_file_string = "./dave_conf/var/daves_machines/dbscan_clustered_dataset.npy";
	File::create(dataset_file_string)?;
	let memberships_file_string = "./dave_conf/var/daves_machines/dbscan_clustered_memberships.npy";
	File::create(memberships_file_string)?;
	let (records, cluster_memberships) = (cluster_memberships.records, cluster_memberships.targets);

	println!("---------------------------------------------------------");
	println!("##==> Writing DBScan Records to Dataset File ...");
	write_npy(
		dataset_file_string,
		&records,
	).expect("Failed to write dbscan_clustered_dataset.npy file");
	println!("##==> INFO! DBScan Dataset File Written Successfully\n");

	println!("##==> Writing DBScan Targets to Memberships File ...");
	write_npy(
		memberships_file_string,
		&cluster_memberships.map(|&x| x.map(|c| c as i64).unwrap_or(-1)),
	).expect("Failed to write to dbscan_clustered_memberships.npy file");
	println!("##==> INFO! DBScan Memberships File Written Successfully");
	println!("---------------------------------------------------------");

	Ok(())
}
