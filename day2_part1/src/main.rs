use std::fs::File;
use std::io::Read;
use std::str;
use std::str::FromStr;

fn main() {
    let mut file = File::open("../input").unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let mut lines = vec![];
    let mut j = 0;
    for (i, byte) in data.iter().enumerate() {
        if let b'\n' = byte {
            lines.push(str::from_utf8(&data[j..i]).unwrap());
            j = i + 1;
        }
    }
    let mut x = 0;
    let mut y = 0;
    lines.iter().for_each(|line| {
        if let Some(_) = line.find("forward") {
            x += i32::from_str(line.split_whitespace().nth(1).unwrap()).unwrap();
        }
        else if let Some(_) = line.find("up") {
            y -= i32::from_str(line.split_whitespace().nth(1).unwrap()).unwrap();
        }
        else if let Some(_) = line.find("down") {
            y += i32::from_str(line.split_whitespace().nth(1).unwrap()).unwrap();
        }
        else {panic!()};
    });
    println!("answer: {}", x * y);
}
