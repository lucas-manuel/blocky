use read_input::prelude::*;

fn main() {
    let input = input::<String>().get();

    let mut block = String::from("/");
    let subblock_left = String::from("/*** ");
    let subblock_right = String::from(" ***/");

    for i in 0..(input.len() + 8) {
        block.push_str(&"*".to_string());
    }

    block.push_str(&"/".to_string());

    println!("{0}\n{1}{2}{3}\n{4}", block, subblock_left, input, subblock_right, block);
}
