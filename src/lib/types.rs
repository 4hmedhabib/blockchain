use crate::{runtime, support};

pub type AccountId = String;
pub type BlockNumber = u32;
pub type Nonce = u32;
pub type Balance = u128;
pub type Content = &'static str;

pub type Extrinsic = support::Extrinsic<AccountId, runtime::RuntimeCall>;
pub type Header = support::Header<BlockNumber>;
pub type Block = support::Block<Header, Extrinsic>;
