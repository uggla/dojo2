use dojo2::{calculate_price_formated, convert_currency, Currency, MinreqClient, Percentage};
use url::Url;

const URL: &str = "https://open.er-api.com/v6/latest/EUR";

fn main() {
    println!(
        "{}",
        calculate_price_formated(5, 345, Some(Percentage::new(10.0)))
    );

    match convert_currency(
        20.0,
        Currency::Krupnic,
        MinreqClient(Url::parse(URL).unwrap()),
    ) {
        Ok(krupnic) => println!("{}", krupnic),
        Err(error) => panic!("Error: {:?}", error),
    };

    match convert_currency(20.0, Currency::Usd, MinreqClient(Url::parse(URL).unwrap())) {
        Ok(usd) => println!("{}", usd),
        Err(error) => panic!("Error: {:?}", error),
    };

    // We should panic !
    match convert_currency(
        20.0,
        Currency::Usd,
        MinreqClient(Url::parse("http://localhost").unwrap()),
    ) {
        Ok(usd) => println!("{}", usd),
        Err(error) => panic!("Error: {:?}", error),
    };
}
