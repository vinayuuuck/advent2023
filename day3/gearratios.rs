
use std::io;
use std::fs;

fn checkindices(matrix: &Vec<Vec<char>>, indices: &Vec<(usize, usize)>) -> bool {
    // check if at any of the indices, there is a symbol which is not a dot or a number
    for (i, j) in indices {
        if (matrix[*i][*j] != '.' && !matrix[*i][*j].is_numeric()) {
            return true;
        }
    }

    return false;
}


fn main() {

    let mut total = 0;

    let mut inputfile = String::new();
    println!("Enter the path of the input file: ");
    io::stdin().read_line(&mut inputfile).expect("Failed to read line");

    let content = fs::read_to_string(inputfile.trim()).expect("Something went wrong reading the file");
    let lines = content.split("\n");

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        matrix.push(row);
    }

    for i in 0..matrix.len() {
        // go through the row
        // if num is encountered, check till the end of the number and so we obtain the number to check for
        // add the surrounding indices to the indicestocheck vector which will be (i, startnum-1), (i, endnum+1), (i-1, startnum), (i+1, startnum), (i-1, endnum), (i+1, endnum), (i-1, startnum-1), (i+1, startnum-1), (i-1, endnum+1), (i+1, endnum+1)
        // if num is not encountered, continue
        // if num passes check, add it to totaxl
        // add required checks if the indices are out of bounds

        let mut indicestocheck: Vec<(usize, usize)> = Vec::new();
        let mut startnum = 0;
        let mut endnum = 0;

        let mut j = 0;
        while j < matrix[i].len() {
            if matrix[i][j].is_numeric() {
                startnum = j;
                while j < matrix[i].len() && matrix[i][j].is_numeric() {
                    j += 1;
                }
                endnum = j-1;
                if i==0 {
                    if startnum == 0 {
                        indicestocheck.push((i+1, startnum+1));
                        indicestocheck.push((i, endnum+1));
                        indicestocheck.push((i+1, endnum));
                        indicestocheck.push((i+1, endnum+1));
                    }
                    else if endnum == matrix[i].len()-1 {
                        indicestocheck.push((i, startnum-1));
                        indicestocheck.push((i+1, startnum));
                        indicestocheck.push((i+1, startnum-1));
                        indicestocheck.push((i+1, startnum+1));
                        indicestocheck.push((i+1, endnum-1));
                        indicestocheck.push((i+1, endnum));
                    }
                    else {
                        indicestocheck.push((i, startnum-1));
                        indicestocheck.push((i, startnum+1));
                        indicestocheck.push((i, endnum+1));
                        indicestocheck.push((i+1, startnum));
                        indicestocheck.push((i+1, startnum-1));
                        indicestocheck.push((i+1, startnum+1));
                        indicestocheck.push((i+1, endnum));
                        indicestocheck.push((i+1, endnum+1));
                    }
                }
                else if i==matrix.len()-1 {
                    if startnum == 0 {
                        indicestocheck.push((i-1, startnum));
                        indicestocheck.push((i-1, startnum+1));
                        indicestocheck.push((i, endnum+1));
                        indicestocheck.push((i-1, endnum));
                        indicestocheck.push((i-1, endnum+1));
                    }
                    else if endnum == matrix[i].len()-1 {
                        indicestocheck.push((i, startnum-1));
                        indicestocheck.push((i-1, startnum));
                        indicestocheck.push((i-1, startnum-1));
                        indicestocheck.push((i-1, startnum+1));
                        indicestocheck.push((i-1, endnum-1));
                        indicestocheck.push((i-1, endnum));
                    }
                    else {
                        indicestocheck.push((i, startnum-1));
                        indicestocheck.push((i, endnum+1));
                        indicestocheck.push((i-1, startnum));
                        indicestocheck.push((i-1, startnum-1));
                        indicestocheck.push((i-1, startnum+1));
                        indicestocheck.push((i-1, endnum));
                        indicestocheck.push((i-1, endnum+1));
                        indicestocheck.push((i-1, endnum-1));
                    }
                }
                else {
                    if startnum == 0 {
                        indicestocheck.push((i-1, startnum));
                        indicestocheck.push((i+1, startnum));
                        indicestocheck.push((i, startnum+1));
                        indicestocheck.push((i-1, startnum+1));
                        indicestocheck.push((i+1, startnum+1));
                        indicestocheck.push((i, endnum+1));
                        indicestocheck.push((i-1, endnum));
                        indicestocheck.push((i+1, endnum));
                        indicestocheck.push((i-1, endnum+1));
                        indicestocheck.push((i+1, endnum+1));
                    }
                    else if endnum == matrix[i].len()-1 {
                        indicestocheck.push((i, startnum-1));
                        indicestocheck.push((i-1, startnum));
                        indicestocheck.push((i+1, startnum));
                        indicestocheck.push((i-1, startnum-1));
                        indicestocheck.push((i+1, startnum-1));
                        indicestocheck.push((i-1, endnum));
                        indicestocheck.push((i+1, endnum));
                        indicestocheck.push((i-1, endnum-1));
                        indicestocheck.push((i+1, endnum-1));
                    }
                    else {
                        indicestocheck.push((i, startnum-1));
                        indicestocheck.push((i, endnum+1));
                        indicestocheck.push((i-1, startnum));
                        indicestocheck.push((i+1, startnum));
                        indicestocheck.push((i-1, endnum));
                        indicestocheck.push((i+1, endnum));
                        indicestocheck.push((i-1, startnum-1));
                        indicestocheck.push((i+1, startnum-1));
                        indicestocheck.push((i-1, endnum+1));
                        indicestocheck.push((i+1, endnum+1));
                        indicestocheck.push((i-1, startnum+1));
                        indicestocheck.push((i+1, startnum+1));
                        indicestocheck.push((i-1, endnum-1));
                        indicestocheck.push((i+1, endnum-1));
                    }
                }

                if checkindices(&matrix, &indicestocheck) {
                    // print number we are checking
                    println!("{}", matrix[i][startnum..endnum+1].iter().collect::<String>().parse::<i32>().unwrap());
                    total += matrix[i][startnum..endnum+1].iter().collect::<String>().parse::<i32>().unwrap();
                }
                indicestocheck.clear();
            }
            j += 1;
        }
    }

    println!("Total: {}", total);
}