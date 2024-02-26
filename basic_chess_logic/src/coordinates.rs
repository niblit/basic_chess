use std::collections::HashMap;

pub struct Coordinates {
    column: usize,
    row: usize,
}

impl Coordinates {
    pub fn new(column: i32, row: i32) -> Option<Coordinates> {
        if Coordinates::is_within_range(column) && Coordinates::is_within_range(row) {
            return Some(Coordinates {
                column: column as usize,
                row: row as usize,
            });
        }
        None
    }

    fn is_within_range(value: i32) -> bool {
        return 0 <= value && value <= 7;
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn to_notation(&self) -> String {
        let column_to_letter: HashMap<usize, &str> = HashMap::from([
            (0, "a"),
            (1, "b"),
            (2, "c"),
            (3, "d"),
            (4, "e"),
            (5, "f"),
            (6, "g"),
            (7, "h"),
        ]);

        format!("{}{}", column_to_letter[&self.column], self.row + 1)
    }
}
