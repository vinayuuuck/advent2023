
use std::collections::HashMap;
use std::io;
use std::fs;

fn main() {

    let mut colours = HashMap::new();
    colours.insert("red", 12);
    colours.insert("green", 13);
    colours.insert("blue", 14);

    let mut inputfile = String::new();
    println!("Enter the path of the input file: ");
    io::stdin().read_line(&mut inputfile).expect("Failed to read line");

    let content = fs::read_to_string(inputfile.trim()).expect("Something went wrong reading the file");
    let lines = content.split("\n");

    let mut sum = 0;
    for line in lines {
        sum += game_possible(line, &colours);
    }

    println!("The sum of all possible games is: {}", sum);
}

fn game_possible(game: &str, colours: &HashMap<&str, i32>) -> i32 {
    let mut cubes = HashMap::new();
    cubes.insert("red", 0);
    cubes.insert("green", 0);
    cubes.insert("blue", 0);

    let mut possible = true;

    let game_num = &game[5..game.find(":").unwrap()];
    let game_num: i32 = game_num.parse().unwrap();

    let sets = game[game.find(":").unwrap()+2..].split("; ");

    for s in sets{
        let g = s.split(", ");

        for c in g {
            let mut c = c.split(" ");
            let num = c.next().unwrap();
            let num: i32 = num.parse().unwrap();
            let colour = c.next().unwrap();

            if num > colours[&colour] {
                possible = false;
                break;
            }
        }
        if !possible {
            break;
        }
    }

    if possible {
        return game_num;
    } else {
        return 0;
    }

}