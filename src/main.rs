mod circular_vec;
mod solver;
mod tuple_gen;
mod tuple_read;
mod tuple_save;
use circular_vec::CircularVec;
use solver::solve;
use std::time::{Duration, Instant};

fn main() {
    println!("Generating tuples\n");
    let start = Instant::now();

    let results = tuple_gen::generate_tuples(50_000_000);
    let duration = start.elapsed();
    println!(
        "Generated tuples in: {:?} milliseconds\n",
        duration.as_millis()
    );

    println!("Solving {} eqations\n", results.len());
    let start = Instant::now();
    
    let mut solutions = solver::solve(results);
   
    let duration = start.elapsed();
    println!(
        "Solved equations in: {:?} milliseconds \n",
        duration.as_millis()
    );

    //let file_name = String::from("foo.txt");
    // print!("{}\n", results.len());
    // tuple_save::save_tuples(file_name.clone(), results);
    // let r = tuple_read::read_tuples(file_name);
    // print!("resutls len: {:?}\n", r.len() );

    // let mut c = CircularVec::new(vec![(1, 2, 3), (4, 6, 3), (9, 9, 9)]);
    // for _i in 0..20 {
    //     print!("next: {:?}\n", c.next());
    // }
}
