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

    left_list.sort_unstable();
    right_list.sort_unstable();


    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    // Print the result
    println!("Total distance between lists: {}", total_distance);

    Ok(())