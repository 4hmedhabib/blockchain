use blockchain::{
    balances::{self, Pallet},
    system,
};

#[test]
fn init_balances() {
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

    assert_eq!(balances.balance(&ahmed), 0);

    balances.set_balance(&ahmed, 100);

    assert_eq!(balances.balance(&ahmed), 100);
    assert_eq!(balances.balance(&mohamed), 0);
}
