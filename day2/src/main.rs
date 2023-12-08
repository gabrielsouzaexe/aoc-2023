use std::cmp;
use std::fs;

fn main() {
    let binding: String = fs::read_to_string("input.txt").expect("Should be able to read file");

    part1(binding.clone());
    part2(binding.clone());
}

fn part1(binding: String) {
    let games = binding.split("\n").into_iter();

    let mut count = 0;

    for game in games {
        let (left, right): (&str, &str) = game.split_at(game.rfind(":").unwrap());

        let right = right.strip_prefix(":").unwrap();
        let id: i64 = left.split(" ").into_iter().last().unwrap().parse().unwrap();

        right.split("; ").all(|bag| is_valid(&bag)).then(|| {
            count += id;
        });
    }

    println!("Result - {}", count)
}

fn part2(binding: String) {
    let mut sum: i32 = 0;

    for game in binding.lines() {
        let mut red_min_value: i32 = i32::default();
        let mut blue_min_value: i32 = i32::default();
        let mut green_min_value: i32 = i32::default();

        let (_, right) = game.split_at(game.rfind(":").unwrap());
        let right = right.strip_prefix(":").unwrap();

        for a in right.trim().split("; ") {
            let aux: Vec<_> = a.trim_start().split(", ").collect();

            for color in aux {
                let splitted: Vec<&str> = color.split(" ").collect();
                let quantity = splitted[0].parse::<i32>().unwrap();
                let color = splitted[1];

                match color {
                    "red" => red_min_value = cmp::max(red_min_value, quantity),
                    "green" => green_min_value = cmp::max(green_min_value, quantity),
                    "blue" => blue_min_value = cmp::max(blue_min_value, quantity),
                    &_ => unreachable!(),
                };
            }
        }

        let power = red_min_value * blue_min_value * green_min_value;
        sum += power
    }

    println!("Result Part 2 - {}", sum)
}

fn is_valid(bag: &&str) -> bool {
    return bag
        .split(", ")
        .into_iter()
        .all(|item| is_color_up_to_max(item));
}

fn is_color_up_to_max(item: &str) -> bool {
    let aux: Vec<_> = item.trim().split(" ").collect();

    let quantity = aux.first().unwrap();
    let color = aux.last().unwrap();
    let quantity_n: i32 = quantity.parse::<i32>().unwrap();

    match color {
        &"red" => return quantity_n <= 12,
        &"green" => return quantity_n <= 13,
        &"blue" => return quantity_n <= 14,
        &_ => unreachable!(),
    };
}

fn parse_game() {}
