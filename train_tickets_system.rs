use std::collections::HashMap;
use std::io;
fn calculate_discount(age: u32, time: u32, price: &f64) -> Result<f64, &str> {
    let mut discount = 0.0;
    if time < 7 || (time >= 19 && time <= 23) {
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
    let mut i = 0;
    let mut profit = 0.0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();
        if input == "NEXT DESTINATION" {
            break;
        }
        let age: u32 = input[0..input.find('y').unwrap()].parse().unwrap();
        let time: u32 = input[input.len() - 5..input.len() - 3].parse().unwrap();
        let discount = calculate_discount(age, time, &price);
        match discount {
            Ok(discount) => {
                profit += price - discount;
                i += 1;
            }
            Err(_) => (),
        }
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
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim()[..input.len() - 4].to_string();
        let data: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();
        let price: f64 = data[1].parse().unwrap();
        prices.insert(data[0].to_string(), price);
    }
    for (k, v) in prices {
        println!("{}:", k);
        let total_price = calculate_profit(&v);
        profits.insert(k, total_price);
    }
    for (k, v) in profits {
        println!("{} : {}", k, v);
    }
}
