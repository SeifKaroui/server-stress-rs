use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn save_tuples<P>(file_name: P, results: Vec<(i16, i16, i16)>) -> ()
where
    P: AsRef<Path>,
{
    let mut file = File::create(file_name).unwrap();

    for tuple in results.iter() {
        file.write(tuple.0.to_ne_bytes().as_ref()).unwrap();
        file.write(tuple.1.to_ne_bytes().as_ref()).unwrap();
        file.write(tuple.2.to_ne_bytes().as_ref()).unwrap();
       
        //print!("{:?}\n", tuple);
    }
}
