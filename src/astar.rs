use super::node::Node;
use ndarray::Array2;
use std::collections::BinaryHeap;

type HeuristicFn = fn(usize, usize, usize, usize) -> f64;

pub fn astar(
    weights: &Array2<f64>,
    start: usize,
    goal: usize,
    diag_ok: bool,
    heuristic_fn: HeuristicFn,
) -> i32 {
    let width = weights.shape()[0];
    let height = weights.shape()[1];
    let path = vec![0; width * height];

    let start_node = Node::new(start, 0.0, 0);
    let mut costs = vec![f64::INFINITY; width * height];
    costs[0] = 0.0;

    let mut frontier = BinaryHeap::new();
    frontier.push(start_node);

    let nbrs: [usize; 8] = [0; 8];

    let goal_i = goal / width;
    let goal_j = goal % width;
    let start_i = start / width;
    let start_j = start % width;

    while (frontier.capacity() != 0) {
        let current = frontier.pop().unwrap();
        let current_idx = current.idx;
        let current_i = current_idx / width;
        let current_j = current_idx % width;

        if current_idx == goal {
            return current.path_length;
        }

        for i in 0..8 {
            let nbr_i = current_i + nbrs[i];
            let nbr_j = current_j + nbrs[i + 1];

            if nbr_i < 0 || nbr_i >= height || nbr_j < 0 || nbr_j >= width {
                continue;
            }

            let nbr_idx = nbr_i * width + nbr_j;
            let cost = weights[[nbr_i, nbr_j]];

            if cost == f64::INFINITY {
                continue;
            }

            let new_cost = current.cost + cost;
            let heuristic = heuristic_fn(nbr_i, nbr_j, goal_i, goal_j);
            let priority = new_cost + heuristic;

            if new_cost < costs[nbr_idx] {
                costs[nbr_idx] = new_cost;
                let nbr_node = Node::new(nbr_idx, priority, current.path_length + 1);
                frontier.push(nbr_node);
            }
        }
    }

    0
}
