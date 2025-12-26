use blockchain::{
    balances::{self, Pallet},
    system, types,
};

struct TestConfig {}

impl system::Config for TestConfig {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for TestConfig {
    type Balance = types::Balance;
}

#[test]
fn transfer_balance() {
    let mut balances: Pallet<TestConfig> = balances::Pallet::new();

    let ahmed = "Ahmed".to_string();
    let mohamed = "Mohamed".to_string();

    balances.set_balance(&ahmed, 100);
    let _ = balances.transfer(ahmed.clone(), mohamed.clone(), 50);

    assert_eq!(balances.balance(&ahmed), 50);
    assert_eq!(balances.balance(&mohamed), 50);
}

#[test]
fn transfer_balance_overflow() {
    let mut balances: Pallet<TestConfig> = balances::Pallet::new();

    let ahmed = "Ahmed".to_string();
    let mohamed = "Mohamed".to_string();

    balances.set_balance(&ahmed, 100);
    balances.set_balance(&mohamed, u128::MAX);

    let result = balances.transfer(ahmed.clone(), mohamed.clone(), 50);

    assert_eq!(result, Err("Overflow when adding to balance"));
    assert_eq!(balances.balance(&ahmed), 100);
    assert_eq!(balances.balance(&mohamed), u128::MAX);
}
