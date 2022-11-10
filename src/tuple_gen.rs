use rand::Rng;

pub fn generate_tuples(len:usize) -> Vec<(i16, i16, i16)> {
    let mut results: Vec<(i16, i16, i16)> = vec![];

    let mut rng = rand::thread_rng();

    let results = loop {
        let rand_tuple = rng.gen::<(i16, i16, i16)>();
        let delta: i64 =
            (rand_tuple.1 as i64).pow(2) - 4 * (rand_tuple.0 as i64) * (rand_tuple.2 as i64);
        //println!("Random tuple: {:?} detla: {}", rand_tuple, delta);
        if delta > 0 {
            results.push(rand_tuple);
        }
        if results.len() >= len {
            break results;
        }
    };
    results
}
