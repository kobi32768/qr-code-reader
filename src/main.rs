use std::{any::Any, fs::File, io::Read};

fn main() {
    // println!("Enter path to XORed file:");
    // let mut filepath = String::new();
    // std::io::stdin().read_line(&mut filepath).unwrap();
    let filepath = "C:/Users/kobi32768/Downloads/yui_rmqr.txt";

    let mut f = File::open(filepath).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let mut bin = Vec::<Pixel>::new();
    for c in contents.chars() {
        match c {
            '0' => bin.push(0),
            '1' => bin.push(1),
            '\r' => (),
            '\n' => (),
            _ => panic!("Invalid character in file"),
        }
    }

    let bytes = pack_into_bytes(bin);

    let mut blocks = vec![vec![0; 27]; 2]; // i 2, j 27
    for j in 0..14 {
        blocks[0][j] = bytes[j * 2];
    }
    for j in 0..15 {
        blocks[1][j] = bytes[j * 2 + 1];
    }

    let mut out = Vec::<u8>::new();
    for i in 0..2 {
        for j in 0..15 {
            out.push(blocks[i][j]);

            println!("{:08b} {:2x}", blocks[i][j], blocks[i][j]);
        }
    }
}

// TODO: Option をいい感じに使う

fn pack_into_bytes(bin: Vec<Pixel>) -> Vec<u8> {
    let mut packed = Vec::<u8>::new();
    for i in (0..bin.len()).step_by(8) {
        let mut byte = 0;
        for j in 0..8 {
            byte |= (bin.get(i + j).unwrap_or(&0) & MASK_VALUE) << (7 - j);
        }
        packed.push(byte);
    }
    packed
}

fn unpack_from_bytes(bytes: Vec<u8>) -> Vec<Pixel> {
    let mut unpacked = Vec::<Pixel>::new();
    for byte in bytes {
        for i in 0..8 {
            let pixel: Pixel = (byte >> (7 - i)) & MASK_VALUE | MASK_VALID;
            unpacked.push(pixel);
        }
    }
    unpacked
}

type Pixel = u8;
const MASK_VALUE: Pixel = 1;
const MASK_VALID: Pixel = 2;

/*
            is_valid
            | value
            | |
0 0 0 0 0 0 0 0
*/
