use std::io;
fn main() {
    let mut num_sticks = 0;
    let mut num_stone = 0;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let mut input_string = input.trim().to_string();
    while input_string != "END" {
        if input_string == "Sticks" {
            num_sticks += 1;
        }
        if input_string == "Wood" {
            num_sticks += 4;
        }
        if input_string == "Stone" {
            num_stone += 1;
        }
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        input_string = input.trim().to_string();
    }
    if num_sticks >= 2 && num_stone >= 3 {
        num_sticks /= 2;
        num_stone /= 3;
        println!("{}", if num_sticks < num_stone {num_sticks} else {num_stone})
    } else {
        println!("{}", 0)
    }
}
