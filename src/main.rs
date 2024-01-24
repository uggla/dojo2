use dojo2::{calculate_price_formated, convert_currency, Currency};

fn main() {
    println!("{}", calculate_price_formated(5, 345, Some(10.0)));
    println!("{}", convert_currency(20.0, Currency::Krupnic));
}
