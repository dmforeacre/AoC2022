use std::time::Instant;

use crate::OutputStruct;

pub fn pt2() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day2/input.txt").expect("Open failed");

    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    let iter = string.split("\n");
    let mut score = 0;
    iter.into_iter().for_each(|s| {
        if !s.is_empty() {
            let moves: Vec<&str> = s.split(" ").collect();
            match moves[0] {
                "A" => match moves[1] {
                    "X" => score += 3,
                    "Y" => score += 4,
                    _ => score += 8
                },
                "B" => match moves[1] {
                    "X" => score += 1,
                    "Y" => score += 5,
                    _ => score += 9
                },
                _ => match moves[1] {
                    "X" => score += 2,
                    "Y" => score += 6,
                    _ => score += 7
                }
            }
        }
    });

    output.calcTime = clock.elapsed().as_micros();
    output.answer = score.to_string();

    return output;
}