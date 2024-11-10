use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::Path};

use anyhow::Result;
use smartcore::{
    ensemble::random_forest_classifier::RandomForestClassifier, linalg::basic::matrix::DenseMatrix,
    metrics::distance::euclidian::Euclidian, neighbors::knn_classifier::KNNClassifier,
    tree::decision_tree_classifier::DecisionTreeClassifier,
};

type Classifier = DecisionTreeClassifier<i32, i32, DenseMatrix<i32>, Vec<i32>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub classifier: Classifier,
}

impl Model {
    pub fn new(classifier: Classifier) -> Self {
        Model { classifier }
    }
    pub fn export<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = File::create(path)?;

        let content = serde_json::to_string(&self)?;

        file.write_all(content.as_bytes())?;

        println!("Model successfully exported!");

        Ok(())
    }
}
