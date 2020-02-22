use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone)]
pub struct Edge {
    pub to: usize,
    pub cost: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
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

pub fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize) -> Vec<usize> {
    let mut dist = vec![std::usize::MAX; adj_list.len()];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.to,
            };

            if dist[next.position] > next.cost {
                dist[next.position] = next.cost;
                heap.push(next);
            }
        }
    }

    dist
}

mod test {
    use super::*;

    #[test]
    fn calc_shortest_path() {
        let inputs = vec![
            // (from, to, cost)
            (0, 1, 2),
            (0, 2, 5),
            (1, 2, 4),
            (1, 3, 6),
            (1, 4, 10),
            (2, 3, 2),
            (3, 5, 1),
            (4, 5, 3),
            (4, 6, 5),
            (5, 6, 9),
        ];

        let mut graph = vec![vec![]; 7];

        for (from, to, cost) in inputs {
            graph[from].push(Edge { to: to, cost: cost });

            graph[to].push(Edge {
                to: from,
                cost: cost,
            });
        }

        let dists = shortest_path(&graph, 0);

        assert_eq!(dists[0], 0);
        assert_eq!(dists[3], 7);
        assert_eq!(dists[6], 16);
    }
}
