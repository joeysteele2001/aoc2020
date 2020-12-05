const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1()
}

fn part1() {
    println!("*** PART 1 ***");

    let mut count_valid = 0;
    let mut data = PassportData::new();

    for line in INPUT.lines() {
        // The previous passport data is finished
        if line.is_empty() {
            if data.is_valid() {
                count_valid += 1;
            }

            data = PassportData::new();
            continue;
        }

        data.update_from_str(line);
    }

    // Check the final passport
    if data.is_valid() {
        count_valid += 1;
    }

    println!("{}", count_valid);
}

pub struct PassportData {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl PassportData {
    pub fn new() -> Self {
        Self {
            byr: false,
            iyr: false,
            eyr: false,
            hgt: false,
            hcl: false,
            ecl: false,
            pid: false,
            cid: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }

    pub fn update_from_str(&mut self, s: &str) {
        for keyvalue in s.split_whitespace() {
            let mut kv_iter = keyvalue.split(':');
            let key = kv_iter.next().unwrap();
            let _val = kv_iter.next().unwrap();

            match key {
                "byr" => {
                    self.byr = true;
                }

                "iyr" => {
                    self.iyr = true;
                }

                "eyr" => {
                    self.eyr = true;
                }

                "hgt" => {
                    self.hgt = true;
                }

                "hcl" => {
                    self.hcl = true;
                }

                "ecl" => {
                    self.ecl = true;
                }

                "pid" => {
                    self.pid = true;
                }

                "cid" => {
                    self.cid = true;
                }

                _ => { /* invalid key; do nothing */ }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_passport() {
        let data = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";

        let mut pp_data = PassportData::new();
        pp_data.update_from_str(data);

        assert!(pp_data.is_valid())
    }

    #[test]
    fn test_invalid_passport_1() {
        let data = "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in";

        let mut pp_data = PassportData::new();
        pp_data.update_from_str(data);

        assert!(!pp_data.is_valid())
    }

    #[test]
    fn test_invalid_passport_2() {
        let data = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929";

        let mut pp_data = PassportData::new();
        pp_data.update_from_str(data);

        assert!(!pp_data.is_valid())
    }
}
