use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut sum: i32 = 0;
    let mut counter: i32 = 0;

    for line in reader.lines() {
        match line {
            Ok(s) => {
                if s.is_empty() {
                    if counter > sum {
                        sum = counter;
                    }
                    counter = 0;
                } else {
                    counter += s.trim().parse::<i32>().unwrap();
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    println!("{}", sum);

    Ok(())
}
