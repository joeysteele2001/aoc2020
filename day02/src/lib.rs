use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
struct PasswordPolicy {
    restricted_character: char,
    lower_position: usize,
    upper_position: usize,
}

impl PasswordPolicy {
    pub fn parse_from_string(policy: &str) -> Option<Self> {
        let policy = policy.lines().next()?;
        let mut parts = policy.split_whitespace();

        let times_allowed = parts.next()?;
        let mut nums = times_allowed.split('-').map(|n| n.parse::<usize>());

        let lower_position = nums.next()?.ok()? - 1; // index starting at zero!
        let upper_position = nums.next()?.ok()? - 1;

        let restricted_character = parts.next()?.chars().next()?;

        Some(Self {
            restricted_character,
            lower_position,
            upper_position,
        })
    }

    pub fn is_met_by(&self, password: &str) -> bool {
        let password = password.chars();

        let lower_and_upper_chars = password.clone().nth(self.lower_position).and_then(|c1| {
            let c2 = password.clone().nth(self.upper_position)?;
            Some((c1, c2))
        });

        if let Some((c1, c2)) = lower_and_upper_chars {
            (c1 == self.restricted_character) ^ (c2 == self.restricted_character)
        } else {
            false
        }
    }
}

impl FromStr for PasswordPolicy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_from_string(s).ok_or(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PasswordDBEntry {
    policy: PasswordPolicy,
    password: String,
}

impl PasswordDBEntry {
    pub fn parse_from_string(entry: &str) -> Option<Self> {
        let entry = entry.lines().next()?;
        let mut parts = entry.split(':');

        let policy: PasswordPolicy = parts.next()?.parse().ok()?;
        let password = parts.next()?.trim().to_string();

        Some(Self { policy, password })
    }

    pub fn is_valid(&self) -> bool {
        self.policy.is_met_by(&self.password)
    }
}

impl FromStr for PasswordDBEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_from_string(s).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_met_by1() {
        let policy = PasswordPolicy {
            restricted_character: 'a',
            lower_position: 0,
            upper_position: 2,
        };

        assert!(policy.is_met_by("abcde"))
    }

    #[test]
    fn test_is_met_by2() {
        let policy = PasswordPolicy {
            restricted_character: 'b',
            lower_position: 0,
            upper_position: 2,
        };

        assert!(!policy.is_met_by("cdefg"))
    }

    #[test]
    fn test_is_met_by3() {
        let policy = PasswordPolicy {
            restricted_character: 'c',
            lower_position: 1,
            upper_position: 8,
        };

        assert!(!policy.is_met_by("ccccccccc"))
    }

    #[test]
    fn test_parse_policy1() {
        let policy_str = "1-3 a";

        assert_eq!(
            policy_str.parse::<PasswordPolicy>().unwrap(),
            PasswordPolicy {
                restricted_character: 'a',
                lower_position: 0,
                upper_position: 2,
            }
        )
    }

    #[test]
    fn test_parse_policy2() {
        let policy_str = "1-3 b: cdefg";

        assert_eq!(
            policy_str.parse::<PasswordPolicy>().unwrap(),
            PasswordPolicy {
                restricted_character: 'b',
                lower_position: 0,
                upper_position: 2,
            }
        )
    }

    #[test]
    fn test_parse_policy3() {
        let policy_str = "2-9 c: cccccccc\n1-3 b: cdefg";

        assert_eq!(
            policy_str.parse::<PasswordPolicy>().unwrap(),
            PasswordPolicy {
                restricted_character: 'c',
                lower_position: 1,
                upper_position: 8,
            }
        )
    }

    #[test]
    fn test_parse_entry1() {
        let entry_str = "1-3 a: abcde";

        let expected_policy = PasswordPolicy {
            restricted_character: 'a',
            lower_position: 0,
            upper_position: 2,
        };

        assert_eq!(
            entry_str.parse::<PasswordDBEntry>().unwrap(),
            PasswordDBEntry {
                policy: expected_policy,
                password: "abcde".to_string(),
            }
        )
    }

    #[test]
    fn test_parse_entry2() {
        let entry_str = "1-3 b: cdefg";

        let expected_policy = PasswordPolicy {
            restricted_character: 'b',
            lower_position: 0,
            upper_position: 2,
        };

        assert_eq!(
            entry_str.parse::<PasswordDBEntry>().unwrap(),
            PasswordDBEntry {
                policy: expected_policy,
                password: "cdefg".to_string(),
            }
        )
    }

    #[test]
    fn test_parse_entry3() {
        let entry_str = "2-9 c: ccccccccc\n1-3 a: abcde\n";

        let expected_policy = PasswordPolicy {
            restricted_character: 'c',
            lower_position: 1,
            upper_position: 8,
        };

        assert_eq!(
            entry_str.parse::<PasswordDBEntry>().unwrap(),
            PasswordDBEntry {
                policy: expected_policy,
                password: "ccccccccc".to_string(),
            }
        )
    }

    #[test]
    fn test_valid1() {
        let entry = "1-3 a: abcde".parse::<PasswordDBEntry>().unwrap();

        assert!(entry.is_valid())
    }

    #[test]
    fn test_valid2() {
        let entry = "1-3 b: cdefg".parse::<PasswordDBEntry>().unwrap();

        assert!(!entry.is_valid())
    }

    #[test]
    fn test_valid3() {
        let entry = "2-9 c: cccccccc\n1-3 a: abcde\n"
            .parse::<PasswordDBEntry>()
            .unwrap();

        assert!(!entry.is_valid())
    }
}
