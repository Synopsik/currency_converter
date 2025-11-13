use std::env;
use currency_converter::{parse_string, parse_amount, convert};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();


    if args.len() != 3 + 1 {
        println!("Usage: CURRENCY1 CURRENCY2 AMOUNT");
        return
    }

    let cur1 = parse_string(&args[1]);
    let cur2 = parse_string(&args[2]);
    let amount = parse_amount(&args[3]);

    // Doesn't need to be unwrapped for now since we aren't using the returned variable
    convert(cur1, cur2, amount).await;
}