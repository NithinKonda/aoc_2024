use reqwest::blocking;
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    
    let session_cookie = "session_cookie";
    let url = "https://adventofcode.com/2024/day/1/input";


    let client = blocking::Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    
    for line in response.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Some(&left), Some(&right)) = (parts.get(0), parts.get(1)) {
            left_list.push(left.parse()?);
            right_list.push(right.parse()?);
        }
    }

    let mut right_freq: HashMap<i32, i32> = HashMap::new();
    for &num in &right_list {
        *right_freq.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = left_list
        .iter()
        .map(|&num| num * right_freq.get(&num).unwrap_or(&0))
        .sum();

    println!("Similarity score: {}", similarity_score);

    Ok(())
}
