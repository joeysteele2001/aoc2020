const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("*** PART 1 ***");

    let max_id = INPUT.lines().map(decode_boarding_pass).max().unwrap();

    println!("{}", max_id);
}

fn part2() {
    // If every number within our 11-bit range is present, except for one, then
    // bitwise XORing each id together will give us the missing number.
    // This works because XORing tells us whether an even or odd number of bits are set over a given column.

    println!("*** PART 2 ***");

    let missing_pass = INPUT
        .lines()
        .map(decode_boarding_pass)
        .fold(0, |acc, id| acc ^ id);

    println!("{}", missing_pass);
}

fn decode_boarding_pass(pass: &str) -> u16 {
    let mut seat_id = 0;

    // Very hacky way of doing this. This is very bad.
    for i in 0..7 {
        let next_bit = match pass.chars().nth(i).unwrap() {
            'F' => 0,
            'B' => 1,
            x => panic!("invalid letter {}", x),
        };

        seat_id <<= 1;
        seat_id |= next_bit;
    }

    for i in 7..10 {
        let next_bit = match pass.chars().nth(i).unwrap() {
            'L' => 0,
            'R' => 1,
            x => panic!("invalid letter {}", x),
        };

        seat_id <<= 1;
        seat_id |= next_bit;
    }

    seat_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        let pass = "FBFBBFFRLR";
        assert_eq!(decode_boarding_pass(pass), 357);
    }
}
