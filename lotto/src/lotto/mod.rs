use std::collections::HashSet;
use log::debug;
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

#[derive(Debug)]
pub struct Lotto {
    pub nums: HashSet<u8>,
}

impl Lotto {
    //빈 생성자
    pub fn new() -> Self {
        Lotto {
            nums: HashSet::new(),
        }
    }

    //랜덤 함수
    fn gen_rand_num() -> u8 {
        rand::thread_rng().gen_range(1..46) as u8
    }

    //nums 맴버에 수동으로 채우기
    pub fn fill_manual(&mut self, param_nums: (u8, u8, u8, u8, u8, u8)) -> bool {
        let arr: [u8; 6] = [param_nums.0, param_nums.1, param_nums.2, param_nums.3, param_nums.4, param_nums.5];
        let nums_set: HashSet<u8> = HashSet::from(arr);

        match nums_set.len() {
            6 => {
                self.nums = nums_set;
                true
            }
            _ => false
        }
    }

    // nums 맴버에 랜덤수 채우기
    pub fn fill_rand_nums(&mut self) {
        while let 0..=5 = self.nums.len() {
            let r_num = Lotto::gen_rand_num();
            //debug!("r_rnum is {r_num}");
            self.nums.insert(r_num);
        }
    }

    //다른 lotto 와 비교
    pub fn diff(&self, other: &Lotto) -> i8 {
        let diff_count = self.nums.difference(&other.nums).count() as i8;
        let x = (diff_count - 6).abs();
        x
    }
}