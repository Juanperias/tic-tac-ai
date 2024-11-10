use crate::models::{data::CsvData, model::Model};
use anyhow::Result;
use smartcore::{
    ensemble::random_forest_classifier::RandomForestClassifier,
    neighbors::knn_classifier::KNNClassifier,
    tree::decision_tree_classifier::DecisionTreeClassifier,
};

pub fn start(file: &String, dist: &String) -> Result<()> {
    let data = CsvData::from_csv(file)?;
    let train_data = data.to_train_data();

    println!("Training model!");
    let classifier = DecisionTreeClassifier::fit(&train_data.x, &train_data.y, Default::default())?;
    let model = Model::new(classifier);
    model.export(dist)?;

    Ok(())
}
