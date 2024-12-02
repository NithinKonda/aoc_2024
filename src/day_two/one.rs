let session_cookie = "";
let url = "https://adventofcode.com/2024/day/2/input";

// Fetch the input
let client = blocking::Client::new();
let response = client
    .get(url)
    .header("Cookie", format!("session={}", session_cookie))
    .send()?
    .text()?;

// Parse the input into reports
let reports: Vec<Vec<i32>> = response
    .lines()
    .filter_map(|line| {
        Some(
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect(),
        )
    })
    .collect();

// Determine the number of safe reports
let safe_count = reports.iter().filter(|report| is_safe(report)).count();

println!("Number of safe reports: {}", safe_count);

Ok(())
}

/// Function to check if a report is safe
fn is_safe(report: &Vec<i32>) -> bool {
// Check if the report is strictly increasing or strictly decreasing
let is_increasing = report.windows(2).all(|pair| match pair {
    [a, b] => *b > *a && (*b - *a) >= 1 && (*b - *a) <= 3,
    _ => false,
});

let is_decreasing = report.windows(2).all(|pair| match pair {
    [a, b] => *a > *b && (*a - *b) >= 1 && (*a - *b) <= 3,
    _ => false,
});

is_increasing || is_decreasing