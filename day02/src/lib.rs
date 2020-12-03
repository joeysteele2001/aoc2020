use std::ops;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PasswordPolicy {
    pub restricted_character: char,
    pub times_allowed: ops::RangeInclusive<usize>,
}

impl PasswordPolicy {
    pub fn parse_from_string(policy: &str) -> Option<Self> {
        let policy = policy.lines().next()?;
        let mut parts = policy.split_whitespace();

        let times_allowed = parts.next()?;
        let mut nums = times_allowed.split('-').map(|n| n.parse());

        let min = nums.next()?.ok()?;
        let max = nums.next()?.ok()?;

        let times_allowed = min..=max;

        let restricted_character = parts.next()?.chars().next()?;

        Some(Self {
            restricted_character,
            times_allowed,
        })
    }

    pub fn is_met_by(&self, password: &str) -> bool {
        let num_restricted_char = password
            .chars()
            .filter(|&c| c == self.restricted_character)
            .count();

        self.times_allowed.contains(&num_restricted_char)
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
    password: String
}

impl PasswordDBEntry {
    pub fn parse_from_string(entry: &str) -> Option<Self> {
        let entry = entry.lines().next()?;
        let mut parts = entry.split(':');

        let policy: PasswordPolicy = parts.next()?.parse().ok()?;
        let password = parts.next()?.trim().to_string();

        Some(Self {
            policy,
            password,
        })
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
            times_allowed: 1..=3,
        };

        assert!(policy.is_met_by("abcde"))
    }

    #[test]
    fn test_is_met_by2() {
        let policy = PasswordPolicy {
            restricted_character: 'b',
            times_allowed: 1..=3,
        };

        assert!(!policy.is_met_by("cdefg"))
    }

    #[test]
    fn test_is_met_by3() {
        let policy = PasswordPolicy {
            restricted_character: 'c',
            times_allowed: 2..=9,
        };

        assert!(policy.is_met_by("ccccccccc"))
    }

    #[test]
    fn test_parse_policy1() {
        let policy_str = "1-3 a";

        assert_eq!(
            policy_str.parse::<PasswordPolicy>().unwrap(),
            PasswordPolicy {
                restricted_character: 'a',
                times_allowed: 1..=3,
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
                times_allowed: 1..=3,
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
                times_allowed: 2..=9,
            }
        )
    }

    #[test]
    fn test_parse_entry1() {
        let entry_str = "1-3 a: abcde";

        let expected_policy = PasswordPolicy {
            restricted_character: 'a',
            times_allowed: 1..=3,
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
            times_allowed: 1..=3,
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
            times_allowed: 2..=9,
        };

        assert_eq!(
            entry_str.parse::<PasswordDBEntry>().unwrap(),
            PasswordDBEntry {
                policy: expected_policy,
                password: "ccccccccc".to_string(),
            }
        )
    }
}
