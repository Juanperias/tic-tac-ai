// This part is designed for this specific dataset: https://www.kaggle.com/datasets/ulrikthygepedersen/tic-tac-toe

use std::{fs::File, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use smartcore::linalg::basic::matrix::DenseMatrix;

#[repr(i32)]
pub enum Pieces {
    X = 1,
    O = 2,
    Blank = 0,
}

#[repr(i32)]
pub enum Winner {
    X = 1,
    O = 2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CsvItem {
    #[serde(rename = "top-left-square")]
    top_left_square: String,

    #[serde(rename = "top-middle-square")]
    top_middle_square: String,

    #[serde(rename = "top-right-square")]
    top_right_square: String,

    #[serde(rename = "middle-left-square")]
    middle_left_square: String,

    #[serde(rename = "middle-middle-square")]
    middle_middle_square: String,

    #[serde(rename = "middle-right-square")]
    middle_right_square: String,

    #[serde(rename = "bottom-left-square")]
    bottom_left_square: String,

    #[serde(rename = "bottom-middle-square")]
    bottom_middle_square: String,

    #[serde(rename = "bottom-right-square")]
    bottom_right_square: String,

    #[serde(rename = "Class")]
    winner: String,
}

impl CsvItem {
    pub fn get_board(&self) -> Vec<i32> {
        vec![
            // Top
            self.square_to_number(&self.top_left_square),
            self.square_to_number(&self.top_middle_square),
            self.square_to_number(&self.top_right_square),
            // Middle
            self.square_to_number(&self.middle_left_square),
            self.square_to_number(&self.middle_middle_square),
            self.square_to_number(&self.middle_right_square),
            // Bottom
            self.square_to_number(&self.bottom_left_square),
            self.square_to_number(&self.bottom_middle_square),
            self.square_to_number(&self.bottom_right_square),
        ]
    }
    pub fn square_to_number(&self, square: &str) -> i32 {
        match square {
            "b'x'" => Pieces::X as i32,
            "b'o'" => Pieces::O as i32,
            "b'b'" => Pieces::Blank as i32,
            _ => panic!("Unknown Piece"),
        }
    }
    pub fn get_winner(&self) -> i32 {
        match self.winner.as_str() {
            "b'positive'" => Winner::X as i32,
            "b'negative'" => Winner::O as i32,
            _ => panic!("Unknown winner"),
        }
    }
}
#[derive(Debug)]
pub struct CsvData {
    pub items: Vec<CsvItem>,
}

impl CsvData {
    pub fn from_csv<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);
        let items: Vec<CsvItem> = rdr.deserialize().filter_map(Result::ok).collect();
        Ok(Self { items })
    }
    pub fn to_train_data(&self) -> TrainData {
        let data: Vec<Vec<i32>> = self.items.iter().map(|item| item.get_board()).collect();
        let responses: Vec<i32> = self.items.iter().map(|item| item.get_winner()).collect();

        println!("{:?}", responses);
        TrainData {
            x: DenseMatrix::from_2d_vec(&data),
            y: responses,
        }
    }
}

#[derive(Debug)]
pub struct TrainData {
    pub x: DenseMatrix<i32>,
    pub y: Vec<i32>,
}
