use std::{collections::{HashSet, HashMap}, mem::MaybeUninit};

use itertools::Itertools;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Line{
    Horizontal(u32, (u32,u32)),
    Vertical(u32, (u32,u32)),
    //startpoint and length
    DownDiagonal((u32,u32), u32),
    //up has increasing y
    UpDiagonal((u32,u32), u32)
}
impl Line {
    fn get_coords(&self) ->Vec<(u32, u32)> {
        match self.to_owned() {
            Line::Horizontal(y, (x1,x2)) => (x1..=x2).map(|x| (x, y)).collect(),
            Line::Vertical(x, (y1,y2)) => (y1..=y2).map(|y| (x, y)).collect(),
            Line::DownDiagonal((x,y), length) => (0..=length).map(|i| (x+i,y-i)).collect(),
            Line::UpDiagonal((x,y), length) => (0..=length).map(|i| (x+i,y+i)).collect(),   
        }
    }
}

fn to_line(((x1,y1),(x2,y2)): ((u32,u32),(u32,u32))) -> Option<Line> {
    
    if x1 == x2 {
        let (y1,y2) = (y1.min(y2), y2.max(y1));
        Some(Line::Vertical(x1, (y1, y2)))
    }
    else if y1 == y2 {
        let (x1,x2) = (x1.min(x2), x2.max(x1));
        Some(Line::Horizontal(y1, (x1,x2)))
    } else {
        Some(
        if x1 < x2 {
            if y1 < y2 {Line::UpDiagonal((x1, y1), x2-x1)}
            else {Line::DownDiagonal((x1, y1), x2-x1)}
        } else {
            if y2 < y1 {Line::UpDiagonal((x2, y2), x1-x2)}
            else {Line::DownDiagonal((x2, y2), x1-x2)}
        })
    }
}

fn to_orthogonal_line(((x1,y1),(x2,y2)): ((u32,u32),(u32,u32))) -> Option<Line> {
    
    if x1 == x2 {
        let (y1,y2) = (y1.min(y2), y2.max(y1));
        Some(Line::Vertical(x1, (y1, y2)))
    }
    else if y1 == y2 {
        let (x1,x2) = (x1.min(x2), x2.max(x1));
        Some(Line::Horizontal(y1, (x1,x2)))
    } else {
        None
    }
}


fn main() {
    let mut intersections: Vec<u32> = vec![0; 1000*1000];
   
    for line in std::fs::read_to_string("input.txt").unwrap().lines().filter_map(|line| {
        to_line(
            line.split(" -> ").map(|coords| {
                coords.split(',').map(|x| x.parse::<u32>().unwrap()).collect_tuple().unwrap()
            }).collect_tuple().unwrap()
        )

    }) {
        for (x,y) in line.get_coords() {
            intersections[(x*1000+y) as usize] += 1
        }
    }
    println!("{}", intersections.iter().filter(|&&x| x>=2).count());
    


    
}
