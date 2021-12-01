use std::fs::File;
use std::io::Read;
fn main() {
    let mut file = File::open("./src/input").unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let mut ints: Vec<u64> = vec![];
    let mut j = 0;
    for (i, byte) in data.iter().enumerate() {
        match byte {
            b'\n' => {
                ints.push(parse(&data[j..i]));
                j = i + 1;
            }
            b'0'..=b'9' => (),
            _ => panic!(),
        }
    }
    let mut n = 0;
    for i in 1..ints.len() {
        if &ints[i] > &ints[i - 1] {
            n += 1;
        }
    }
    println!("answer: {}", n);
}

fn parse(data: &[u8]) -> u64 {
    data.iter().fold(0, |a, b| 10 * a + (b - b'0') as u64)
}
