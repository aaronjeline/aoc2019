use std::fs;
use std::collections::HashMap;

type Inst = (Dirs, usize);
type Coord = (usize,usize);
type Board = HashMap<Coord,(usize, usize)>;

#[derive(Debug,Clone)]
enum Dirs {
    Right,
    Left,
    Up,
    Down
}

enum Mode {
    First,
    Second
}

use Dirs::*;
use Mode::*;

fn main() {
    let (a,b) = load("3.in");
    let bsize = findmax(&a,&b);
    let center = bsize / 2;
    let mut board = HashMap::new();
    match travel(a, &mut board, center, First) {
        Some(_) => panic!("Bad"),
        None => ()
    };
    match travel(b, &mut  board, center, Second) {
        Some(d) => println!("Answer: {}", d),
        _ => panic!("No Answer!")
    };
}

fn travel(i : Vec<Inst>, b : &mut Board, c : usize, snd: Mode) -> Option<usize>{
    let mut dists = vec![];
    let mut dist = 0;
    let (mut x, mut y) = (c,c);
    for step in i {
        for _ in 0..(step.1) {
            match step {
                (Right, _) => x += 1,
                (Left, _) => x -= 1,
                (Up, _) => y += 1,
                (Down, _) => y -=  1
            };
            dist  += 1;
            let (cc,xdist) = *b.get(&(x,y)).unwrap_or(&(0,0));
            let r = match (cc, &snd) {
                (_, First) => (1,dist),
                (1, Second) => {
                    dists.push(dist + xdist);
                    (2,xdist)
                }
                (_, Second) => (cc,xdist)
            };
            b.insert((x,y), r);
        }
    }
    dists.iter().min().map(|x| *x)
}


fn taxidist(a : (usize,usize), b : (usize,usize)) -> usize{
    ((a.0 as i32 - b.0 as i32).abs() + (a.1  as i32 - b.1 as i32).abs()) as usize
}


fn findmax(a : &Vec<Inst>, b : &Vec<Inst>) -> usize {
    let a_s : usize = a.iter().map(|x| x.1).sum();
    let b_s = b.iter().map(|x| x.1).sum();
    (if a_s > b_s { a_s } else { b_s })
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
