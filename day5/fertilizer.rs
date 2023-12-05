
use std::io;
use std::fs;
use std::collections::HashMap;
use std::str;

fn seeds(slist: &str) -> HashMap<i32, i32> {
    let mut seeds = HashMap::new();
    for seed in slist.split(" ") {
        seeds.insert(seed.parse::<i32>().unwrap(), seed.parse::<i32>().unwrap());
    }
    seeds
}

fn makemap(inlist: Vec<Vec<i32>>) -> HashMap<i32, i32> {
    // TODO
    return HashMap::new();
}

fn findlist(lines: std::str::Split<'_, &str>, idx: usize) -> Vec<Vec<i32>> {
    // TODO
    return Vec::new();
}

fn main() {

    println!("Enter the path of the input file: ");
    let mut inputfile = String::new();
    io::stdin().read_line(&mut inputfile).expect("Failed to read line");
    let content = fs::read_to_string(inputfile.trim()).expect("Something went wrong reading the file");
    let lines = content.split("\n");
    
    let mut seedmap: HashMap<i32, i32> = HashMap::new();

    let mut s2smap: HashMap<i32, i32> = HashMap::new();
    let mut s2fmap: HashMap<i32, i32> = HashMap::new();
    let mut f2wmap: HashMap<i32, i32> = HashMap::new();
    let mut w2lmap: HashMap<i32, i32> = HashMap::new();
    let mut l2tmap: HashMap<i32, i32> = HashMap::new();
    let mut t2hmap: HashMap<i32, i32> = HashMap::new();
    let mut h2lmap: HashMap<i32, i32> = HashMap::new();

    for (idx, line) in lines.clone().into_iter().enumerate() {

        if line == "" {
            continue;
        }

        if &line[0..5] == "seeds" {
            let seedlist = line[7..].trim();
            seedmap = seeds(seedlist);
        }

        else if &line[0..4] == "seed" {
            s2smap = makemap(findlist(lines.clone().into_iter(), idx));
        }

        else if &line[0..4] ==  "soil" {
            s2fmap = makemap(findlist(lines.clone().into_iter(), idx));
        }

        else if &line[0..5] == "fertil" {
            f2wmap = makemap(findlist(lines.clone().into_iter(), idx));
        }

        else if &line[0..5] == "water" {
            w2lmap = makemap(findlist(lines.clone().into_iter(), idx));
        }

        else if &line[0..4] == "light" {
            l2tmap = makemap(findlist(lines.clone().into_iter(), idx));
        }

        else if &line[0..4] == "temp" {
            t2hmap = makemap(findlist(lines.clone().into_iter(), idx));
        }

        else if &line[0..4] == "humi" {
            h2lmap = makemap(findlist(lines.clone().into_iter(), idx));
        }
    }

    // output the map
    println!("{:?}", seedmap);
}