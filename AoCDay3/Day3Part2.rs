use std::fs::File;
use std::path::Path;
use std::io::{self, prelude::*, BufReader};
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut gamma = 0;
    let mut eps = 0;
    let mut ones_array:[i32;12] = [0;12];
    let mut zero_array:[i32;12] = [0;12];
    let mut Total = 0;
    // Open AoC File
    let file = File::open("Day3Input.txt").expect("Cant Open!!");
    let mut reader = BufReader::new(file);
    for line in reader.lines(){
        // Get OpCode and Dir
        let str = line.unwrap();
        let vals = str.chars();
        let mut ii = 0;
        for c in vals {
            if c == '0'{
               zero_array[ii] = zero_array[ii]+1;
            }
            else if c =='1'{
               ones_array[ii] = ones_array[ii]+1;
            }
            ii = ii+1;
        }
    }
    println!("Results: ");
    let mut gamma_str = String::new();
    let mut eps_str = String::new();
    for item in zero_array.into_iter().enumerate() {
        let (i, x): (usize, &i32) = item;
        print!("Zero's @[{}] = {};", i, x);
        let tmpOne = ones_array[i];
        println!(" One's @[{}] = {} ",i,tmpOne);
        //now to figure out what their asking
    }
    //let GamVal = isize::from_str_radix(&gamma_str,2).unwrap();
    //let EpsVal = isize::from_str_radix(&eps_str,2).unwrap();
    //let Total = GamVal*EpsVal;
    //println!("Total Value: {}",Total);
    io::stdin().read_line(&mut String::new()).unwrap();
   Ok(())
}