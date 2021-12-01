use std::fs;

struct PolicyAndPassword<'a> {
    min: usize,
    max: usize,
    character: char,
    password: &'a str,
}

fn main() {
    let input = fs::read_to_string("input-02.txt").unwrap();
    let policies_and_passwords = parse_input(&input);
    println!("Part 1: {}", part1(&policies_and_passwords));
    println!("Part 2: {}", part2(&policies_and_passwords));
}

fn part1(policies_and_passwords: &Vec<PolicyAndPassword>) -> u16 {
    let mut valid_count = 0;
    for policy_and_password in policies_and_passwords {
        let char_count: usize = policy_and_password
            .password
            .chars()
            .filter(|c| *c == policy_and_password.character)
            .count();
        if policy_and_password.min <= char_count && char_count <= policy_and_password.max {
            valid_count += 1;
        }
    }
    valid_count
}

fn part2(policies_and_passwords: &Vec<PolicyAndPassword>) -> u16 {
    let mut valid_count = 0;
    for policy_and_password in policies_and_passwords {
        let char_1 = policy_and_password
            .password
            .chars()
            .nth(policy_and_password.min - 1)
            .unwrap();
        let char_2 = policy_and_password
            .password
            .chars()
            .nth(policy_and_password.max - 1)
            .unwrap();
        if (char_1 == policy_and_password.character && char_2 != policy_and_password.character)
            || (char_1 != policy_and_password.character && char_2 == policy_and_password.character)
        {
            valid_count += 1;
        }
    }
    valid_count
}

fn parse_input(input: &String) -> Vec<PolicyAndPassword> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let policy_password: Vec<&str> = line.split(": ").collect();
            let policy = policy_password[0];
            let nums_char: Vec<&str> = policy.split(" ").collect();
            let nums: Vec<usize> = nums_char[0]
                .split("-")
                .map(|x| usize::from_str_radix(x, 10).unwrap())
                .collect();
            PolicyAndPassword {
                min: nums[0],
                max: nums[1],
                character: nums_char[1].chars().next().unwrap_or(' '),
                password: policy_password[1],
            }
        })
        .collect()
}
