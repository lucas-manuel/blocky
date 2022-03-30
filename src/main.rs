fn main() {
    let input = std::env::args().nth(1).expect("NO INPUT GIVEN");

    let mut block = String::from("/");
    let subblock_left = String::from("/*** ");
    let subblock_right = String::from(" ***/");

    for _i in 0..(input.len() + 8) {
        block.push_str(&"*".to_string());
    }

    block.push_str(&"/".to_string());

    println!("{0}\n{1}{2}{3}\n{4}", block, subblock_left, input, subblock_right, block);
}
