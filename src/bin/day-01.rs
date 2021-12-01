use std::fs;

fn main() {
    let contents = fs::read_to_string("input-01.txt").expect("Could not read input-01.txt");
    let numbers: Vec<u32> = contents
        .split("\n")
        .map(|x| u32::from_str_radix(x, 10).unwrap_or(0))
        .collect();

    println!("Part 1: {}", part1(&numbers));

    println!("Part 2: {}", part2(&numbers));
}

fn part1(numbers: &Vec<u32>) -> u32 {
    for num1 in numbers {
        for num2 in numbers {
            if num1 + num2 == 2020 {
                return num1 * num2;
            }
        }
    }
    0
}

fn part2(numbers: &Vec<u32>) -> u32 {
    for num1 in numbers {
        for num2 in numbers {
            for num3 in numbers {
                if num1 + num2 + num3 == 2020 {
                    return num1 * num2 * num3;
                }
            }
        }
    }
    0
}
