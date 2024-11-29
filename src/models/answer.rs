use log::error;
use validator::Validate;

#[derive(Validate)]
pub struct Answer {
    #[validate(range(min = 1, max = 2))]
    level: u8,
    value: String,
}

impl Answer {
    pub fn new(level: u8, value: String) -> Option<Answer> {
        let answer = Answer { level, value };
        match answer.validate() {
            Ok(_) => Some(answer),
            Err(e) => {
                for (field, errors) in e.field_errors() {
                    error!("validation failed for {field}: {:#?}", errors);
                }
                return None;
            }
        }
    }
}
