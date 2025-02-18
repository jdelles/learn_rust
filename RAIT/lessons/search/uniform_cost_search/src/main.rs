use std::collections::VecDeque;

const GRID: [[i32; 6]; 5] = [
    [0, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 0],
    [0, 0, 1, 1, 1, 0],
    [0, 0, 0, 0, 1, 0],
];

const INIT: [usize; 2] = [0, 0];
const GOAL: [usize; 2] = [GRID.len() - 1, GRID[0].len() - 1];
const COST: i32 = 1;

const DELTA: [[i32; 2]; 4] = [[-1, 0], [0, -1], [1, 0], [0, 1]];

fn search(
    grid: &[[i32; 6]; 5],
    init: [usize; 2],
    cost: i32,
) -> Result<Vec<(i32, usize, usize)>, &'static str> {
    let mut closed_list = vec![vec![0; grid[0].len()]; grid.len()];
    closed_list[init[0]][init[1]] = 1;

    let mut g = 0;
    let mut x = init[0];
    let mut y = init[1];

    let mut open_list = VecDeque::new();
    open_list.push_back((g, x, y));

    loop {
        if open_list.is_empty() {
            return Err("fail");
        } else {
            open_list.make_contiguous().sort_by(|a, b| b.cmp(a));
            let next_node = open_list.pop_back().unwrap();
            g = next_node.0;
            x = next_node.1;
            y = next_node.2;
            closed_list[x][y] = 1;

            if x == GOAL[0] && y == GOAL[1] {
                return Ok(vec![(g, x, y)]);
            } else {
                for i in 0..DELTA.len() {
                    let x2 = (x as i32 + DELTA[i][0]) as usize;
                    let y2 = (y as i32 + DELTA[i][1]) as usize;
                    if x2 < grid.len() && y2 < grid[0].len() {
                        if closed_list[x2][y2] == 0 && grid[x2][y2] == 0 {
                            let g2 = g + cost;
                            open_list.push_back((g2, x2, y2));
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    match search(&GRID, INIT, COST) {
        Ok(path) => println!("Path found: {:?}", path),
        Err(message) => println!("Search failed: {}", message),
    }
}
