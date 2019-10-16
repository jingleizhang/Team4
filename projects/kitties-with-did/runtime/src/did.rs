/*
    PUBLISHED DRAFT: Decentralized Identifier (DID) 0.13
    https://w3c-ccg.github.io/did-spec/
*/

use support::{decl_module, decl_storage, decl_event, ensure, StorageValue, StorageMap, dispatch::Result, Parameter};
use system::ensure_signed;
use runtime_io::blake2_256;
use codec::{Encode, Decode};

pub const ERR_DID_REQUIRED: &str = "DID required";
pub const ERR_DID_ALREADY_EXISTS: &str = "DID already exists";
pub const ERR_DID_USER_ALREADY_HAS: &str = "User already has DID";
pub const ERR_DID_TOO_LONG: &str = "DID too long";

pub const BYTEARRAY_LIMIT: usize = 64;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct MetadataDid<AccountId> {
    pub address: AccountId,
    pub creator: AccountId,
    pub active: bool,
    pub did_prefix: Vec<u8>,
}

decl_storage! {
	trait Store for Module<T: Trait> as DidModule {
		Identity get(identity): map T::AccountId => T::Hash;
        IdentityOf get(identity_of): map T::Hash => Option<T::AccountId>;
        Metadata get(metadata): map T::Hash => Option<MetadataDid<T::AccountId>>;
        DidCount get(did_count): u64;
	}
}

pub fn validate_did(did: &[u8]) -> Result {
    ensure!(did.len() > 0, ERR_DID_REQUIRED);
    ensure!(did.len() <= BYTEARRAY_LIMIT, ERR_DID_TOO_LONG);
    Ok(())
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

        /// Create DID
        pub fn create_did(origin, account: T::AccountId, active: bool, did_type: u8) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(!<Identity<T>>::exists(&account), ERR_DID_USER_ALREADY_HAS);

            let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
		    let mut raw_hash = payload.using_encoded(blake2_256);

            //ensure!(!<IdentityOf<T>>::exists(&(raw_hash.clone())), ERR_DID_ALREADY_EXISTS);

            Ok(())
        }

        /// Resolve DID to metadata
        pub fn resolve(origin, did: T::Hash) -> Result {
            let _ = ensure_signed(origin)?;

            Ok(())
        }
	}
}

decl_event!(
	pub enum Event<T> where 
    AccountId = <T as system::Trait>::AccountId, 
    <T as system::Trait>::Hash, 
    {
		Created(AccountId, Hash),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok, parameter_types};
	use sr_primitives::{traits::{BlakeTwo256, IdentityLookup}, testing::Header};
	use sr_primitives::weights::Weight;
	use sr_primitives::Perbill;

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type WeightMultiplierUpdate = ();
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
	}
	impl Trait for Test {
		type Event = ();
	}
	type TemplateModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			// assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			// assert_eq!(TemplateModule::something(), Some(42));
		});
	}
}
