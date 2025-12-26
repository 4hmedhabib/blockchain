use blockchain::{balances, proof_of_existence, runtime, support, types};

fn main() {
    let mut runtime = runtime::Runtime::new();

    let ahmed = "Ahmed".to_string();
    let mohamed = "Mohamed".to_string();
    let abdallah = "Abdallah".to_string();

    runtime.balances.set_balance(&ahmed, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: ahmed.clone(),
                call: runtime::RuntimeCall::Balances(balances::Call::Transfer {
                    to: mohamed.clone(),
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: ahmed.clone(),
                call: runtime::RuntimeCall::Balances(balances::Call::Transfer {
                    to: abdallah.clone(),
                    amount: 20,
                }),
            },
        ],
    };

    runtime
        .execute_block(block_1)
        .expect("wrong block execution");

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: ahmed.clone(),
                call: runtime::RuntimeCall::ProofOfExistence(
                    proof_of_existence::Call::CreateClaim {
                        content: "my_document",
                    },
                ),
            },
            support::Extrinsic {
                caller: mohamed.clone(),
                call: runtime::RuntimeCall::ProofOfExistence(
                    proof_of_existence::Call::CreateClaim {
                        content: "mohamed's document",
                    },
                ),
            },
        ],
    };

    runtime
        .execute_block(block_2)
        .expect("wrong block execution");

    println!("Runtime: {:#?}", runtime)
}
