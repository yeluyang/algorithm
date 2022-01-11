fn gen_minesweeper(m: usize, n: usize, k: usize) -> Vec<Vec<i32>> {
    let mine = {
        let mut buf = Vec::with_capacity(m * n);
        for i in 0..m * n {
            buf.push(i);
        }
        let mut ret = Vec::with_capacity(k);
        for i in 0..k {
            let r = rand::random::<usize>() % (m * n - i);
            ret.push(buf[r]);
            buf.swap(r, m * n - i - 1)
        }
        ret
    };
    let mut ret = vec![vec![0; n]; m];
    for l in mine {
        let i = l / n;
        let j = l % n;
        ret[i][j] = -1;
        let i_rng = match (i > 0, i < m - 1) {
            (true, true) => i - 1..=i + 1,
            (false, true) => i..=i + 1,
            (true, false) => i - 1..=i,
            (false, false) => i..=i,
        };
        let j_rng = match (j > 0, j < n - 1) {
            (true, true) => j - 1..=j + 1,
            (false, true) => j..=j + 1,
            (true, false) => j - 1..=j,
            (false, false) => j..=j,
        };
        for i in i_rng {
            for j in j_rng.clone() {
                if ret[i][j] >= 0 {
                    ret[i][j] += 1;
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let cases = vec![(5usize, 4usize, 3usize)];
        for c in cases {
            let mut actual = gen_minesweeper(c.0, c.1, c.2);
            assert_eq!(actual.len(), c.0);
            for i in 0..c.0 {
                assert_eq!(actual[i].len(), c.1);
                for j in 0..c.1 {
                    if actual[i][j] < 0 {
                        assert_eq!(actual[i][j], -1);
                        actual[i][j] = -1;
                        let i_rng = match (i > 0, i < c.0 - 1) {
                            (true, true) => i - 1..=i + 1,
                            (false, true) => i..=i + 1,
                            (true, false) => i - 1..=i,
                            (false, false) => i..=i,
                        };
                        let j_rng = match (j > 0, j < c.1 - 1) {
                            (true, true) => j - 1..=j + 1,
                            (false, true) => j..=j + 1,
                            (true, false) => j - 1..=j,
                            (false, false) => j..=j,
                        };
                        for i in i_rng {
                            for j in j_rng.clone() {
                                if actual[i][j] > 0 {
                                    actual[i][j] -= 1;
                                }
                            }
                        }
                    }
                }
            }
            let mut count = 0usize;
            for i in 0..c.0 {
                for j in 0..c.1 {
                    assert!(actual[i][j] <= 0);
                    if actual[i][j] < 0 {
                        count += 1;
                    }
                }
            }
            assert_eq!(count, c.2);
        }
    }
}
