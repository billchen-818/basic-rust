#[derive(Debug)]
struct BankCard {
    card_number: String,
    card_holder: String,
    amount: f64,
}

impl BankCard {
    fn new(card_number: String, card_holder: String) -> Self {
        Self {
            card_number,
            card_holder,
            amount: 0.0,
        }
    }

    fn sub(&mut self, amount: f64) {
        self.amount -= amount;
    }

    fn add(&mut self, amount: f64) {
        self.amount += amount;
    }

    fn getcard_number(&self) -> &String {
        &self.card_number
    }

    fn getcard_holder(&self) -> &String {
        &self.card_holder
    }
}

fn main() {
    let mut card = BankCard::new("1234 5678 9012 3456".to_string(), "John Doe".to_string());

    card.add(100.0);
    card.sub(50.0);

    println!("{:?}", card);

    println!("Card number: {}", card.getcard_number());
    println!("Card holder: {}", card.getcard_holder());
}
