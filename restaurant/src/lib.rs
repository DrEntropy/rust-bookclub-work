    //! This is a doc comment
    //! front of house module

mod front_of_house;


mod back_of_house;


use crate::front_of_house::hosting;
use crate::back_of_house::Breakfast;

/// eat_at_restaurant
/// 
/// # Examples (fake)
/// ```
/// let mut list = Vec::new();
/// list.push("John");
/// restaurant::eat_at_restaurant();
/// 
/// assert_eq!(list.len(), 1);
/// ```
/// 

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    hosting::add_to_waitlist();

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    //meal.seasonal_fruit = String::from("blueberries");
}


 