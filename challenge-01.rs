fn solve(init: &str, seq: &str) -> String {
    let radix: u32 = 10;

    let mut digits: Vec<u32> = init.chars().map(|c| c.to_digit(radix).unwrap()).collect();

    let mut curr: usize = 0;
    let length: usize = init.len();

    seq.chars().for_each(|c| match c {
        'R' => curr = curr.wrapping_add(1) % length,
        'L' => curr = curr.wrapping_sub(1) % length,
        'U' => digits[curr] = (digits[curr] + 1) % radix,
        'D' => digits[curr] = (digits[curr] + radix - 1) % radix,
        _ => panic!("Error: unknown symbol."),
    });

    digits
        .into_iter()
        .map(|d| d.to_string())
        .collect::<String>()
}

fn main() {
    println!(
        "{}",
        solve("528934712834", "URDURUDRUDLLLLUUDDUDUDUDLLRRRR")
    )
}
