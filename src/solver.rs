pub fn solve(equations: Vec<(i16, i16, i16)>) -> Vec<(f64, f64)> {
    let mut solutions = vec![];
    for tuple in equations {
        let delta: f64 = ((tuple.1 as i64).pow(2) - 4 * (tuple.0 as i64) * (tuple.2 as i64)) as f64;
        let x1 =
            -((tuple.1 as f64 + delta.sqrt()) as f64 / (2 as f64 * tuple.0 as f64) as f64) as f64;
        let x2 =
            -((tuple.1 as f64 - delta.sqrt()) as f64 / (2 as f64 * tuple.0 as f64) as f64) as f64;
        solutions.push((x1, x2));
    }
    solutions
}
