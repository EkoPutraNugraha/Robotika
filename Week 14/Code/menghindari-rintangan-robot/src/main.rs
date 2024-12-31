use std::collections::VecDeque;

fn main() {
    let grid = vec![
        vec![0, 0, 0, 1, 0],
        vec![0, 1, 0, 1, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 0, 0, 0],
    ];

    let start = (0, 0);
    let goal = (4, 4);

    match find_path(&grid, start, goal) {
        Some(path) => {
            println!("Path found:");
            for step in path {
                println!("({}, {})", step.0, step.1);
            }
        }
        None => println!("No path found."),
    }
}

fn find_path(grid: &Vec<Vec<u8>>, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let rows = grid.len();
    let cols = grid[0].len();

    let directions = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ];

    let mut visited = vec![vec![false; cols]; rows];
    let mut queue = VecDeque::new();
    let mut came_from = vec![vec![None; cols]; rows];

    queue.push_back(start);
    visited[start.0][start.1] = true;

    while let Some(current) = queue.pop_front() {
        if current == goal {
            let mut path = vec![];
            let mut temp = Some(current);
            while let Some(pos) = temp {
                path.push(pos);
                temp = came_from[pos.0][pos.1];
            }
            path.reverse();
            return Some(path);
        }

        for (dx, dy) in &directions {
            let new_x = current.0 as isize + dx;
            let new_y = current.1 as isize + dy;

            if new_x >= 0 && new_y >= 0 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if new_x < rows && new_y < cols && grid[new_x][new_y] == 0 && !visited[new_x][new_y] {
                    queue.push_back((new_x, new_y));
                    visited[new_x][new_y] = true;
                    came_from[new_x][new_y] = Some(current);
                }
            }
        }
    }

    None
}
