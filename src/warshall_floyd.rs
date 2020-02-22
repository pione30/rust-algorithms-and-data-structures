use std::cmp;

pub fn warshall_floyd(num_of_vertexes: usize, dists: &mut Vec<Vec<isize>>) {
    for k in 0..num_of_vertexes {
        for i in 0..num_of_vertexes {
            for j in 0..num_of_vertexes {
                dists[i][j] = cmp::min(dists[i][j], dists[i][k] + dists[k][j]);
            }
        }
    }
}

mod test {
    use super::*;

    const INF: isize = isize::max_value() / 3;

    #[test]
    fn calc_shortest_path() {
        let num_of_vertexes = 7;
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

        let mut dists = vec![vec![INF; num_of_vertexes]; num_of_vertexes];
        for i in 0..num_of_vertexes {
            dists[i][i] = 0;
        }

        for (from, to, cost) in inputs {
            dists[from][to] = cost;
            dists[to][from] = cost;
        }

        warshall_floyd(num_of_vertexes, &mut dists);

        assert_eq!(dists[0][0], 0);
        assert_eq!(dists[1][5], 7);
        assert_eq!(dists[0][6], 16);
    }
}
