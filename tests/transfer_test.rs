use blockchain::{
    balances::{self, Pallet},
    system,
};

#[test]
fn transfer_balance() {
    struct TestConfig {}

    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl balances::Config for TestConfig {
        type Balance = u128;
    }

    let mut balances: Pallet<TestConfig> = balances::Pallet::new();

    let ahmed = "Ahmed".to_string();
    let mohamed = "Mohamed".to_string();

    balances.set_balance(&ahmed, 100);
    let _ = balances.transfer(&ahmed, &mohamed, 50);

    assert_eq!(balances.balance(&ahmed), 50);
    assert_eq!(balances.balance(&mohamed), 50);
}

#[test]
fn transfer_balance_overflow() {
    struct TestConfig {}

    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl balances::Config for TestConfig {
        type Balance = u128;
    }

    let mut balances: Pallet<TestConfig> = balances::Pallet::new();

    let ahmed = "Ahmed".to_string();
    let mohamed = "Mohamed".to_string();

    balances.set_balance(&ahmed, 100);
    balances.set_balance(&mohamed, u128::MAX);

    let result = balances.transfer(&ahmed, &mohamed, 50);

    assert_eq!(result, Err("Overflow when adding to balance"));
    assert_eq!(balances.balance(&ahmed), 100);
    assert_eq!(balances.balance(&mohamed), u128::MAX);
}
