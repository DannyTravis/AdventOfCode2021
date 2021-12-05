use std::fs::File;
use std::path::Path;
use std::io::{self, prelude::*, BufReader};
use std::io::Read;

fn main() -> std::io::Result<()> {
    // Open AoC File
    let mut Total = 0;
    let mut TotFor = 0;
    let mut TotDepth = 0;
    let file = File::open("AoCDay2.txt").expect("Cant Open!!");
    let mut reader = BufReader::new(file);
    for line in reader.lines(){
        // Get OpCode and Dir
        let str = line.unwrap();
        let split = str.split(" ");
        let splitVec = split.collect::<Vec<&str>>();
        //tested, works, so now what?
        let Op = splitVec[0];
        let Val:i32 = splitVec[1].parse().unwrap();
        if Op == "forward"{
            TotFor = TotFor + Val;
        }else if Op == "down"{
            TotDepth = TotDepth + Val;
        }else{
            TotDepth = TotDepth - Val;
        }
    }
    Total = TotFor*TotDepth;
    println!("Total: {}",Total);
    io::stdin().read_line(&mut String::new()).unwrap();
   Ok(())
}