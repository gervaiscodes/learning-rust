struct MasterCard {
    number: u32,
    verification: u32
}

struct BitCredit {
    btc_number: u32
}

trait CreditCharge {
    fn charge_with_id(&self, id: u32) -> bool;
}

impl CreditCharge for MasterCard {
    fn charge_with_id(&self, id: u32) -> bool {
        self.number < id && self.verification < id
    }
}

impl CreditCharge for BitCredit {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 2 == self.btc_number % 2
    }
}

fn main() {
    let card = BitCredit { btc_number: 1024 };
    let code = 4096;

    if card.charge_with_id(code) {
        println!("Success!")
    } else {
        println!("Failure");
    }
}
