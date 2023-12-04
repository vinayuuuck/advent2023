
use std::io;
use std::fs;
use std::collections::HashSet;

fn main(){
    
    println!("Enter the path of the input file: ");
    let mut inputfile = String::new();
    io::stdin().read_line(&mut inputfile).expect("Failed to read line");
    let content = fs::read_to_string(inputfile.trim()).expect("Something went wrong reading the file");
    let lines = content.split("\n");

    let mut total = 0;

    for line in lines {
        let nums = &line[9..];

        let win_nums_str = nums.split(" | ").collect::<Vec<&str>>()[0].trim().split(" ").collect::<Vec<&str>>();
        let our_nums_str = nums.split(" | ").collect::<Vec<&str>>()[1].trim().split(" ").collect::<Vec<&str>>();

        let mut win_nums = HashSet::new();
        for num in win_nums_str {
            if num != "" {
                win_nums.insert(num.parse::<i32>().unwrap());
            }
        }

        let mut our_nums = HashSet::new();
        for num in our_nums_str {
            if num != "" {
                our_nums.insert(num.parse::<i32>().unwrap());
            }
        }

        let get_win = win_nums.intersection(&our_nums).collect::<Vec<&i32>>().len();
        if get_win !=0 {
            total += 2_i32.pow((get_win-1) as u32);
        }

    }

    println!("Total amount won: {}", total);

}