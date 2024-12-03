 let session_cookie = "";
    let url = "https://adventofcode.com/2024/day/3/input";


    let client = blocking::Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;


    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;


    let total_sum: i64 = re
        .captures_iter(&response)
        .map(|cap| {
            let x: i64 = cap[1].parse().unwrap();
            let y: i64 = cap[2].parse().unwrap();
            x * y
        })
        .sum();

    println!("Total sum of all valid multiplications: {}", total_sum);

    Ok(())