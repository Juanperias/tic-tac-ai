use std::fs::read_to_string;

use anyhow::Result;
use smartcore::linalg::basic::matrix::DenseMatrix;

use crate::models::{data::Pieces, model::Model};

pub fn parse_game(game: &str) -> DenseMatrix<i32> {
    let mut board: Vec<i32> = Vec::new();
    for square in game.chars() {
        match square {
            'x' => board.push(Pieces::X as i32),
            'o' => board.push(Pieces::O as i32),
            'b' => board.push(Pieces::Blank as i32),
            _ => board.push(Pieces::Blank as i32),
        };
    }

    if board.len() > 9 {
        board.truncate(9);
    } else {
        while board.len() < 9 {
            board.push(Pieces::Blank as i32);
        }
    }
    DenseMatrix::from_2d_vec(&vec![board])
}

pub fn model(game: &String, file: &String) -> Result<()> {
    let content = read_to_string(file)?;
    let model: Model = serde_json::from_str(&content)?;

    let input_matrix = parse_game(game);
    let result = model.classifier.predict(&input_matrix)?;

    if result == vec![1] {
        println!("X wins this game!");
    } else {
        println!("O wins this game!");
    }
    Ok(())
}
