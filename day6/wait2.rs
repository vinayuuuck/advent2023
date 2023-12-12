
use std::io;
use std::fs;

fn get_ways(time: i64, dist: i64) -> i64 {
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

    let mut timestr = String::new();
    let mut diststr = String::new();

    let line1 = lines.next().unwrap().split(":").collect::<Vec<&str>>()[1];
    let line2 = lines.next().unwrap().split(":").collect::<Vec<&str>>()[1];

    for i in line1.split(""){
        if i.parse::<i32>().is_ok(){
            timestr.push(i.parse().unwrap());
        }
    }

    for i in line2.split(""){
        if i.parse::<i32>().is_ok(){
            diststr.push(i.parse().unwrap());
        }
    }

    let time = timestr.parse::<i64>().unwrap();
    let dist = diststr.parse::<i64>().unwrap();

    println!("Ways: {}", get_ways(time, dist));

}