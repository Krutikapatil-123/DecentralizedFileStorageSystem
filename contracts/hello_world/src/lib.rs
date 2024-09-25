#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Define a structure to track file metadata
#[contracttype]
#[derive(Clone)]
pub struct FileMetadata {
    pub file_id: u64,       // Unique ID for the file
    pub file_name: String,  // File name
    pub file_hash: String,  // Hash of the file for verification
    pub owner: String,      // Owner of the file
    pub created_at: u64,    // Timestamp of when the file was uploaded
    pub encrypted: bool,    // Indicates whether the file is encrypted
    pub access_control: u64 // Reference to permissions associated with the file
}

// Define a structure for access control
#[contracttype]
#[derive(Clone)]
pub struct AccessControl {
    pub file_id: u64,       // Reference to the file ID
    pub user: String,       // Address or identifier for the user
    pub permission: bool    // Permission status (true: access granted, false: access denied)
}

// For referencing FileMetadata struct
const FILE_METADATA: Symbol = symbol_short!("FILE_META");

// For referencing AccessControl struct
const ACCESS_CONTROL: Symbol = symbol_short!("ACCE_CON");

// For creating unique file IDs
const COUNT_FILES: Symbol = symbol_short!("C_FILES");

#[contract]
pub struct DecentralizedFileStorage;

#[contractimpl]
impl DecentralizedFileStorage {
    // This function creates and stores file metadata
    pub fn create_file(env: Env, file_name: String, file_hash: String, owner: String, encrypted: bool) -> u64 {
        let mut count_files: u64 = env.storage().instance().get(&COUNT_FILES).unwrap_or(0);
        count_files += 1;

        // To get the timestamp for file creation
        let time = env.ledger().timestamp();

        // Creating an instance of FileMetadata
        let new_file = FileMetadata {
            file_id: count_files,
            file_name: file_name.clone(),
            file_hash: file_hash.clone(),
            owner: owner.clone(),
            created_at: time,
            encrypted,
            access_control: count_files, // Each file has its own access control
        };

        // Store file metadata in storage
        env.storage().instance().set(&FILE_METADATA, &new_file);
        env.storage().instance().set(&COUNT_FILES, &count_files);

        // Log file creation event
        log!(&env, "File created with ID: {}, by owner: {}", new_file.file_id, owner);

        new_file.file_id
    }

    // This function allows the owner to set access control for the file
    pub fn set_access_control(env: Env, file_id: u64, user: String, permission: bool) {
        // Fetching the file metadata
        let file_metadata: FileMetadata = env.storage().instance().get(&FILE_METADATA).unwrap_or_else(|| panic!("File not found!"));

        // Make sure only the owner can set access control
        if file_metadata.file_id == file_id {
            // Create AccessControl entry
            let new_access = AccessControl {
                file_id,
                user: user.clone(),
                permission,
            };

            // Store the access control data
            env.storage().instance().set(&ACCESS_CONTROL, &new_access);

            log!(&env, "Access for file ID: {} set for user: {} with permission: {}", file_id, user, permission);
        } else {
            panic!("You are not the owner of this file.");
        }
    }

    // This function retrieves file metadata based on file ID
    pub fn get_file_metadata(env: Env, file_id: u64) -> FileMetadata {
        let file_metadata: FileMetadata = env.storage().instance().get(&FILE_METADATA).unwrap_or_else(|| panic!("File not found!"));
        log!(&env, "File retrieved with ID: {}", file_id);
        file_metadata
    }

    // This function checks if a user has access to a specific file
    pub fn check_access(env: Env, file_id: u64, user: String) -> bool {
        let access_control: AccessControl = env.storage().instance().get(&ACCESS_CONTROL).unwrap_or_else(|| panic!("Access control not found!"));

        if access_control.file_id == file_id && access_control.user == user {
            log!(&env, "User {} has access: {}", user, access_control.permission);
            access_control.permission
        } else {
            log!(&env, "User {} does not have access.", user);
            false
        }
    }
}
