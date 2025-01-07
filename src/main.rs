// 1. Trait tanımla
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

// 2. BankAccount struct'ı oluştur
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

// 3. Trait'i BankAccount üzerinde uygula
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("{} hesabına {} yatırıldı. Güncel bakiye: {}", 
                self.holder_name, amount, self.balance);
    }

    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
            println!("{} hesabından {} çekildi. Güncel bakiye: {}", 
                    self.holder_name, amount, self.balance);
        } else {
            println!("{} hesabında yeterli bakiye yok!", self.holder_name);
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // 4. İki farklı hesap oluştur
    let mut account1 = BankAccount {
        account_number: 1001,
        holder_name: String::from("Ali"),
        balance: 0.0,
    };

    let mut account2 = BankAccount {
        account_number: 2002,
        holder_name: String::from("Ayşe"),
        balance: 0.0,
    };

    // 5. Birinci hesaba para yatır
    account1.deposit(1000.0);

    // 6. İkinci hesaptan para çekmeden önce biraz para ekleyelim
    account2.deposit(500.0);
    account2.withdraw(200.0);

    // 7. Her iki hesabın güncel bakiyesini yazdır
    println!("{} adlı hesabın güncel bakiyesi: {}", 
            account1.holder_name, account1.balance());

    println!("{} adlı hesabın güncel bakiyesi: {}", 
            account2.holder_name, account2.balance());
}
