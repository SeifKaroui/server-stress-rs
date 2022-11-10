use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_tuples<P>(file_name: P) -> Vec<(i16, i16, i16)>
where
    P: AsRef<Path>,
{
    let mut read_results: Vec<(i16, i16, i16)> = vec![];

    let mut file = File::open(file_name).unwrap();
    let mut file_content = Vec::new();

    file.read_to_end(&mut file_content).expect("Unable to read");

    for line in file_content.chunks(6) {
        print!("line len: {}", line.len());
        if line.len() > 0 {
            let a = i16::from_ne_bytes([line[0], line[1]]);
            let b = i16::from_ne_bytes([line[2], line[3]]);
            let c = i16::from_ne_bytes([line[4], line[5]]);
            let t = (a, b, c);
            read_results.push(t);
            //print!("tuple: {:?}\n", t);
        }
    }
    read_results
}
