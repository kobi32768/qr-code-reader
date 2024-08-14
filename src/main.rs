fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    // remove newline code
    let input = input.lines().collect::<String>();

    let mut bytes: Vec<u8> = vec![0; (input.len() / 8) + 1];
    for (ci, c) in input.chars().enumerate() {
        let bi: usize = ci / 8;
        bytes[bi] |= (c.to_digit(2).unwrap() as u8) << 7 - (ci % 8);

        if ci % 8 == 7 {
            println!("{:08b} {:02x}", bytes[bi], bytes[bi]);
        }
    }

    let modulo = input.len() % 8;
    let mask = (1 << modulo) - 1 as u8;
    println!("{:0>w$b}", (bytes[input.len() / 8] >> 8 - modulo) & mask, w = modulo);
}
