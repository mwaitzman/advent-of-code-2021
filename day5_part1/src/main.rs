use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::str;
use std::str::FromStr;
fn main() {
    let mut file = File::open("./input").unwrap();
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

    let mut structs = Vec::with_capacity(lines.len());
    lines.iter().for_each(|line| {
        structs.push(LineStruct::new(line));
    });

    //really shitty quadratic and sparse/mostly-wasted space usage here in the size of `points`, but whatever, it's easy to implement and I don't need more right now
    let mut max_value = 0;
    structs.iter().for_each(|n| {
        if n.x1 > max_value {max_value = n.x1;}
        if n.y1 > max_value {max_value = n.y1;}
        if n.x2 > max_value {max_value = n.x2;}
        if n.y2 > max_value {max_value = n.y2;}
    });

    let mut points =  vec![vec![0; max_value.try_into().unwrap()]; max_value.try_into().unwrap()];
    structs.iter().for_each(|line| {
        if line.x1 == line.x2 {
            if line.y1 > line.y2 {
                for v in line.y2..=line.y1 {//not sure if iterators would go in reverse automatically so adding these else ifs to both blocks to make sure the behavior is correct
                    points[line.x1 as usize][v as usize] += 1;
                }
            }
            else {
                for v in line.y1..=line.y2 {
                    points[line.x1 as usize][v as usize] += 1;//~~still seems stupid to me that it won't coerce the u32 to usize automatically, as u32 is guaranteed to losslessly convert to a usize for every possible value of it ~~-- NEVER MIND, I thought rust didn't support architectures below 32 bits. would it auto-coerce if the code it was wrapped in had `#[cfg(any(target_pointer_width = "64"), (target_pointer_width = "32")]`?
                }
            }
        }   
        else if line.y1 == line.y2 {
            if line.x1 > line.x2 {
                for v in line.x2..=line.x1 {
                    points[v as usize][line.y1 as usize] += 1;
                }
            }
            else {
                for v in line.x1..=line.x2 {
                    points[v as usize][line.y1 as usize] += 1;
                }
            }
        }
    });

    let mut count = 0;
    points.iter().for_each(|pointvec| {
        pointvec.iter().for_each(|point| {
            if point > &1 {count += 1;}//I really have to take the 1 as a reference to get it to compile??
        });
    });
    println!("answer: {}", count);
}
struct LineStruct {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}
impl LineStruct {
    pub fn new(line: &str) -> Self {//not robust at all but it doesn't need to
        //example input: "445,187 -> 912,654"
        Self {
            x1: u32::from_str(&line[..line.find(',').unwrap()]).unwrap(),
            y1: u32::from_str(&line[line.find(',').unwrap()+','.len_utf8()..line.find(' ').unwrap()]).unwrap(),
            x2: u32::from_str(line.rsplit(',').next().unwrap()).unwrap(),
            y2: u32::from_str(&line.rsplit(' ').next().unwrap().rsplit(',').next().unwrap()).unwrap(),
        }
    }
}