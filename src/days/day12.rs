use std::cmp::Ordering;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::{BinaryHeap, HashSet};

pub fn solve() -> SolutionPair {
    (Solution::Usize(part_one()), Solution::Usize(part_two()))
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    steps: usize,
    x: usize,
    y: usize,
    val: usize
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

}

fn part_one() -> usize{
    let grid: Vec<Vec<_>> = read_to_string("input/day12.txt").unwrap()
        .lines()
        .map(|line| line.chars()
            .map(|x| {
                if x == 'S'{
                    0
                } else if x == 'E' {
                    27
                } else {
                    x as usize - 96
                }
            })
            .collect())
        .collect();

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    let starting_grid_location =  grid
        .iter()
        .enumerate()
        .map(|col| (col.0, col.1))
        .flat_map(|x| x.1.iter().enumerate().map(move |row| (x.0, row.0, row.1)))
        .filter(|x| *x.2 == 0)
        .next()
        .unwrap();

    let corners_excluded = vec![(-1, 0), (0,-1), (1,0), (0, 1)];
    let starting_node = Node{steps: 0, x: starting_grid_location.0, y: starting_grid_location.1, val: *starting_grid_location.2 };
    queue.push(starting_node);
    visited.insert((starting_grid_location.0, starting_grid_location.1));

    let mut final_steps = 0;

    while let Some(Node{steps, x, y, val}) = queue.pop() {
        if val == 27{
            final_steps = steps;
            break;
        }

        for neighbour in corners_excluded.iter(){
            let mut neighbour_x = x;
            let mut neighbour_y = y;
            if neighbour.0 == -1{
                neighbour_x = neighbour_x.checked_sub(1).unwrap_or(0);
            } else {
                neighbour_x = neighbour_x + neighbour.0 as usize;
            }
            if neighbour.1 == -1{
                neighbour_y = neighbour_y.checked_sub(1).unwrap_or(0);
            } else {
                neighbour_y = neighbour_y + neighbour.1 as usize;
            }

            let next_node_location = (neighbour_x, neighbour_y);

            if  neighbour_x < grid.len() && neighbour_y < grid[0].len() &&
                !visited.contains(&next_node_location) &&
                grid[neighbour_x][neighbour_y] as i32 - val as i32 <= 1{

                queue.push(Node{steps: steps + 1, x: neighbour_x, y: neighbour_y, val: grid[neighbour_x][neighbour_y]});
                visited.insert(next_node_location);
            }
        }
    };
    return final_steps;
}

fn part_two() -> usize{
    let grid: Vec<Vec<_>> = read_to_string("input/day12.txt").unwrap()
        .lines()
        .map(|line| line.chars()
            .map(|x| {
                if x == 'S'{
                    0
                } else if x == 'E' {
                    27
                } else {
                    x as usize - 96
                }
            })
            .collect())
        .collect();

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let starting_grid_location =  grid
        .iter()
        .enumerate()
        .map(|col| (col.0, col.1))
        .flat_map(|x| x.1.iter().enumerate().map(move |row| (x.0, row.0, row.1)))
        .filter(|x| *x.2 == 27)
        .next()
        .unwrap();

    let corners_excluded = vec![(-1, 0), (0,-1), (1,0), (0, 1)];
    let starting_node = Node{steps: 0, x: starting_grid_location.0, y: starting_grid_location.1, val: *starting_grid_location.2 };
    queue.push(starting_node);
    visited.insert((starting_grid_location.0, starting_grid_location.1));

    let mut final_steps = 0;

    while let Some(Node{steps, x, y, val}) = queue.pop() {
        if val == 1{
            final_steps = steps;
            break;
        }

        for neighbour in corners_excluded.iter(){
            let mut neighbour_x = x;
            let mut neighbour_y = y;
            if neighbour.0 == -1{
                neighbour_x = neighbour_x.checked_sub(1).unwrap_or(0);
            } else {
                neighbour_x = neighbour_x + neighbour.0 as usize;
            }
            if neighbour.1 == -1{
                neighbour_y = neighbour_y.checked_sub(1).unwrap_or(0);
            } else {
                neighbour_y = neighbour_y + neighbour.1 as usize;
            }

            let next_node_location = (neighbour_x, neighbour_y);

            if neighbour_x < grid.len() && neighbour_y < grid[0].len() &&
                !visited.contains(&next_node_location) &&
                grid[neighbour_x][neighbour_y] as i32 - val as i32 >= -1{

                queue.push(Node{steps: steps + 1, x: neighbour_x, y: neighbour_y, val: grid[neighbour_x][neighbour_y]});
                visited.insert(next_node_location);
            }
        }
    };

    return final_steps;
}