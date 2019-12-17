use std::fs;
use std::collections::HashMap;

type Orbits = HashMap<String, Vec<String>>;

fn main() {
    let os = load("6.test");
}

fn load(f : &str) -> Orbits {
    let mut os : Orbits = HashMap::new();
    let contents = fs::read_to_string(f).expect("Can't read input");
    for line in contents.split("\n") {
        let ld : Vec<&str> = line.split(")").collect();
        let mut cur = os.get(ld[0]).unwrap_or(&vec![]).clone();
        cur.push(ld[1].to_string());
        os.insert(ld[0].to_string(), cur);
    }
    os
}
