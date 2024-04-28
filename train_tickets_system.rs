use std::collections::HashMap;
use std::io;

fn calculate_discount(age: u32, time: u32, price: &f64) -> Result<f64, &str> {
    let mut discount = 0.0;
    if (time >= 0 && time < 7) || (time >= 19 && time <= 23) {
        discount += 0.05;
    }
    if age >= 7 && age < 12 {
        discount += 0.2;
    } else if age >= 12 && age < 18 || age > 64 {
        discount += 0.1;
    }
    if age > 6 { Ok(discount * price) } else { Err("") }
}

fn calculate_profit(price: &f64) -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut i = 0;
    let mut profit = 0.0;
    let mut input_clone = input.clone().trim().to_string();
    while input_clone != "NEXT DESTINATION".to_string() {
        let age: u32 = input[0..input.find('y').unwrap()].parse().unwrap();
        let time: u32 = input[input.len() - 6..input.len() - 4].parse().unwrap();
        let discount = calculate_discount(age, time, &price);
        match discount {
            Ok(discount) => {
                profit += price - discount;
                i += 1;
            }
            Err(_) => (),
        }
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        input_clone = input.clone().trim().to_string();
    }
    if i >= 4 {
        profit -= price * 0.05 * i as f64;
    }
    profit
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();
    let mut prices = HashMap::new();
    let mut profits = HashMap::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let input_clone = input.clone();
        let trimmed_input = input_clone.trim().to_string();
        let trimmed_input = trimmed_input[..trimmed_input.len() - 3].to_string();

        let data: Vec<String> = trimmed_input.split(" ").map(|s| s.to_string()).collect();
        let price: f64 = data[1].parse().unwrap();
        prices.insert(data[0].to_string(), price);
    }
    for (k, v) in &prices {
        println!("{}:", k);
        let total_price = calculate_profit(&v);
        profits.insert(k, total_price);
    }
    for (k, v) in &profits {
        println!("{} : {}", k, v);
    }
}
