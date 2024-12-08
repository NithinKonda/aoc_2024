use std::fs;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    values: Vec<char>,
}

impl Grid {
    fn new(width: usize, height: usize, values: Vec<char>) -> Self {
        if width * height != values.len() {
            panic!("This grid is not coherent");
        }
        Self {
            width,
            height,
            values,
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        if x < self.width && y < self.height {
            self.values[x + y * self.width]
        } else {
            panic!("This is outside the grid");
        }
    }

    // Extract Subgrid
    fn extract_sub_grid(&self, start_x: usize, start_y: usize, sub_width: usize, sub_height: usize) -> Self {
        if start_x >= self.width || start_y >= self.height {
            panic!("The beginning of the subgrid is outside the grid");
        }
        if start_x + sub_width > self.width || start_y + sub_height > self.height {
            panic!("The end of the subgrid is outside the grid");
        }

        let mut values = Vec::new();
        for j in 0..sub_height {
            for i in 0..sub_width {
                values.push(self.get(start_x + i, start_y + j));
            }
        }
        Grid::new(sub_width, sub_height, values)
    }

    // Check if subgrid matches a certain mask
    fn is_sub_grid_matching_mask(&self, start_x: usize, start_y: usize, mask: &Grid) -> bool {
        let sub_grid = self.extract_sub_grid(start_x, start_y, mask.width, mask.height);
        for j in 0..sub_grid.height {
            for i in 0..sub_grid.width {
                let mask_value = mask.get(i, j);
                if mask_value != ' ' && mask_value != sub_grid.get(i, j) {
                    return false;
                }
            }
        }
        true
    }

    // Count all subgrid matching a mask
    fn count_subgrid_matching_mask(&self, mask: &Grid) -> usize {
        let mut count = 0;
        for j in 0..=self.height - mask.height {
            for i in 0..=self.width - mask.width {
                if self.is_sub_grid_matching_mask(i, j, mask) {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let part2_start_time = Instant::now();

    // Extract Grid
    let input = fs::read_to_string("src/in.txt").expect("Failed to read input file");

    let mut grid_values = Vec::new();
    let mut grid_width = 0;
    let mut grid_height = 0;

    for line in input.lines() {
        let clean_line = line.trim();
        grid_values.extend(clean_line.chars());
        grid_height += 1;
        grid_width = grid_width.max(clean_line.len());
    }

    let grid = Grid::new(grid_width, grid_height, grid_values);

    // Create Masks
    let mask1 = Grid::new(3, 3, vec!['M', ' ', 'S', ' ', 'A', ' ', 'M', ' ', 'S']);
    let mask2 = Grid::new(3, 3, vec!['M', ' ', 'M', ' ', 'A', ' ', 'S', ' ', 'S']);
    let mask3 = Grid::new(3, 3, vec!['S', ' ', 'M', ' ', 'A', ' ', 'S', ' ', 'M']);
    let mask4 = Grid::new(3, 3, vec!['S', ' ', 'S', ' ', 'A', ' ', 'M', ' ', 'M']);

    // Count matches
    let matches = grid.count_subgrid_matching_mask(&mask1)
        + grid.count_subgrid_matching_mask(&mask2)
        + grid.count_subgrid_matching_mask(&mask3)
        + grid.count_subgrid_matching_mask(&mask4);

    println!("Matches: {}", matches);

    let part2_runtime = part2_start_time.elapsed();
    println!("Runtime: {:.6} seconds", part2_runtime.as_secs_f64());
}
