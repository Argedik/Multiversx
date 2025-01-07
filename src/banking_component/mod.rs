// 1. Trait tanımla
pub trait Account {
  fn deposit(&mut self, amount: f64);
  fn withdraw(&mut self, amount: f64);
  fn balance(&self) -> f64;
}

// 2. BankAccount struct'ı oluştur
pub struct BankAccount {
  pub account_number: u32,
  pub holder_name: String,
  pub balance: f64,
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
