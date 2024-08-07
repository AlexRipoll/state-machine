use support::{Block, Dispatch};
use types::{AccountId, Balance, BlockNumber, Nonce};

mod balances;
mod proof_of_existence;
mod support;
mod system;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl crate::system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl crate::balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new(),
        }
    }

    // Execute a block of extrinsics. Increments the block number.
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();
        if self.system.block_number() != block.header.block_number {
            return Err("block number mistmatch");
        }

        block
            .extrinsics
            .into_iter()
            .enumerate()
            .for_each(|(i, extrinsic)| {
                self.system.inc_nonce(&extrinsic.caller);
                self.dispatch(extrinsic.caller, extrinsic.call)
                    .map_err(|e| {
                        eprintln!(
                        "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                        block.header.block_number, i, e
                    )
                    });
            });

        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    // Dispatch a call on behalf of a caller. Increments the caller's nonce.
    //
    // Dispatch allows us to identify which underlying module call we want to execute.
    // Note that we extract the `caller` from the extrinsic, and use that information
    // to determine who we are executing the call on behalf of.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
            RuntimeCall::ProofOfExistence(call) => {
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }

        Ok(())
    }
}

fn main() {
    let mut runtime = Runtime::new();

    runtime.balances.set_balance(&"alice".to_string(), 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![support::Extrinsic {
            caller: "alice".to_string(),
            call: RuntimeCall::Balances(balances::Call::Transfer {
                to: "bob".to_string(),
                amount: 69,
            }),
        }],
    };

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: "alice".to_string(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: &"Hello, world!",
                }),
            },
            support::Extrinsic {
                caller: "bob".to_string(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: &"Hello, world!",
                }),
            },
        ],
    };

    let block_3 = types::Block {
        header: support::Header { block_number: 3 },
        extrinsics: vec![
            support::Extrinsic {
                caller: "alice".to_string(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::RevokeClaim {
                    claim: &"Hello, world!",
                }),
            },
            support::Extrinsic {
                caller: "bob".to_string(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: &"Hello, world!",
                }),
            },
        ],
    };

    // Execute the extrinsics which make up our blocks.
    // If there are any errors, our system panics, since we should not execute invalid blocks.
    runtime.execute_block(block_1).expect("invalid block");
    runtime.execute_block(block_2).expect("invalid block");
    runtime.execute_block(block_3).expect("invalid block");

    println!("{:#?}", runtime);
}
