use interface::ext_ft;
use near_sdk::{
    assert_one_yocto, collections::UnorderedSet, env, json_types::U128, near, require, AccountId,
    BorshStorageKey, NearToken, PanicOnDefault, Promise,
};

mod interface;

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct ControlledTransfer {
    pub owner_id: AccountId,
    pub operator_id: AccountId,
    pub valid_destinations: UnorderedSet<AccountId>,
}

#[near(serializers=[borsh])]
#[derive(BorshStorageKey)]
pub enum StorageKey {
    SafeDestinationsList,
}

#[near]
impl ControlledTransfer {
    #[init]
    pub fn new(owner_id: AccountId, operator_id: AccountId) -> Self {
        Self {
            owner_id,
            operator_id,
            valid_destinations: UnorderedSet::new(StorageKey::SafeDestinationsList),
        }
    }

    // ***************
    // * owner config
    // ***************
    #[payable]
    pub fn set_operator_id(&mut self, operator_id: AccountId) {
        assert_one_yocto();
        self.assert_only_owner();
        self.operator_id = operator_id;
    }

    // ***************
    // * assert
    // ***************
    pub(crate) fn assert_only_owner(&self) {
        require!(
            self.owner_id == env::predecessor_account_id(),
            "Only the owner can call this function."
        );
    }
    pub(crate) fn assert_operator(&self) {
        require!(
            self.operator_id == env::predecessor_account_id(),
            "Only the operator can call this function."
        );
    }
    pub(crate) fn assert_valid_destination(&self, receiver_id: &AccountId) {
        if !self.valid_destinations.contains(receiver_id) {
            panic!("destination {} not allowed", receiver_id.as_str())
        }
    }

    // ***************
    // * owner functions
    // ***************

    #[payable]
    pub fn add_valid_destination(&mut self, account_id: AccountId) {
        assert_one_yocto();
        self.assert_only_owner();
        if !self.valid_destinations.contains(&account_id) {
            self.valid_destinations.insert(&account_id);
        }
    }

    #[payable]
    pub fn remove_valid_destination(&mut self, account_id: AccountId) {
        assert_one_yocto();
        self.assert_only_owner();
        self.valid_destinations.remove(&account_id);
    }

    /**************************/
    /*   Operator functions   */
    /*   non-payable, can be  */
    /*   called with a        */
    /*   function-access key  */
    /**************************/
    pub fn ft_transfer(
        &mut self,
        token_contract_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    ) -> Promise {
        self.assert_operator();
        self.assert_valid_destination(&receiver_id);
        ext_ft::ext(token_contract_id)
            .with_unused_gas_weight(1)
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .ft_transfer(receiver_id, amount, memo)
    }

    pub fn ft_transfer_call(
        &mut self,
        token_contract_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> Promise {
        self.assert_operator();
        self.assert_valid_destination(&receiver_id);
        ext_ft::ext(token_contract_id)
            .with_unused_gas_weight(1)
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .ft_transfer_call(receiver_id, amount, memo, msg)
    }

    pub fn transfer_near(&mut self, receiver_id: AccountId, amount: NearToken) -> Promise {
        self.assert_operator();
        self.assert_valid_destination(&receiver_id);
        Promise::new(receiver_id).transfer(amount)
    }

    /**********************/
    /*   View functions   */
    /**********************/

    pub fn get_owner_id(&self) -> String {
        self.owner_id.to_string()
    }
    pub fn get_operator_id(&self) -> String {
        self.operator_id.to_string()
    }
    /// Returns a list of accounts
    pub fn get_valid_destinations(&self) -> Vec<(AccountId)> {
        self.valid_destinations.as_vector().iter().collect()
    }
}
