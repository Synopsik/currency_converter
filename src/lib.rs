mod exchange;

pub fn parse_string(string: &str) -> Option<&str> {
    if !string.is_empty() {
        Some(string)
    } else {
        eprintln!("Please enter a valid currency abbreviation");
        None
    }
}

pub fn parse_amount(amount: &str) -> Option<f64> {
    let amount = amount.parse::<f64>().ok()?;

    if amount > 0. {
        Some(amount)
    } else {
        eprintln!("Please enter an amount greater than zero");
        None
    }
}


pub async fn convert(
    cur1: Option<&str>,
    cur2: Option<&str>,
    amount: Option<f64>,
) -> Option<f64> {
    // Using cur1 and cur2 strings...
    let _cur1 = cur1?;
    let _cur2 = cur2?;
    let amount = amount?;

    // Return real conversion rates
    let exchange_rate = exchange::get_exchange_rate(_cur1, _cur2).await.unwrap();

    // Convert amount
    let converted = amount * exchange_rate;

    println!(
        "Converting {} {} to {}\nConversion Rate: {:.2}\nAmount: {:.2}",
        &amount,
        &_cur1.to_uppercase(),
        &_cur2.to_uppercase(),
        &exchange_rate,
        &converted,
    );

    Some(converted)
}

#[test]
fn test_parse_string() {
    assert_eq!(parse_string("usd"), Some("usd"));
    assert_eq!(parse_string("cad"), Some("cad"));
    assert_eq!(parse_string("123"), Some("123"));
    assert_eq!(parse_string(""), None);
}

#[test]
fn test_parse_amount() {
    assert_eq!(parse_amount("10.00"), Some(10.));
    assert_eq!(parse_amount("99"), Some(99.));
    assert_eq!(parse_amount("-2"), None);
    assert_eq!(parse_amount("abc"), None);
}