use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
    time::Instant,
};

fn main() {
    let content = read_file("./assets/d4-input.txt");
    let lines = split_by_blank(content);

    // let answer = solution_one(&lines);
    let start = Instant::now();
    let answer = solution_two(&lines);
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);

    println!("ok passports: {}", answer);
}

// Solutions
fn solution_one(lines: &Vec<String>) -> usize {
    let mut count = 0;

    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    'outer: for line in lines.iter() {
        let passport = line
            .split_whitespace()
            .map(|x| x.split(":").collect::<Vec<&str>>()[0])
            .collect::<Vec<&str>>();

        for &field in required.iter() {
            if !passport.contains(&field) {
                continue 'outer;
            }
        }

        count += 1;
    }

    return count;
}

fn solution_two(lines: &Vec<String>) -> usize {
    let mut count = 0;

    'outer: for line in lines.iter() {
        // println!("\n");

        let passport = line
            .split_whitespace()
            .map(|s| s.split(':').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        if !has_required(&passport) {
            continue 'outer;
        }

        // println!("\n{:?}", passport);

        if !has_valid_fields(&passport) {
            continue 'outer;
        }

        count += 1;
    }

    return count;
}

// Helpers
fn has_required(pass: &Vec<Vec<&str>>) -> bool {
    let passport = pass.iter().map(|x| x[0]).collect::<Vec<&str>>();

    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for &field in required.iter() {
        if !passport.contains(&field) {
            return false;
        }
    }

    true
}

fn has_valid_fields(pass: &Vec<Vec<&str>>) -> bool {
    for field in pass {
        match field.as_slice() {
            ["byr", x] => {
                let x = x.parse::<i32>().unwrap();

                if x < 1920 || 2002 < x {
                    // println!("invalid birth year");
                    return false;
                }
            }

            ["iyr", x] => {
                let x = x.parse::<i32>().unwrap();

                if x < 2010 || 2020 < x {
                    // println!("invalid issue year");
                    return false;
                }
            }

            ["eyr", x] => {
                let x = x.parse::<i32>().unwrap();

                if x < 2020 || 2030 < x {
                    // println!("invalid expr year");
                    return false;
                }
            }

            ["hgt", x] => {
                let unit = &x[x.len() - 2..];

                let x = match unit {
                    "in" => ("in", x.split_at(x.len() - 2).0.parse::<i32>().unwrap()),
                    "cm" => ("cm", x.split_at(x.len() - 2).0.parse::<i32>().unwrap()),
                    _ => return false,
                };

                match x {
                    ("in", x) => {
                        if x < 59 || 76 < x {
                            // println!("invalid height in");
                            return false;
                        }
                    }

                    ("cm", x) => {
                        if x < 150 || 193 < x {
                            // println!("invalid height cm");
                            return false;
                        }
                    }

                    _ => return false,
                }
            }

            ["hcl", x] => {
                let mut chs = x.chars();

                if x.len() != 7 {
                    // println!("invalid hair length");
                    return false;
                }

                if chs.next().unwrap() != '#' {
                    // println!("invalid hair #");
                    return false;
                }

                for c in chs {
                    if !('a' <= c && c <= 'z') && !('0' <= c && c <= '9') {
                        // println!("invalid hair digits");
                        return false;
                    }
                }
            }

            ["ecl", x] => {
                let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

                if !colors.contains(x) {
                    // println!("invalid eyecolor");
                    return false;
                }
            }

            ["pid", x] => {
                if x.len() != 9 {
                    // println!("invalid pid");
                    return false;
                }
            }

            _ => {}
        }
    }

    true
}

fn split_by_blank(text: String) -> Vec<String> {
    text.trim_end()
        .split("\n\n")
        .map(|x| String::from(x.replace('\n', " ")))
        .collect::<Vec<String>>()
}

fn read_file(path: &str) -> String {
    let content = std::fs::read_to_string(path).expect("Couldn't read file.");

    content.to_string()
}
