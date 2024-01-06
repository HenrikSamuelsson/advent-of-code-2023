const DATA: &str = include_str!("../personal-puzzle-input.txt");

struct LensBox<'s> {
    num: usize,
    slots: Vec<(&'s str, u8)>,
}
impl<'s> LensBox<'s> {
    fn new(num: usize) -> Self {
        Self { num, slots: vec![] }
    }
    fn update(&mut self, lens: &'s str, fl: &str) {
        if fl.is_empty() {
            if let Some(index) =
              self.slots.iter().position(|(l, f)| *l == lens) {
                self.slots.remove(index);
            }
        } else {
            let fl = fl.parse().unwrap();

            if let Some(f) =
              self.slots.iter_mut().find_map(|(l, f)|
              if *l == lens { Some(f) } else { None }) {
                *f = fl;
            } else {
                self.slots.push((lens, fl));
            }
        }
    }
    fn power(&self) -> usize {
        self.slots.iter().enumerate()
            .map(|(i, lens)| (self.num + 1) * (i + 1) * lens.1 as usize)
            .sum()
    }
}

fn hash(data: &str) -> usize {
    data.bytes().fold(0, |hash, c| ((hash + c as usize) * 17) % 256)
}

fn part_1(data: &str) -> usize {
    data.trim().split(',').fold(0, |sum, step| sum + hash(step))
}

fn part_2(data: &str) -> usize {
    let number_of_boxes = 256;
    let mut boxes = (0..number_of_boxes).map(LensBox::new).collect::<Vec<_>>(); 

    for step in data.trim().split(',') {
        let (label, fl) = step.split_once(['=', '-']).unwrap();
        boxes[hash(label)].update(label, fl);
    }

    boxes.iter().filter_map(|b|
        if b.slots.is_empty() {
            None
        } else {
            Some(b.power())
        }).sum()
}

pub fn main() {
    println!("Day 15.1 = {}", part_1(DATA));
    println!("Day 15.2 = {}", part_2(DATA));
}

mod examples {
    #![cfg(test)]
    use super::*;

    const EX_A: &str = "HASH";
    const EX_B: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    const PART_1A: usize = 52;
    const PART_1B: usize = 1320;
    const PART_2: usize = 145;

    #[test] fn test_1() {
        assert_eq!(PART_1A, part_1(EX_A));
        assert_eq!(PART_1B, part_1(EX_B));
    }
    #[test] fn test_2() {
        assert_eq!(PART_2, part_2(EX_B));
    }
}
