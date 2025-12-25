use blockchain::system;

#[test]
fn init_system() {
    struct TestConfig;

    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    let system: system::Pallet<TestConfig> = system::Pallet::new();

    assert_eq!(system.block_number(), 0);
}

#[test]
fn increment_block() {
    struct TestConfig;

    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    let mut system: system::Pallet<TestConfig> = system::Pallet::new();
    system.inc_block_number();
    assert_eq!(system.block_number(), 1);
}

#[test]
fn increment_nonce() {
    struct TestConfig;

    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    let mut system: system::Pallet<TestConfig> = system::Pallet::new();

    let ahmed = "Ahmed".to_string();

    system.inc_nonce(&ahmed);
    system.inc_nonce(&ahmed);
    assert_eq!(system.get_nonce(&ahmed), 2);
}
