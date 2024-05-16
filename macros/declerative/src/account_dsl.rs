use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Account {
    pub money: u32,
}
impl Account {
    pub fn add(&mut self, money: u32) {
        self.money = self.money.add(money)
    }
    pub fn subtract(&mut self, money: u32) {
        self.money = self.money.sub(money)
    }
}

macro_rules! exchange {
    (Give 0 to $name:ident) => {
        println!("Cheapskate");
    };
    (Give $amount:literal to $name:ident) => {
        $name.add($amount)
    };
    (Take $amount:literal from $name:ident) => {
        $name.subtract($amount)
    };
    (Give $amount:literal from $giver:ident to $receiver:ident) => {
        $giver.subtract($amount);
        $receiver.add($amount)
    };
}
