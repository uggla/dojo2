// Remove warnings about unused code this should be removed for production use
#![allow(dead_code)]

fn add(left: usize, right: usize) -> usize {
    left + right
}

fn calculate_price<T: Into<f64>>(quantity: u32, item_price: T, tax_rate: Option<f64>) -> f64 {
    let tax_rate = match tax_rate {
        Some(tax_rate) => tax_rate / 100.0,
        None => 0.0,
    };
    apply_discount(quantity as f64 * item_price.into()) * (1.0 + tax_rate)
}

pub fn calculate_price_formated<T: Into<f64>>(
    quantity: u32,
    item_price: T,
    tax_rate: Option<f64>,
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
}

pub fn convert_currency(euros: f64, currency: Currency) -> String {
    match currency {
        Currency::Krupnic => format!("{:.2} Krupnic", euros * 2.0),
        Currency::Zorglub => format!("{:.2} Zorglub", euros * 3.0),
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
        assert_eq!(calculate_price(2, 10, Some(20.0)), 24.0);
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
        assert_eq!(calculate_price(3, 1.21, Some(5.0)), 3.8115);
    }

    #[test]
    fn test_calculate_price_tc3() {
        assert_eq!(calculate_price(3, 1.21, Some(20.0)), 4.356);
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
            calculate_price_formated(3, 1.21, Some(5.0)),
            String::from("3.81 €")
        );
    }

    #[test]
    fn test_calculate_price_formated_tc3() {
        assert_eq!(
            calculate_price_formated(3, 1.21, Some(20.0)),
            String::from("4.36 €")
        );
    }

    #[test]
    fn test_calculate_price_formated_with_discount_tc1() {
        assert_eq!(
            calculate_price_formated(5, 345, Some(10.0)),
            String::from("1840.58 €")
        );
    }

    #[test]
    fn test_calculate_price_formated_with_discount_tc2() {
        assert_eq!(
            calculate_price_formated(5, 1299, Some(10.0)),
            String::from("6787.28 €")
        );
    }

    #[test]
    fn test_convert_currency_krupnic_tc1() {
        assert_eq!(
            convert_currency(20.0, Currency::Krupnic),
            String::from("40.00 Krupnic")
        );
    }

    #[test]
    fn test_convert_currency_krupnic_tc2() {
        assert_eq!(
            convert_currency(20.0, Currency::Zorglub),
            String::from("60.00 Zorglub")
        );
    }
}
