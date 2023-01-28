/* Prompt: Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.


How many such routes are there through a 20×20 grid?

Link: https://projecteuler.net/problem=15
 */
fn calc_paths(m: usize, n: usize) -> usize {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; n + 1]; m + 1];
    for i in 0..m + 1 {
        for j in 0..n + 1 {
            if i == 0 || j == 0 {
                grid[i][j] = 1;
            } else {
                grid[i][j] = grid[i][j - 1] + grid[i - 1][j]
            }
        }
    }
    grid[m][n]
}

fn main() {
    println!("The number of total paths is {}", calc_paths(20, 20));
    // The number of total paths is 137846528820
}
