use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone)]
pub struct Edge {
    pub to: usize,
    pub cost: isize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: isize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize) -> isize {
    let mut used = vec![false; adj_list.len()];
    let mut res = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if used[position] {
            continue;
        }

        for adj in &adj_list[position] {
            if !used[adj.to] {
                heap.push(State {
                    cost: adj.cost,
                    position: adj.to,
                });
            }
        }
        used[position] = true;
        res += cost;
    }

    res
}

mod test {
    use super::*;

    #[test]
    fn calc_shortest_path_introduction_to_algorithms() {
        let inputs = vec![
            // (from, to, cost)
            (0, 1, 4),
            (0, 7, 8),
            (1, 2, 8),
            (1, 7, 11),
            (2, 3, 7),
            (2, 5, 4),
            (2, 8, 2),
            (3, 4, 9),
            (3, 5, 14),
            (4, 5, 10),
            (5, 6, 2),
            (6, 7, 1),
            (6, 8, 6),
            (7, 8, 7),
        ];

        let mut graph = vec![vec![]; 9];

        for (from, to, cost) in inputs {
            graph[from].push(Edge { to: to, cost: cost });
            graph[to].push(Edge {
                to: from,
                cost: cost,
            });
        }

        let min_cost = shortest_path(&graph, 0);
        assert_eq!(min_cost, 37);
    }

    #[test]
    fn calc_shortest_path_arihon() {
        let inputs = vec![
            // (from, to, cost)
            (0, 2, 1),
            (1, 2, 2),
            (1, 4, 10),
            (2, 3, 3),
            (2, 5, 7),
            (3, 5, 1),
            (3, 6, 5),
            (4, 5, 5),
            (5, 6, 8),
        ];

        let mut graph = vec![vec![]; 7];

        for (from, to, cost) in inputs {
            graph[from].push(Edge { to: to, cost: cost });
            graph[to].push(Edge {
                to: from,
                cost: cost,
            });
        }

        let min_cost = shortest_path(&graph, 0);
        assert_eq!(min_cost, 17);
    }
}
