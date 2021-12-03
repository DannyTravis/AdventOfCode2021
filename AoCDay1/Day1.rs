use std::fs::File;
use std::path::Path;
use std::io::{self, prelude::*, BufReader};
use std::io::Read;

fn main() -> std::io::Result<()> {
    // Open AoC File
    let mut last= 0;
    let mut count = 0;
    let mut rollNum = 0;
    let file = File::open("AoC_input_Day1.txt").expect("Cant Open!!");
    let mut reader = BufReader::new(file);
    for line in reader.lines(){
        // we need three continous reads for current
        if last == 0{
            let current = line.unwrap().parse().unwrap(); 
            last = current;
        }
        else{
            let current = line.unwrap().parse().unwrap(); // current value
            if current > last{
                count = count+1;
                }
            last = current;
        }
    }
    println!("Number: {}",count);
    io::stdin().read_line(&mut String::new()).unwrap();
   Ok(())
}

