use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Map(Vec<Row>);

impl Map {
    pub fn count_trees(&self, dx: usize, dy: usize) -> usize {
        let mut count = 0;
        let mut j = 0;

        for row in self.rows().step_by(dy).skip(1) {
            j += dx;

            if row.get(j) {
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
                return Some(Self(vec![Row(vec![])]));
            }
        };

        let mut rows = vec![];

        for line in input.lines() {
            if line.len() != col_length {
                return None;
            }

            let mut row = Vec::with_capacity(col_length);
            Self::parse_row_from_str(line, &mut row)?;
            rows.push(Row(row));
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

    fn rows(&self) -> impl Iterator<Item = &Row> {
        self.0.iter()
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_string(s).ok_or(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Row(Vec<bool>);

impl Row {
    fn items(&self) -> impl Iterator<Item = &bool> {
        self.0.iter()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    /// Get the element at `idx`, where the row repeats infinitely.
    fn get(&self, idx: usize) -> bool {
        self.0[idx % self.len()]
    }
}
