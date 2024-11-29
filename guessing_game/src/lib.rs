pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
        if value < 1 {
           return Err(format!("Guess value must be greater than or equal to 1, got {value}."));
        } else if value > 100 {
           return Err(format!("Guess value must be less than or equal to 100, got {value}."));
        }

        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
    
}