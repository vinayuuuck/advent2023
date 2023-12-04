
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

        let win_nums = line[9..].split(" | ").collect::<Vec<&str>>()[0].trim().split(" ").collect::<HashSet<&str>>();
        let our_nums = line[9..].split(" | ").collect::<Vec<&str>>()[1].trim().split(" ").collect::<HashSet<&str>>();

        let win_nums = win_nums.into_iter().filter(|&x| x != "").collect::<HashSet<&str>>();
        let our_nums = our_nums.into_iter().filter(|&x| x != "").collect::<HashSet<&str>>();

        let intersection_len = win_nums.intersection(&our_nums).collect::<HashSet<&&str>>().len();

        if intersection_len != 0 {
            total += 2_i32.pow((intersection_len-1) as u32);
        }
    }

    println!("Total amount won: {}", total);

}