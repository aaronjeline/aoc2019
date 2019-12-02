use std::fs;

fn main() {
    let instrs = load("2.in");
    for noun in 0..100 {
        for verb in 0..100 {
            match run(instrs.clone(), noun, verb) {
                19690720 => {
                    println!("Answer: {}", 100 * noun + verb); break;
                },
                _ => (),
            }
        }
    }
}

fn run(mut instrs : Vec<usize>, noun:usize, verb:usize) -> usize {
    let mut pc = 0;
    instrs[1] = noun;
    instrs[2] = verb;

    loop {
        let addr : usize;
        let val : usize;

        let instr = &instrs[pc..pc+4];
        match instr[0] {
            1 => {
                addr = instr[3];
                val = instrs[instr[1]] + instrs[instr[2]];
            },
            2 => {
                addr = instr[3];
                val = instrs[instr[1]] * instrs[instr[2]];
            },
            99 => break,
            i => {println!("Invalid opcode {}, pc {}", i, pc); panic!();},
        }
        instrs[addr] = val;
        pc += 4;
    }

    instrs[0]
}

fn load(s : &str) -> Vec<usize> {
    let mut data = Vec::new();
    let contents = fs::read_to_string(s).expect("Read Error");
    for num in contents.split(",") {
        match num.trim().parse::<usize>() {
            Ok(i) => data.push(i),
            _ => println!("Couldn't parse: {}", num),
        };
    }

    data
}
