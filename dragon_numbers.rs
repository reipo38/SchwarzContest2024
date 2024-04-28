use std::io;

fn permute(nums: &mut Vec<char>) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = Vec::new();
    let x = nums.len() - 1;
    permutations(&mut res, nums, 0, x);
    res
}

fn swap(nums: &mut Vec<char>, l: usize, h: usize) {
    let temp = nums[l];
    nums[l] = nums[h];
    nums[h] = temp;
}

fn permutations(res: &mut Vec<Vec<char>>, nums: &mut Vec<char>, l: usize, h: usize) {
    if l == h {
        res.push(nums.clone());
    } else {
        for i in l..=h {
            swap(nums, l, i);
            permutations(res, nums, l + 1, h);
            swap(nums, l, i);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let num_string = input.trim().to_string();
    let num_int:i32 = num_string.parse().unwrap();
    let mut num_vec: Vec<char> = num_string.chars().collect();
    let mut is_dragon = false;
    let mut is_pseudodragon = false;
    if num_int >= 100 {
        let permutations: Vec<Vec<char>> = permute(&mut num_vec);
        for i in 0..permutations.len()-1 {
            let num:String = permutations[i].clone().into_iter().collect();
            let left_num:i32;
            let right_num:i32;
            if num.len() % 2 == 0 {
                left_num = num[0..num.len()/2].parse().unwrap();
                right_num = num[num.len()/2..num.len()].parse().unwrap();
                if left_num * right_num == num_int {
                    is_dragon = true;
                }
            }
            else if num.len() > 2 {
                if i % 2 == 0 {
                    left_num = num[0..num.len()/2 + 1].parse().unwrap();
                    right_num = num[num.len()/2 + 1..num.len()].parse().unwrap()
                } else {
                    left_num = num[0..num.len()/2].parse().unwrap();
                    right_num = num[num.len()/2..num.len()].parse().unwrap();
                }
                if left_num * right_num == num_int {
                    is_pseudodragon = true;
                }
            }
        }
    }
    if is_dragon {
        println!("True Dragon")
    } else if is_pseudodragon {
        println!("Pseudodragon")
    } else {
        println!("Normal number")
    }
}