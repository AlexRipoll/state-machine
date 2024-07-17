use support::{Block, Dispatch};
use types::{AccountId, Balance, BlockNumber, Nonce};

mod balances;
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
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

impl crate::system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl crate::balances::Config for Runtime {
    type Balance = types::Balance;
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
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

    runtime.execute_block(block_1).expect("invalid block");

    println!("{:#?}", runtime);
}
