use itertools::Itertools;
use rand::prelude::*;
use std::fmt::Write;
use std::io;

pub struct Shutbox {
    numbers: Vec<u8>,
    num_open: u8,
}

impl Shutbox {
    pub fn new() -> Self {
        Self {
            numbers: (1..=9).collect(),
            num_open: 9,
        }
    }

    pub fn get_open_num(&self) -> u8 {
        self.num_open
    }

    pub fn result(&self) -> u8 {
        self.numbers.iter().sum()
    }

    pub fn shut(&mut self, nums: &[u8]) {
        nums.iter()
            .for_each(|n| self.numbers[(*n - 1) as usize] = 0);
        self.num_open = self.numbers.iter().filter(|x| **x != 0).count() as u8;
    }

    pub fn get_string(&self) -> String {
        self.numbers
            .iter()
            .fold(String::new(), |mut s, n| {
                if *n == 0 {
                    write!(s, "_ ").ok();
                } else {
                    write!(s, "{} ", *n).ok();
                }
                s
            })
            .trim()
            .to_owned()
    }

    pub fn get_avail_answers(&self, val: u8) -> Option<Vec<Vec<u8>>> {
        let mut res: Vec<Vec<u8>> = Vec::new();
        let v: Vec<u8> = self.numbers.clone();
        let v: Vec<_> = v.into_iter().filter(|x| *x != 0).collect();
        for i in v.into_iter().powerset() {
            if i.iter().sum::<u8>() == val {
                res.push(i);
            }
        }

        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }
}

pub fn dice() -> u8 {
    let mut rng = thread_rng();

    rng.gen_range(1..=6)
}

pub fn get_user_input() -> String {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();

    user_input
}

pub fn wait_for_key_press() {
    io::stdin().read_line(&mut String::new()).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dice() {
        let value = dice();
        assert!(value.ge(&1) && value.le(&6));
    }

    #[test]
    fn test_shutbox_new() {
        let shutbox = Shutbox::new();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], shutbox.numbers);
    }

    #[test]
    fn test_shutbox_result() {
        let shutbox = Shutbox::new();
        assert_eq!(45, shutbox.result());
    }

    #[test]
    fn test_shutbox_shut() {
        let mut shutbox = Shutbox::new();
        shutbox.shut(&vec![1, 5, 9]);
        assert_eq!(vec![0, 2, 3, 4, 0, 6, 7, 8, 0], shutbox.numbers);
        assert_eq!(30, shutbox.result());
    }

    #[test]
    fn test_shutbox_get_string() {
        let mut shutbox = Shutbox::new();
        shutbox.shut(&vec![1, 5, 9]);
        assert_eq!("_ 2 3 4 _ 6 7 8 _", shutbox.get_string());
    }

    #[test]
    fn test_shutbox_get_avail_answers() {
        let mut shutbox = Shutbox::new();
        shutbox.shut(&vec![1, 2, 3, 5, 9]);
        assert_eq!("_ _ _ 4 _ 6 7 8 _", shutbox.get_string());
        assert!(shutbox.get_avail_answers(12).is_some());
        assert!(shutbox.get_avail_answers(9).is_none());
        assert!(shutbox.get_avail_answers(5).is_none());
        assert!(shutbox.get_avail_answers(11).is_some());
    }
}
