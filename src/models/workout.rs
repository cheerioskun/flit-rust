use crate::models::exercise::Exercise;
use time::Date;
#[derive(Debug)]
pub struct Workout {
    date: Date,
    exercises: Vec<Exercise>,
}
