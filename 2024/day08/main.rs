use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn isvalid(node: &(i32, i32), imax: i32, jmax: i32) -> bool {
    node.0 >= 0 && node.0 < imax && node.1 >= 0 && node.1 < jmax
}

fn add_node(antinodes: &mut Vec<(i32,i32)>, node: &(i32,i32)) {
    if !antinodes.contains(node) {
        antinodes.push(*node);
    }
}

fn part1(antennas: &HashMap<String, Vec<(i32,i32)>>, imax: i32, jmax: i32) -> usize {
    let mut antinodes = Vec::<(i32, i32)>::new();
    for (_,v) in antennas.iter() {
        if v.len() == 1 {
            continue
        }
        for i in 0..v.len()-1 {
            for j in i+1..v.len() {
                let a0 = v[i];
                let a1 = v[j];
                let ii = a1.0 - a0.0;
                let jj = a1.1 - a0.1;
                let node1 = (a1.0+ii, a1.1+jj);
                let node2 = (a0.0-ii, a0.1-jj);
                for node in [node1, node2].iter() {
                    if isvalid(&node, imax, jmax) {
                        add_node(&mut antinodes, node);
                    }
                }
            }
        }
    }
    antinodes.len()
}

fn part2(antennas: &HashMap<String, Vec<(i32,i32)>>, imax: i32, jmax: i32) -> usize {
    let mut antinodes = Vec::<(i32, i32)>::new();
    for (_,v) in antennas.iter() {
        if v.len() == 1 {
            continue
        }
        for i in 0..v.len()-1 {
            for j in i+1..v.len() {
                let a0 = v[i];
                let a1 = v[j];
                let ii = a1.0 - a0.0;
                let jj = a1.1 - a0.1;
                let mut node = a0;
                while isvalid(&node, imax, jmax) {
                    add_node(&mut antinodes, &node);
                    node.0 -= ii;
                    node.1 -= jj;
                }
                node = a1;
                while isvalid(&node, imax, jmax) {
                    add_node(&mut antinodes, &node);
                    node.0 += ii;
                    node.1 += jj;
                }
            }
        }
    }
    antinodes.len()
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut imax: i32 = 0;
    let mut jmax: i32 = 0;
    let mut antennas = HashMap::<String, Vec<(i32, i32)>>::new();
    for (i,line) in reader.lines().enumerate() {
        let line = line.unwrap();
        jmax = line.chars().count() as i32;
        imax += 1;
        for (j,c) in line.chars().enumerate() {
            if c == '.' {
                continue
            }
            antennas.entry(c.to_string())
                .or_insert(Vec::new())
                .push((i as i32, j as i32));
        }
    }
    println!("{} antinodes found in part 1", part1(&antennas, imax, jmax));
    println!("{} antinodes found in part 2", part2(&antennas, imax, jmax));

}