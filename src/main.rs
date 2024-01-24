use dojo2::{calculate_price_formated, convert_currency, Currency, Percentage};

fn main() {
    println!(
        "{}",
        calculate_price_formated(5, 345, Some(Percentage::new(10.0)))
    );
    println!("{}", convert_currency(20.0, Currency::Krupnic));
}
