#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Position {
    pub row: i32,
    pub column: i32,
}

impl Position {
    pub fn new(row: i32, column: i32) -> Self {
        Self { row, column }
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    data: Vec<char>,
    rows: i32,
    columns: i32,
}

impl Grid {
    /// Create new grid from string formatted pattern
    pub fn new(content: &str) -> Self {
        let data: Vec<_> = content
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let rows = data.len() as i32;
        let columns = data[0].len() as i32;
        let data: Vec<char> = data.into_iter().flatten().collect();
        Self {
            data,
            rows,
            columns,
        }
    }

    fn get_index(&self, position: Position) -> Result<usize, &'static str> {
        if !(0..self.rows).contains(&position.row) {
            return Err("row is off grid");
        }
        if !(0..self.columns).contains(&position.column) {
            return Err("column is off grid");
        }
        let index = (self.columns * position.row + position.column) as usize;
        if !(0..self.data.len()).contains(&index) {
            return Err("data does not contain calculated index");
        }
        Ok(index)
    }

    fn get_position(&self, index: usize) -> Position {
        let row = index as i32 / self.columns;
        let column = index as i32 % self.columns;
        Position::new(row, column)
    }

    /// Retrieve value of position
    pub fn get(&self, row: i32, column: i32) -> Option<char> {
        let position = Position::new(row, column);
        let index = match self.get_index(position) {
            Ok(index) => index,
            Err(_) => return None,
        };
        Some(self.data[index])
    }

    /// Update value of position
    pub fn set(&mut self, row: i32, column: i32, value: char) -> Result<(), &str> {
        let position = Position::new(row, column);
        let index = match self.get_index(position) {
            Ok(index) => index,
            Err(_) => return Err("can't set value off grid"),
        };
        self.data[index] = value;
        Ok(())
    }

    /// Get position of value (only first one)
    pub fn position(&self, value: char) -> Option<Position> {
        let position = match self.data.iter().position(|&x| x == value) {
            Some(index) => self.get_position(index),
            None => return None,
        };
        Some(position)
    }

    /// Get all positions of value
    pub fn positions(&self, value: char) -> Vec<Position> {
        let mut iterator = self.data.iter();
        let mut positions = Vec::new();
        let mut last_index = 0;
        loop {
            let index = match iterator.position(|&x| x == value) {
                Some(index) => index,
                None => break,
            };
            last_index += index;
            let position = self.get_position(last_index);
            last_index += 1;
            positions.push(position);
        }
        positions
    }

    pub fn contains(&self, position: &Position) -> bool {
        if position.row < 0 || position.row >= self.rows {
            return false;
        }
        if position.column < 0 || position.column >= self.columns {
            return false;
        }
        true
    }
}
