//! 🗄️ Blockchain state management
//! 
//! Efficient state storage and transitions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 🗄️ Blockchain state manager
#[derive(Debug, Clone)]
pub struct StateManager {
    accounts: HashMap<Vec<u8>, Account>,
    contracts: HashMap<Vec<u8>, Contract>,
    current_height: u64,
}

/// 👤 Account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub balance: u64,
    pub nonce: u64,
    pub code_hash: Option<[u8; 32]>, // For contract accounts
}

/// 📄 Smart contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub code: Vec<u8>,
    pub storage: HashMap<Vec<u8>, Vec<u8>>,
    pub owner: Vec<u8>,
}

impl StateManager {
    /// 🆕 Create new state manager
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            contracts: HashMap::new(),
            current_height: 0,
        }
    }

    /// 👤 Get account information
    pub fn get_account(&self, address: &[u8]) -> Option<&Account> {
        self.accounts.get(address)
    }

    /// 👤 Get or create account
    pub fn get_or_create_account(&mut self, address: &[u8]) -> &mut Account {
        self.accounts.entry(address.to_vec()).or_insert(Account {
            balance: 0,
            nonce: 0,
            code_hash: None,
        })
    }

    /// 💰 Transfer funds between accounts
    pub fn transfer(&mut self, from: &[u8], to: &[u8], amount: u64) -> Result<(), String> {
        // Check sender balance
        let sender = self.get_or_create_account(from);
        if sender.balance < amount {
            return Err("Insufficient balance".to_string());
        }

        // Perform transfer
        sender.balance -= amount;
        let recipient = self.get_or_create_account(to);
        recipient.balance += amount;

        Ok(())
    }

    /// 📈 Increment account nonce
    pub fn increment_nonce(&mut self, address: &[u8]) {
        let account = self.get_or_create_account(address);
        account.nonce += 1;
    }

    /// 📄 Deploy contract
    pub fn deploy_contract(&mut self, address: &[u8], code: Vec<u8>, owner: Vec<u8>) {
        // Calculate code hash first
        let code_hash = self.hash_code(&code);
        
        let contract = Contract {
            code: code.clone(),
            storage: HashMap::new(),
            owner,
        };

        self.contracts.insert(address.to_vec(), contract);

        // Mark account as contract
        let account = self.get_or_create_account(address);
        account.code_hash = Some(code_hash);
    }

    /// 📄 Get contract
    pub fn get_contract(&self, address: &[u8]) -> Option<&Contract> {
        self.contracts.get(address)
    }

    /// 🔍 Check if address is contract
    pub fn is_contract(&self, address: &[u8]) -> bool {
        self.accounts.get(address)
            .map(|acc| acc.code_hash.is_some())
            .unwrap_or(false)
    }

    /// 📊 Get state statistics
    pub fn get_stats(&self) -> StateStats {
        let total_accounts = self.accounts.len();
        let contract_accounts = self.accounts.values()
            .filter(|acc| acc.code_hash.is_some())
            .count();
        let total_supply = self.accounts.values()
            .map(|acc| acc.balance)
            .sum();

        StateStats {
            total_accounts,
            contract_accounts,
            total_supply,
            current_height: self.current_height,
        }
    }

    /// 🔍 Hash contract code
    fn hash_code(&self, code: &[u8]) -> [u8; 32] {
        use sha3::{Digest, Sha3_256};
        let mut hasher = Sha3_256::new();
        hasher.update(code);
        hasher.finalize().into()
    }
}

/// 📊 State statistics
#[derive(Debug, Clone)]
pub struct StateStats {
    pub total_accounts: usize,
    pub contract_accounts: usize,
    pub total_supply: u64,
    pub current_height: u64,
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_manager_creation() {
        let state = StateManager::new();
        assert_eq!(state.accounts.len(), 0);
        assert_eq!(state.current_height, 0);
        
        println!("🗄️ State manager created successfully!");
    }

    #[test]
    fn test_account_operations() {
        let mut state = StateManager::new();
        let address = vec![1, 2, 3, 4];
        
        // Create account
        let account = state.get_or_create_account(&address);
        account.balance = 1000;
        
        // Get account
        let retrieved = state.get_account(&address).unwrap();
        assert_eq!(retrieved.balance, 1000);
        assert_eq!(retrieved.nonce, 0);
        
        println!("👤 Account operations working!");
    }

    #[test]
    fn test_transfer() {
        let mut state = StateManager::new();
        let alice = vec![1, 1, 1, 1];
        let bob = vec![2, 2, 2, 2];
        
        // Setup Alice with balance
        state.get_or_create_account(&alice).balance = 1000;
        
        // Transfer from Alice to Bob
        let result = state.transfer(&alice, &bob, 300);
        assert!(result.is_ok());
        
        assert_eq!(state.get_account(&alice).unwrap().balance, 700);
        assert_eq!(state.get_account(&bob).unwrap().balance, 300);
        
        println!("💰 Transfer working!");
        println!("   Alice balance: {}", state.get_account(&alice).unwrap().balance);
        println!("   Bob balance: {}", state.get_account(&bob).unwrap().balance);
    }

    #[test]
    fn test_contract_deployment() {
        let mut state = StateManager::new();
        let contract_address = vec![1, 2, 3, 4];
        let owner = vec![5, 6, 7, 8];
        let code = vec![0x60, 0x80, 0x60, 0x40]; // Example bytecode
        
        state.deploy_contract(&contract_address, code.clone(), owner);
        
        assert!(state.is_contract(&contract_address));
        let contract = state.get_contract(&contract_address).unwrap();
        assert_eq!(contract.code, code);
        
        println!("📄 Contract deployment working!");
    }

    #[test]
    fn test_state_stats() {
        let mut state = StateManager::new();
        
        // Create some accounts
        state.get_or_create_account(&vec![1]).balance = 1000;
        state.get_or_create_account(&vec![2]).balance = 2000;
        state.deploy_contract(&vec![3], vec![0x60], vec![1]);
        
        let stats = state.get_stats();
        assert_eq!(stats.total_accounts, 3);
        assert_eq!(stats.contract_accounts, 1);
        assert_eq!(stats.total_supply, 3000);
        
        println!("📊 State statistics working!");
        println!("   Total accounts: {}", stats.total_accounts);
        println!("   Contract accounts: {}", stats.contract_accounts);
        println!("   Total supply: {}", stats.total_supply);
    }
}