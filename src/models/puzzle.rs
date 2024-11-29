use log::error;
use validator::Validate;

#[derive(Validate)]
pub struct Puzzle {
    #[validate(range(min = 2015, max = 2024))]
    year: u16,
    #[validate(range(min = 1, max = 25))]
    day: u8,
}

impl Puzzle {
    pub fn new(year: u16, day: u8) -> Option<Puzzle> {
        let puzzle = Puzzle { year, day };
        match puzzle.validate() {
            Ok(_) => Some(puzzle),
            Err(e) => {
                for (field, errors) in e.field_errors() {
                    error!("validation failed for {field}: {:#?}", errors);
                }
                return None;
            }
        }
    }
}
