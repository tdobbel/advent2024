use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_trail_part1(pos: &(i32,i32), value: i32, topo: &Vec<u8>, imax: i32, jmax: i32, tails: &mut Vec<(i32,i32)>) {
    if value == 9 && !tails.contains(pos) {
        tails.push(*pos);
    }
    for (ii,jj) in [(-1,0), (1,0), (0,-1), (0,1)].iter() {
        let i = pos.0+*ii as i32;
        let j = pos.1+*jj as i32;
        if i < 0 || i >= imax || j < 0 || j >= jmax {
            continue;
        }
        let newvalue = topo[(i*jmax+j) as usize] as i32;
        if newvalue - value == 1 {
            find_trail_part1(&(i,j), newvalue, topo, imax, jmax, tails);
        }
    }
}


fn count_trails_part2(pos: &(i32,i32), value: i32, topo: &Vec<u8>, imax: i32, jmax: i32) -> usize {
    if value == 9  {
        return 1;
    }
    let mut score: usize = 0;
    for (ii,jj) in [(-1,0), (1,0), (0,-1), (0,1)].iter() {
        let i = pos.0+*ii as i32;
        let j = pos.1+*jj as i32;
        if i < 0 || i >= imax || j < 0 || j >= jmax {
            continue;
        }
        let newvalue = topo[(i*jmax+j) as usize] as i32;
        if newvalue - value == 1 {
            score += count_trails_part2(&(i,j), newvalue, topo, imax, jmax);
        }
    }
    score
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut imax: i32 = 0;
    let mut jmax: i32 = 0;
    let mut topo = Vec::<u8>::new();
    let mut seeds = Vec::<(i32,i32)>::new();
    for (i,line) in reader.lines().enumerate() {
        let line = line.unwrap();
        jmax = line.len() as i32;
        imax += 1;
        for (j,v) in line.chars()
            .map(|c| c.to_string().parse::<u8>().unwrap())
            .enumerate() {
            if v == 0 {
                seeds.push((i as i32,j as i32));
            }
            topo.push(v);
        }
    }
    let mut totalscore1: usize = 0;
    let mut totalscore2: usize = 0;
    for seed in seeds.iter() {
        let mut tails = Vec::<(i32,i32)>::new();
        find_trail_part1(seed, 0, &topo, imax, jmax, &mut tails);
        totalscore1 += tails.len();
        totalscore2 += count_trails_part2(seed, 0, &topo, imax, jmax);
    }
    println!("Total score part 1: {}", totalscore1);
    println!("Total score part 2: {}", totalscore2);
}
