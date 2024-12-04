
    let session_cookie = "";
    let url = "https://adventofcode.com/2024/day/4/input";
    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?;
    let input = response.text()?;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let directions = vec![
        (0, 1),   
        (1, 0),   
        (1, 1),   
        (1, -1),  
        (0, -1),  
        (-1, 0),  
        (-1, -1), 
        (-1, 1),  
    ];

    let word = "XMAS";
    let word_len = word.len();
    let mut count = 0;

    // Search for the word in all directions
    for (row, line) in grid.iter().enumerate() {
        for (col, _) in line.iter().enumerate() {
            for &(dx, dy) in &directions {
                if let Some(_) = find_word(&grid, row as isize, col as isize, dx, dy, word, word_len)
                {
                    count += 1;
                }
            }
        }
    }

    println!("The word '{}' appears {} times.", word, count);
    Ok(())
}

fn find_word(
    grid: &[Vec<char>],
    mut x: isize,
    mut y: isize,
    dx: isize,
    dy: isize,
    word: &str,
    word_len: usize,
) -> Option<()> {
    for (i, ch) in word.chars().enumerate() {
        let nx = x + i as isize * dx;
        let ny = y + i as isize * dy;

        if nx < 0 || ny < 0 || nx >= grid.len() as isize || ny >= grid[0].len() as isize {
            return None;
        }

        if grid[nx as usize][ny as usize] != ch {
            return None;
        }
    }
    Some(())