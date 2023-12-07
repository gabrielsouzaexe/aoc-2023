use std::fs;

fn main() {
    let binding: String = fs::read_to_string("input.txt").expect("Should be able to read file");
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
