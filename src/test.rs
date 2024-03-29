#![cfg(test)]

use super::{IncrementContract, IncrementContractClient};
use soroban_sdk::{testutils::Logger, Env,vec,symbol};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), vec![&env, symbol!("done")]);
    assert_eq!(client.get_Data(), 1);

    assert_eq!(client.increment(), vec![&env, symbol!("done")]);
    assert_eq!(client.get_Data(), 2);
    // assert_eq!(client.get_Data(), 3);
    
    std::println!("{}", env.logger().all().join("\n"));
}
