// You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

// For example, the record of a few games might look like this:

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
// In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

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