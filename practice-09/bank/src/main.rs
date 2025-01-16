struct Account {
    name: String,
    balance: i64,
}
impl Drop for Account {
    fn drop(&mut self) {
        if self.balance == 0 {
            println!("Account {} dropped", self.name);
        } else {
            println!("Can't drop account {}: Account balance: {}", self.name, self.balance);
        }
    }
}

impl Account {
    fn print_balance(&self) {
        println!("Account {} balance: {}", self.name, self.balance);
    }
    fn transfer_funds(&mut self, acc: &mut Account, amount: i64) {
       if amount > self.balance {
           println!("Not enough funds! Account {} balance: {}", self.name, self.balance);
       } else {
           self.balance -= amount;
           acc.balance += amount;
           println!("Transferred {} from account {} to {}", amount, self.name, acc.name);
       }
    }
}

fn destroy_account(src_acc: &mut Account, dst_acc: &mut Account) {
    println!("Destroying account {}", src_acc.name);
    src_acc.transfer_funds(dst_acc, src_acc.balance);
}

fn main() {
    let mut a1 = Account { name: "a1".to_string(), balance: 34 };
    a1.print_balance();
    let mut a2 = Account {name: "a2".to_string(), balance: 10 };
    a2.print_balance();
    let mut a3 = Account {name: "a3".to_string(), balance: 0 };
    a3.print_balance();

    a1.transfer_funds(&mut a3, 10);
    a1.transfer_funds(&mut a2, 10);
    destroy_account(&mut a1, &mut a2);


    println!("===============Result================");
    a1.print_balance();
    a2.print_balance();
    a3.print_balance();
    println!("===============Dropping==============");

}
