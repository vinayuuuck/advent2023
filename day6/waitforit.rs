
use std::io;
use std::fs;

fn get_ways(time: i32, dist: i32) -> i32 {
    let mut ways = 0;

    for i in 0..time {
        if (i * (time-i)) > dist {
            ways += 1;
        }
    }
    ways
}

fn main(){

    println!("Enter the path of the input file: ");
    let mut inputfile = String::new();
    io::stdin().read_line(&mut inputfile).expect("Failed to read line");
    let content = fs::read_to_string(inputfile.trim()).expect("Something went wrong reading the file");

    let mut lines = content.split("\n");

    let mut times: Vec<i32> = Vec::new();
    let mut dists: Vec<i32> = Vec::new();

    let line1 = lines.next().unwrap().split(":").collect::<Vec<&str>>()[1];
    let line2 = lines.next().unwrap().split(":").collect::<Vec<&str>>()[1];

    for i in line1.split(" "){
        if i.parse::<i32>().is_ok(){
            times.push(i.parse::<i32>().unwrap());
        }
    }

    for i in line2.split(" "){
        if i.parse::<i32>().is_ok(){
            dists.push(i.parse::<i32>().unwrap());
        }
    }

    let mut multi_ways = 0;

    for race in 0..times.len() {
        println!("Time: {}, Distance: {}", times[race], dists[race]);
        let ways = get_ways(times[race], dists[race]);
        println!("Ways: {}", ways);
        if ways > 0 {
            if multi_ways == 0 {
                multi_ways = ways;
            } else {
                multi_ways *= ways;
            }
        }
    }

    println!("{}", multi_ways)
}