use std::fs;
const CELL_LENGHT: usize = 30000;

fn main() {
    let code_str = fs::read_to_string("src.bf").unwrap();
    let mut code = vec![]; //code vec
    for c in code_str.chars() {
        code.push(c)
    }
    let mut ci = 0; //cell index
    let mut cells = vec![0; CELL_LENGHT]; //cells
    let mut ol: Vec<usize> = vec![]; //opening loops
    let mut i = 0; //code index

    while i <= code.len() - 1 {
        step(&code[i], &mut cells, &mut ci, &mut ol, &mut i);
        i += 1;
    }
}

fn step(c: &char, cells: &mut Vec<u8>, ci: &mut usize, ol: &mut Vec<usize>, i: &mut usize) {
    match c {
        '>' => if *ci == CELL_LENGHT -1 {panic!("overflow")} else {*ci += 1},
        '<' => if *ci == 0 {panic!("overflow negative cell at index {}", i)} else {*ci -= 1},
        '+' => {cells[*ci] = cells[*ci].wrapping_add(1)},
        '-' => {cells[*ci] = cells[*ci].wrapping_sub(1)},
        '.' => {
            print!("{}", cells[*ci] as char)
        },
        '[' => ol.push(*i),
        ']' => {
            if cells[*ci] != 0 {
                *i = *ol.last().expect(&format!("Closed bracket without open bracket on char {}", i));
            } else {
                ol.pop();
            }
        },
        _ => {}
    }
}
