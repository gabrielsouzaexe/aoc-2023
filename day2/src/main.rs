use std::fs;

fn main() {
    let binding:String = fs::read_to_string("input.txt").expect("Should be able to read file");
    let content = binding.split("\n").into_iter();

    for game in content {
        println!("{game}");
    }


}
