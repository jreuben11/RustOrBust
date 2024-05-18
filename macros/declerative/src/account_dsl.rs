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

pub enum Currency {
    Euro,
    Dollar,
}
impl From<&str> for Currency {
    fn from(value: &str) -> Self {
        if value.contains("euro") {
            Currency::Euro
        } else {
            // simple fallback to dollars
            Currency::Dollar
        }
    }
}
impl Currency {
    pub fn calculate(&self, amount: u32) -> u32 {
        match self {
            Currency::Euro => amount,
            Currency::Dollar => amount * 2,
        }
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
    (Give $amount:literal $currency:literal to $name:ident) => {
        let curr: Currency = $currency.into(); // From does not need to be public (and in fact cannot be)
        $name.add(curr.calculate($amount))
    }
}
