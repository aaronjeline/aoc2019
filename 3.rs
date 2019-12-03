use std::fs;

type Inst = (Dirs, usize);
type Board = Vec<Vec<usize>>;

#[derive(Debug)]
enum Dirs {
    Right,
    Left,
    Up,
    Down
}

use Dirs::*;

fn main() {
    println!("{:?}", load("3.test"));
}


fn findmax(a : Vec<Inst>, b : Vec<Inst>) -> usize {
    let a_s = a.iter().map(|x| x.1).sum();
    let b_s = b.iter().map(|x| x.1).sum();
    if a_s > b_s { a_s } else { b_s }
}

fn load(s : &str) -> (Vec<Inst>, Vec<Inst>) {
    let contents = fs::read_to_string(s).expect("Read Error");
    let lines : Vec<&str> = contents.split("\n").collect();
    let a = parse_line(lines[0]);
    let b = parse_line(lines[1]);
    (a,b)
}

fn parse_line(line : &str) -> Vec<Inst> {
    let mut r = Vec::new();
    for p in line.split(",") {
        let c : Vec<char> = p.trim().chars().collect();
        let d = match c[0] {
            'U' => Up,
            'L' => Left,
            'R' => Right,
            'D' => Down,
            c => panic!("Bad Opcode : {}", c),
        };

        let s = match (&p[1..]).trim().parse::<usize>() {
            Ok(i) => i,
            _ => panic!("Bad Int!")
        };
        r.push((d,s));
    }
    r
}
