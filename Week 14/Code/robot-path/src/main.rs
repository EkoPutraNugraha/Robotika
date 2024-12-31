use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse to make it a min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_valid_position(x: isize, y: isize, rows: usize, cols: usize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < rows && (y as usize) < cols
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    // Manhattan distance
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}

fn find_path(grid: &Vec<Vec<u8>>, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), usize> = HashMap::new();
    
    g_score.insert(start, 0);
    open_set.push(Node {
        position: start,
        cost: heuristic(start, goal),
    });

    let directions = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ];

    while let Some(current_node) = open_set.pop() {
        let current = current_node.position;

        if current == goal {
            let mut path = Vec::new();
            let mut temp = current;
            while came_from.contains_key(&temp) {
                path.push(temp);
                temp = came_from[&temp];
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        for (dx, dy) in &directions {
            let next_x = current.0 as isize + dx;
            let next_y = current.1 as isize + dy;

            if is_valid_position(next_x, next_y, rows, cols) {
                let next = (next_x as usize, next_y as usize);
                if grid[next.0][next.1] == 1 {
                    continue;
                }

                let tentative_g_score = g_score.get(&current).unwrap_or(&usize::MAX) + 1;

                if tentative_g_score < *g_score.get(&next).unwrap_or(&usize::MAX) {
                    came_from.insert(next, current);
                    g_score.insert(next, tentative_g_score);
                    open_set.push(Node {
                        position: next,
                        cost: tentative_g_score + heuristic(next, goal),
                    });
                }
            }
        }
    }

    None // No path found
}

fn main() {
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0],
    ];

    let start = (0, 0);
    let goal = (4, 4);

    match find_path(&grid, start, goal) {
        Some(path) => {
            println!("Path found:");
            for (x, y) in path {
                println!("({}, {})", x, y);
            }
        }
        None => println!("No path found."),
    }
}
