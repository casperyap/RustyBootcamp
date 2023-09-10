/// A Savings Account
pub struct SavingsAccount{
    balance:i32,
}

impl SavingsAccount{
    /// Creates a new 'SavingsAccount' with a balance of 0
    /// 
    /// #Examples
    /// 
    /// ```
    /// use bank::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get_balance(), 0);
    /// ```
    /// 
    pub fn new() -> SavingsAccount{
        SavingsAccount{balance: 0,}
        // SavingsAccount{balance: 10,}
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: i32) {

        if amount < 0 {
            panic!("Amount should not be negative");
        }

        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: i32) ->Result<i32, String> {
        if amount < self.balance {
            self.balance -= amount;
            Ok(self.balance)
        }
        else {
            Err("Insufficient Funding.".to_owned())
        }
     
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starting_balance_zero(){
        let account = SavingsAccount::new();
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn able_deposit(){
        let amount = 100;
        let mut account = SavingsAccount::new();

        account.deposit(amount);
        assert_eq!(account.get_balance(), amount);
    }

    #[test]
    #[should_panic]
    fn unable_deposit_negative(){        
        let mut account = SavingsAccount::new();
        let amount = -100;
        account.deposit(amount);
    }

    #[test]
    fn able_withdraw(){
        let mut account: SavingsAccount = SavingsAccount::new();
        account.deposit(100);

        let withdraw_r = account.withdraw(50);

        match withdraw_r{
            Ok(balance) => assert_eq!(balance, 50),
            Err(_) => panic!("Unable to withdraw when we should be able to!!!"),
        }
    }

    #[test]
    fn unable_withdraw(){
        let mut account: SavingsAccount = SavingsAccount::new();
        account.deposit(100);

        let withdraw_r = account.withdraw(200);

        if let Ok(balance) = withdraw_r{
            panic!("Withdrawing more than able to. Balance: {}!", balance);
        }
    }

}
