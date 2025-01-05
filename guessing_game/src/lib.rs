
#[derive(Debug, PartialEq)]
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guess_in_range() -> Result<(), String> {
        let guess = Guess::new(50)?;
        assert_eq!(guess.value(), 50);
        Ok(())
    }

    #[test]
    fn guess_too_small() {
        let guess = Guess::new(0);
        assert!(guess.is_err());
        assert_eq!(guess.unwrap_err(), "Guess value must be greater than or equal to 1, got 0.");
    }

    #[test]
    fn guess_too_large() {
        let guess = Guess::new(101);
        assert!(guess.is_err());
        assert_eq!(guess.unwrap_err(), "Guess value must be less than or equal to 100, got 101.");
    }
  
    // an alternative way to test the Guess::new function

    #[test]
    fn guess_in_range2() -> Result<(), String> {
        let guess = Guess::new(20)?;
        if guess.value() == 20 {
            Ok(())
        } else {
            return Err(format!("Expected 20, got {}", guess.value()));
        }
    }

}