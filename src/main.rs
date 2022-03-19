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
    let mut bl: Option<usize> = None; //before loop
    let mut i = 0; //code index

    while i <= code.len() - 1 {
        step(&code[i], &mut cells, &mut ci, &mut bl, &mut i);
        i += 1;
    }
}

fn step(c: &char, cells: &mut Vec<u8>, ci: &mut usize, bl: &mut Option<usize>, i: &mut usize) {
    match c {
        '>' => *ci += 1,
        '<' => *ci -= 1,
        '+' => if cells[*ci] == 255 {cells[*ci] = 0  } else {cells[*ci] += 1},
        '-' => if cells[*ci] == 0   {cells[*ci] = 255} else {cells[*ci] -= 1},
        '.' => {
            print!("{}", cells[*ci] as char)
        },
        '[' => *bl = Some(*i),
        ']' => {
            if cells[*ci] != 0 {
                *i = bl.unwrap();
            } else {
                *bl = None
            }
        },
        _ => {}
    }
}
