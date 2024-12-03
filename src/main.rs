use reqwest::blocking::Client;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch input from the Advent of Code website
    let session_cookie = env::var("").expect("Please set the AOC_SESSION environment variable.");
    let url = "https://adventofcode.com/2024/day/3/input";
    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?;
    let input = response.text()?;

    // Process the input
    let mut enabled = true;
    let mut sum = 0;

    for i in 0..input.len() {
        for j in i + 1..=usize::min(input.len(), i + 15) {
            let slice = &input[i..j];
            if slice == "do()" {
                enabled = true;
            } else if slice == "don't()" {
                enabled = false;
            } else if enabled && slice.starts_with("mul(") && slice.ends_with(")") {
                // Validate and extract numbers
                if let Some((x, y)) = parse_mul(slice) {
                    sum += x * y;
                }
            }
        }
    }

    println!("The sum of enabled multiplications is: {}", sum);
    Ok(())
}

fn parse_mul(slice: &str) -> Option<(i32, i32)> {
    if slice.starts_with("mul(") && slice.ends_with(")") {
        let inner = &slice[4..slice.len() - 1];
        if inner.contains(",") {
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() == 2 {
                if let (Ok(x), Ok(y)) = (parts[0].parse(), parts[1].parse()) {
                    return Some((x, y));
                }
            }
        }
    }
    None
}
