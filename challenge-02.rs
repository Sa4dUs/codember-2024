use std::io::{self, BufRead};

fn solve(code: &str) -> bool {
    let mut flag1: bool = false;
    let mut last_digit: u8 = 0;
    let mut last_char: u8 = 0;

    for c in code.chars() {
        let ascii: u8 = c as u8;
        match ascii {
            48..=57 => {
                if flag1 {
                    return false;
                }

                if ascii < last_digit {
                    return false;
                }

                last_digit = ascii;
            }
            97..=122 => {
                flag1 = true;

                if ascii < last_char {
                    return false;
                }

                last_char = ascii;
            }
            _ => return false,
        }
    }

    return true;
}

fn main() {
    let mut count: [u16; 2] = [0; 2];

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                println!("{}", solve(&line) as usize);
                count[solve(&line) as usize] += 1;
            }
            Err(_) => break,
        }
    }

    println!("{}true{}false", count[1], count[0]);
}
