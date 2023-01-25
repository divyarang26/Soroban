// #![no_std]
// use soroban_sdk::{contractimpl, log, symbol, Env, Symbol};

// const COUNTER: Symbol = symbol!("COUNTER");

// pub struct IncrementContract;

// #[contractimpl]
// impl IncrementContract {
//     /// Increment increments an internal counter, and returns the value.
//     pub fn increment(env: Env) -> u32 {
//         // Get the current count.
//         let mut count: u32 = env
//             .storage()
//             .get(&COUNTER)
//             .unwrap_or(Ok(0)) // If no value set, assume 0.
//             .unwrap(); // Panic if the value of COUNTER is not u32.
//         log!(&env, "count: {}", count);

//         // Increment the count.
//         count += 1;

//         // Save the count.
//         env.storage().set(&COUNTER, &count);

//         // Return the count to the caller.
//         count
//     }
// }

// mod test;

#![no_std]
use soroban_sdk::{contractimpl, symbol, Env, Symbol,Vec,vec,log};

const COUNTER: Symbol = symbol!("COUNTER");

pub struct IncrementContract;

#[contractimpl]
impl IncrementContract{
    pub fn increment(env: Env) -> Vec<Symbol>{
        let mut count: u32  = env
            .storage()
            .get(&COUNTER)
            .unwrap_or(Ok(0)) 
            .unwrap(); 
       log!(&env , "counter",count);
        count += 1;
        env.storage().set(&COUNTER, &count);
        vec![&env, symbol!("done")]
    }
    pub fn get_Data(env:Env) -> u32{
        let mut value: u32 = env
        .storage()
        .get(&COUNTER)
        .unwrap_or(Ok(0)) 
        .unwrap();
        value
    }
}

mod test;