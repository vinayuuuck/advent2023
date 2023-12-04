
use std::io;
use std::fs;

fn winning_card(win_nums: Vec<i32>, our_nums: Vec<i32>) -> i32 {
    let mut count = 0;
    for num in our_nums {
        if win_nums.contains(&num) {
            count += 1;
        }
    }
    return count;
}

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

        let mut win_nums = Vec::new();
        for num in win_nums_str {
            if num != "" {
                win_nums.push(num.parse::<i32>().unwrap());
            }
        }

        let mut our_nums = Vec::new();
        for num in our_nums_str {
            if num != "" {
                our_nums.push(num.parse::<i32>().unwrap());
            }
        }

        let get_win = winning_card(win_nums, our_nums);
        if get_win !=0 {
            total += 2_i32.pow((get_win-1) as u32);
        }

    }

    println!("Total amount won: {}", total);

}