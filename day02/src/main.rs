use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut sums: [i32; 3] = [20, 10, 30];
    let mut counter: i32 = 0;

    for line in reader.lines() {
        match line {
            Ok(s) => {
                if s.is_empty() {
                    let min = sums.iter().min().unwrap();
                    let index = sums.iter().position(|&r| r == *min).unwrap();
                    
                    if counter > *min {
                        sums[index] = counter;
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

    let sum: i32 = sums.iter().sum();
    println!("{:?}", sum);

    Ok(())
}
