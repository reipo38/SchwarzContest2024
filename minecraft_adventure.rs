use std::io;
fn main() {
    let mut num_sticks = 0;
    let mut num_stone = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();
        if input == "Sticks" {
            num_sticks += 1;
        }
        if input == "Wood" {
            num_sticks += 4;
        }
        if input == "Stone" {
            num_stone += 1;
        }
        if input == "END" {
            break;
        }
    }
    if num_sticks >= 2 && num_stone >= 3 {
        num_sticks /= 2;
        num_stone /= 3;
        println!("{}", if num_sticks < num_stone {num_sticks} else {num_stone})
    } else {
        println!("{}", 0)
    }
}
