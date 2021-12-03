use std::fs::File;
use std::path::Path;
use std::io::{self, prelude::*, BufReader};
use std::io::Read;

fn main() -> std::io::Result<()> {
    // Open AoC File
    let mut last= 0;
    let mut count = 0;
    let mut rollNum = 0;
    let mut rollArray: [i32;3] = [0;3];
    let file = File::open("AoC_input_Day1.txt").expect("Cant Open!!");
    let mut reader = BufReader::new(file);
    for line in reader.lines(){
        // we need three continous reads for current
        if rollNum < 2{
            let current = line.unwrap().parse().unwrap(); 
            rollArray[0] = rollArray[1];
            rollArray[1] = rollArray[2];
            rollArray[2] = current;
            rollNum = rollNum +1;
        }
        else{
            let current = line.unwrap().parse().unwrap(); 
            rollArray[0] = rollArray[1];
            rollArray[1] = rollArray[2];
            rollArray[2] = current;
            let sum = rollArray.iter().sum();
            if last == 0{
                last = sum;
            }
            println!("Current:{} Last:{}",sum,last);
            if sum > last{
                count = count+1;
            }
            last = sum;
        }
    }
    println!("Number: {}",count);
    io::stdin().read_line(&mut String::new()).unwrap();
   Ok(())
}