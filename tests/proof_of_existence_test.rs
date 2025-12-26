use blockchain::{proof_of_existence, system, types};

struct TestConfig;

impl proof_of_existence::Config for TestConfig {
    type Content = &'static str;
}

impl system::Config for TestConfig {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

#[test]
fn basic_proof_of_existence() {
    let mut poe = proof_of_existence::Pallet::<TestConfig>::new();

    let ahmed = "Ahmed".to_string();
    let mohamed = "Mohamed".to_string();

    let _ = poe.create_claim(ahmed.clone(), "my_document");
    assert_eq!(poe.get_claim(&"my_document"), Some(&ahmed));

    let res = poe.revoke_claim(mohamed.clone(), "my_document");
    assert_eq!(res, Err("Caller is not the owner of the claim"));

    let res = poe.create_claim(mohamed.clone(), "my_document");
    assert_eq!(res, Err("Claim already exists"));

    let res = poe.revoke_claim(mohamed.clone(), "non existent");
    assert_eq!(res, Err("Claim doest not exist"));

    let res = poe.revoke_claim(ahmed.clone(), "my_document");
    assert_eq!(res, Ok(()));
    assert_eq!(poe.get_claim(&"my_document"), None);
}
