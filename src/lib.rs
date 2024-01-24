// Remove warnings about unused code this should be removed for production use
#![allow(dead_code)]

use serde_json::Value;
use url::Url;

fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Percentage(f64);

impl Percentage {
    pub fn new(value: impl Into<f64>) -> Self {
        let value = value.into();
        if !(0.0..=100.0).contains(&value) {
            panic!("Percentage must be between 0 and 100");
        }
        Self(value)
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

fn calculate_price<T: Into<f64>>(
    quantity: u32,
    item_price: T,
    tax_rate: Option<Percentage>,
) -> f64 {
    let tax_rate = match tax_rate {
        Some(tax_rate) => tax_rate.0 / 100.0,
        None => 0.0,
    };
    apply_discount(quantity as f64 * item_price.into()) * (1.0 + tax_rate)
}

pub fn calculate_price_formated<T: Into<f64>>(
    quantity: u32,
    item_price: T,
    tax_rate: Option<Percentage>,
) -> String {
    format!("{:.2} €", calculate_price(quantity, item_price, tax_rate))
}

fn apply_discount(price: f64) -> f64 {
    if price > 5000.0 {
        price * (1.0 - 5.0 / 100.0)
    } else if price > 1000.0 {
        price * (1.0 - 3.0 / 100.0)
    } else {
        price
    }
}

pub enum Currency {
    Krupnic,
    Zorglub,
    Usd,
}

#[derive(Debug)]
pub enum Error {
    RequestFailed,
    ValueNotFound,
    JsonParsingError(minreq::Error),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        // Clippy would rather use the matches! macro but allow standard
        // pattern matching to no confuse newcomers.
        #[allow(clippy::match_like_matches_macro)]
        match (self, other) {
            (Error::RequestFailed, Error::RequestFailed) => true,
            (Error::ValueNotFound, Error::ValueNotFound) => true,
            (Error::JsonParsingError(_), Error::JsonParsingError(_)) => true,
            _ => false,
        }
    }
}

pub trait HttpClient {
    fn get_usd(&self) -> Result<f64, Error>;
}

struct FakeClient(Url);

impl HttpClient for FakeClient {
    fn get_usd(&self) -> Result<f64, Error> {
        Ok(1.2)
    }
}

pub struct MinreqClient(pub Url);

impl HttpClient for MinreqClient {
    fn get_usd(&self) -> Result<f64, Error> {
        // TODO:
        // Clearly this should be refactor as we can not test if the currency is not found or json
        // parsing error
        let url: minreq::URL = self.0.to_string();
        let currency = minreq::get(url)
            .send()
            .map_err(|_| Error::RequestFailed)?
            .json::<Value>()
            .map_err(Error::JsonParsingError)?;
        let rate = currency["rates"]["USD"]
            .as_f64()
            .ok_or(Error::ValueNotFound)?;
        dbg!(rate);
        Ok(rate)
    }
}

pub fn convert_currency(
    euros: f64,
    currency: Currency,
    httpclient: impl HttpClient,
) -> Result<String, Error> {
    match currency {
        Currency::Krupnic => Ok(format!("{:.2} Krupnic", euros * 2.0)),
        Currency::Zorglub => Ok(format!("{:.2} Zorglub", euros * 3.0)),
        Currency::Usd => Ok(format!("{:.2} $", euros * httpclient.get_usd()?)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_calculate_price_with_taxe() {
        assert_eq!(calculate_price(2, 10, Some(Percentage::new(20.0))), 24.0);
    }

    #[test]
    fn test_calculate_price_without_taxe() {
        assert_eq!(calculate_price(2, 10, None), 20.0);
    }

    #[test]
    fn test_calculate_price_tc1() {
        assert_eq!(calculate_price(3, 1.21, None), 3.63);
    }

    #[test]
    fn test_calculate_price_tc2() {
        assert_eq!(calculate_price(3, 1.21, Some(Percentage::new(5.0))), 3.8115);
    }

    #[test]
    fn test_calculate_price_tc3() {
        assert_eq!(calculate_price(3, 1.21, Some(Percentage(20.0))), 4.356);
    }

    #[test]
    fn test_calculate_price_formated_tc1() {
        assert_eq!(
            calculate_price_formated(3, 1.21, None),
            String::from("3.63 €")
        );
    }

    #[test]
    fn test_calculate_price_formated_tc2() {
        assert_eq!(
            calculate_price_formated(3, 1.21, Some(Percentage(5.0))),
            String::from("3.81 €")
        );
    }

    #[test]
    fn test_calculate_price_formated_tc3() {
        assert_eq!(
            calculate_price_formated(3, 1.21, Some(Percentage(20.0))),
            String::from("4.36 €")
        );
    }

    #[test]
    fn test_calculate_price_formated_with_discount_tc1() {
        assert_eq!(
            calculate_price_formated(5, 345, Some(Percentage(10.0))),
            String::from("1840.58 €")
        );
    }

    #[test]
    fn test_calculate_price_formated_with_discount_tc2() {
        assert_eq!(
            calculate_price_formated(5, 1299, Some(Percentage(10.0))),
            String::from("6787.28 €")
        );
    }

    #[test]
    fn test_convert_currency_krupnic_tc1() {
        assert_eq!(
            convert_currency(
                20.0,
                Currency::Krupnic,
                FakeClient(Url::parse("http://localhost").unwrap())
            )
            .unwrap(),
            String::from("40.00 Krupnic")
        );
    }

    #[test]
    fn test_convert_currency_krupnic_tc2() {
        assert_eq!(
            convert_currency(
                20.0,
                Currency::Zorglub,
                FakeClient(Url::parse("http://localhost").unwrap())
            )
            .unwrap(),
            String::from("60.00 Zorglub")
        );
    }

    #[test]
    fn test_convert_currency_usd_fake_client() {
        assert_eq!(
            convert_currency(
                20.0,
                Currency::Usd,
                FakeClient(Url::parse("http://localhost").unwrap())
            )
            .unwrap(),
            String::from("24.00 $")
        );
    }

    #[test]
    fn test_convert_currency_usd2() {
        // This test is not good because it depends on an external service
        // but this is only to show how it works
        let dollars = convert_currency(
            25.0,
            Currency::Usd,
            MinreqClient(Url::parse("https://open.er-api.com/v6/latest/EUR").unwrap()),
        )
        .unwrap();
        assert_eq!(dollars.ends_with(" $"), true);
        assert_eq!(dollars.starts_with("2"), true);
    }

    #[test]
    fn test_convert_currency_connection_failed() {
        assert_eq!(
            convert_currency(
                20.0,
                Currency::Usd,
                MinreqClient(Url::parse("https://localhost/currency").unwrap())
            )
            .unwrap_err(),
            Error::RequestFailed
        );
    }

    #[test]
    #[should_panic(expected = "Percentage must be between 0 and 100")]
    fn test_percentage_boundary_high() {
        Percentage::new(200.0);
    }

    #[test]
    #[should_panic(expected = "Percentage must be between 0 and 100")]
    fn test_percentage_boundary_low() {
        Percentage::new(-20.0);
    }

    #[test]
    fn test_percentage_various_type1() {
        assert_eq!(Percentage::new(20).get(), 20.0);
    }

    #[test]
    fn test_percentage_various_type2() {
        assert_eq!(Percentage::new(20.0).get(), 20.0);
    }
}
