#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;
use ink_prelude::vec::Vec;
pub use contract_types::*;

pub type Quantity = u64;
pub type ClassId = u32;
pub type TokenId = u64;
pub type GlobalId = u64;
pub type Metadata = Vec<u8>;
pub type Chars = Vec<u8>;
pub type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
pub type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
pub type PerU16 = u16;

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum NFTMartErr {
    Fail,
}

impl ink_env::chain_extension::FromStatusCode for NFTMartErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::Fail),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize = <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;
    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;
    type RentFraction = <ink_env::DefaultEnvironment as Environment>::RentFraction;
    type ChainExtension = NFTMart;
}

#[ink::chain_extension]
pub trait NFTMart {
    type ErrorCode = NFTMartErr;

    #[ink(extension = 2001, returns_result = false)]
    fn fetch_random() -> [u8; 32];

    #[ink(extension = 2002, returns_result = false)]
    fn create_class(metadata: Metadata, name: Chars, description: Chars, properties: u8, royalty_rate: PerU16, category_ids: Vec<GlobalId>) -> (ink_env::AccountId, ClassId);

    #[ink(extension = 2003, returns_result = false)]
    fn proxy_mint(
        to: &ink_env::AccountId,
        class_id: ClassId,
        metadata: Metadata,
        quantity: Quantity,
        charge_royalty: Option<bool>,
    ) -> (ink_env::AccountId, ink_env::AccountId, ClassId, TokenId, Quantity);

    #[ink(extension = 2004, returns_result = false)]
    fn transfer(to: &ink_env::AccountId, class_id: ClassId, token_id: TokenId, quantity: Quantity) -> ();

    #[ink(extension = 1001, handle_status = false, returns_result = false)]
    fn tokens(class_id: ClassId, token_id: TokenId) -> Option<ContractTokenInfo<
        Metadata, Quantity, Balance, BlockNumber, ink_env::AccountId,
    >>;

    #[ink(extension = 1101, handle_status = false, returns_result = false)]
    fn sr25519_verify(
        account: &ink_env::AccountId, signature: Vec<u8>, message: Vec<u8>
    ) -> bool;
}
