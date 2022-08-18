use std::collections::HashSet;
use std::io;
use std::io::{Read, Write};
use log::debug;
use crate::Lotto;

pub struct LottoConsole {
    winning_nums: Lotto,
    my_lotteries: Vec<Lotto>,
}

impl LottoConsole {
    pub fn new() -> Self {
        LottoConsole {
            winning_nums: Lotto::new(),
            my_lotteries: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        loop {
            let command = self.get_input_str("".into());
            match command.trim() {
                "exit" => panic!("종료 합니다 .. "),
                "help" => println!("메뉴입력 ex) status, buy, remove, gogo, result \n"),
                "status" => {
                    println!("총 구매 개수 : {} ", &self.my_lotteries.len());
                    if let 6 = &self.winning_nums.nums.len() {
                        println!("당첨 번호 : {:?} ", &self.winning_nums.nums);
                    } else {
                        println!("추첨 전 입니다.");
                    }
                }
                "buy" => self.buy_lotto(),
                "remove" => self.remove_lotto(),
                "gogo" => self.gogo(),
                "result" => self.show_result(),
                _ => (),
            }
        }
    }

    fn get_input_str(&self, msg: String) -> String {
        let mut buf = String::new();
        print!("{} >", msg);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();
        buf
    }

    fn buy_lotto(&mut self) {
        let buy_count_str = self.get_input_str("구매 개수 입력".into());
        if let Ok(i) = buy_count_str.trim().parse::<u32>() {
            for _ in 0..i {
                let mut new_lotto = Lotto::new();
                new_lotto.fill_rand_nums();
                self.my_lotteries.push(new_lotto);
            }
        } else {
            println!("잘못된 입력 입니다.");
        }
    }
    //로또 추첨
    fn gogo(&mut self) {
        self.winning_nums.nums.clear();
        let auto_str = self.get_input_str("자동으로 구성 하시겠습니까?  press Enter or n\n".into());

        if let "n" = auto_str.trim() {
            loop {
                let user_input = self.input_user_manual_nums();
                if user_input.len() == 6 {
                    self.winning_nums.nums = user_input;
                    break;
                }
            }
        } else {
            //자동 구성
            self.winning_nums.fill_rand_nums();
        }
        println!("로또 당첨 번호  {:?} ", &self.winning_nums.nums);
    }

    fn input_user_manual_nums(&mut self) -> HashSet<u8> {
        //수동 구성
        let num_input = self.get_input_str("숫자 6개 를 입력해 주세요 ex) 11,12,13,14,15,16\n".into());
        let parse_nums = num_input.trim().split(",")
            .map(|n| match n.parse::<u8>() {
                Ok(t) => t,
                Err(e) => 46,
            }).filter(|n| *n < 46).collect::<HashSet<u8>>();
        debug!("{:?}",parse_nums);
        parse_nums
    }

    //비교 결과 출력
    fn show_result(&self) {
        let _5rd: Vec<&Lotto> = self.my_lotteries.iter().filter(|lotto| lotto.diff(&self.winning_nums) == 3).collect();
        let _4rd: Vec<&Lotto> = self.my_lotteries.iter().filter(|lotto| lotto.diff(&self.winning_nums) == 4).collect();
        let _3rd: Vec<&Lotto> = self.my_lotteries.iter().filter(|lotto| lotto.diff(&self.winning_nums) == 5).collect();
        let _1rd: Vec<&Lotto> = self.my_lotteries.iter().filter(|lotto| lotto.diff(&self.winning_nums) == 6).collect();

        println!("5등 당첨 리스트 {:?}\n", _5rd);
        println!("4등 당첨 리스트 {:?}\n", _4rd);
        println!("3등 당첨 리스트 {:?}\n", _3rd);
        println!("1등 당첨 리스트 {:?}\n", _1rd);
        println!("\n<<<<<<< 당첨 통계 >>>>>>>");
        println!("5등 당첨 {}개 ", _5rd.len());
        println!("4등 당첨 {}개 ", _4rd.len());
        println!("3등 당첨 {}개 ", _3rd.len());
        println!("1등 당첨 {}개 ", _1rd.len());
    }
    fn remove_lotto(&mut self) {
        println!("{}개 삭제", self.my_lotteries.len());
        self.my_lotteries.clear();
    }
}