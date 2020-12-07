fn main() {
    part1()
}

fn part1() {
    const INPUT: &str = include_str!("../input.txt");

    println!("*** PART 1 ***");

    let max_id = INPUT.lines().map(decode_boarding_pass).max().unwrap();

    println!("{}", max_id);
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
