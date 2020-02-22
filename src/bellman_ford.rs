pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub cost: isize,
}

const INF: isize = isize::max_value() / 3;

pub fn shortest_path(start: usize, num_of_vertexes: usize, edges: &Vec<Edge>) -> Vec<isize> {
    if start >= num_of_vertexes {
        panic!("start should be less than the number of vertexes");
    }

    let mut dists = vec![INF; num_of_vertexes];
    dists[start] = 0;

    loop {
        let mut update = false;

        for i in 0..edges.len() {
            let edge = &edges[i];
            if dists[edge.from] != INF && dists[edge.to] > dists[edge.from] + edge.cost {
                dists[edge.to] = dists[edge.from] + edge.cost;
                update = true;
            }
        }

        if !update {
            break;
        }
    }

    dists
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

        let mut edges = Vec::new();

        for (from, to, cost) in inputs {
            edges.push(Edge {
                from: from,
                to: to,
                cost: cost,
            });
            // undirected graph
            edges.push(Edge {
                from: to,
                to: from,
                cost: cost,
            });
        }

        let dists = shortest_path(0, 7, &edges);

        assert_eq!(dists[0], 0);
        assert_eq!(dists[3], 7);
        assert_eq!(dists[6], 16);
    }
}
