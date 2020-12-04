use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Map(Vec<Vec<bool>>);

impl Map {
    pub fn count_trees(&self, dx: usize, dy: usize) -> usize {
        let mut count = 0;
        let mut j = 0;

        for row in self.rows().step_by(dy).skip(1) {
            j += dx;

            if row[j % self.n_cols()] {
                // This is a tree
                count += 1;
            }
        }

        return count;
    }

    pub fn parse_string(input: &str) -> Option<Self> {
        let col_length = match input.lines().next() {
            Some(line) => line.len(),
            None => {
                // This is an empty string, so return an empty map
                return Some(Self(vec![vec![]]));
            }
        };

        let mut rows = vec![];

        for line in input.lines() {
            if line.len() != col_length {
                return None;
            }

            let mut row = Vec::with_capacity(col_length);
            Self::parse_row_from_str(line, &mut row)?;
            rows.push(row);
        }

        Some(Self(rows))
    }

    #[must_use]
    fn parse_row_from_str(s: &str, row: &mut Vec<bool>) -> Option<()> {
        let chars = s.lines().next()?.chars();

        for c in chars {
            let is_tree = match c {
                '.' => false,
                '#' => true,
                _ => {
                    // Invalid character
                    return None;
                }
            };

            row.push(is_tree);
        }

        Some(())
    }

    fn rows(&self) -> impl Iterator<Item = &Vec<bool>> + '_ {
        self.0.iter()
    }

    fn n_rows(&self) -> usize {
        self.0.len()
    }

    fn n_cols(&self) -> usize {
        self.0.get(0).map(Vec::len).unwrap_or(0)
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_string(s).ok_or(())
    }
}
