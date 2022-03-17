/// Solution for [2043. Simple Bank System](https://leetcode-cn.com/problems/simple-bank-system/)
pub struct Bank {
    balance: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {
    pub fn new(balance: Vec<i64>) -> Self {
        Bank {
            balance: balance.clone(),
        }
    }

    fn valid(&self, account: i32) -> bool {
        account <= self.balance.len() as i32
    }

    pub fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        self.valid(account1)
            && self.valid(account2)
            && self.withdraw(account1, money)
            && self.deposit(account2, money)
    }

    pub fn deposit(&mut self, account: i32, money: i64) -> bool {
        if self.valid(account) {
            self.balance[account as usize - 1] += money;
            true
        } else {
            false
        }
    }

    pub fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if self.valid(account) && self.balance[account as usize - 1] >= money {
            self.balance[account as usize - 1] -= money;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
        assert_eq!(bank.withdraw(3, 10), true);
        assert_eq!(bank.transfer(5, 1, 20), true);
        assert_eq!(bank.deposit(5, 20), true);
        assert_eq!(bank.transfer(3, 4, 15), false);
        assert_eq!(bank.withdraw(10, 50), false);
    }
}
