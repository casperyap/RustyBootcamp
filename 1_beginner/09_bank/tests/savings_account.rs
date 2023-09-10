use bank::SavingsAccount;

mod utils;

#[test]
fn starting_balance_zero(){
    utils::common_setup();
    let account = SavingsAccount::new();
    assert_eq!(account.get_balance(), 0);
}