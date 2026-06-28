#![cfg(test)]

use super::*;
use ::provenance::{ProvenanceContract, ProvenanceContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn register_provenance_pair(env: &Env) -> (Address, Address, StellarProofContractClient<'_>) {
    let provenance_id = env.register(ProvenanceContract, ());
    let provenance_client = ProvenanceContractClient::new(env, &provenance_id);

    let stellarproof_id = env.register(StellarProofContract, ());
    let stellarproof_client = StellarProofContractClient::new(env, &stellarproof_id);

    provenance_client.initialize(&stellarproof_id);
    stellarproof_client.initialize(&provenance_id);

    (provenance_id, stellarproof_id, stellarproof_client)
}

#[test]
fn test_compute_hash_is_64_char_hex() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, _, client) = register_provenance_pair(&env);
    let content = String::from_str(&env, "test content");
    let owner = Address::generate(&env);

    let result = client.verify_and_mint(
        &content,
        &String::from_str(&env, "wronghash"),
        &owner,
    );

    assert!(!result.success);
    assert_eq!(result.content_hash.len(), 64);
}

#[test]
fn test_verify_success_without_provenance_init() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(StellarProofContract, ());
    let client = StellarProofContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let content = String::from_str(&env, "test content");

    let wrong = client.verify_and_mint(&content, &String::from_str(&env, "wrong"), &owner);
    let correct_hash = wrong.content_hash.clone();

    let result = client.verify_and_mint(&content, &correct_hash, &owner);
    assert!(result.success);
    assert_eq!(result.content_hash, correct_hash);
    assert!(result.certificate_id.is_none());
}

#[test]
fn test_verify_and_mint_with_provenance() {
    let env = Env::default();
    env.mock_all_auths();

    let (provenance_id, _, client) = register_provenance_pair(&env);
    let provenance_client = ProvenanceContractClient::new(&env, &provenance_id);

    let owner = Address::generate(&env);
    let content = String::from_str(&env, "verified content");

    let probe = client.verify_and_mint(&content, &String::from_str(&env, "bad"), &owner);
    let correct_hash = probe.content_hash.clone();

    let result = client.verify_and_mint(&content, &correct_hash, &owner);
    assert!(result.success);
    assert_eq!(result.certificate_id, Some(1));

    let certificate = provenance_client.get_certificate(&1).unwrap();
    assert_eq!(certificate.content_hash, correct_hash);
    assert_eq!(certificate.owner, owner);
}

#[test]
fn test_verify_failure() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, _, client) = register_provenance_pair(&env);
    let owner = Address::generate(&env);
    let content = String::from_str(&env, "test content");
    let wrong_hash = String::from_str(
        &env,
        "0000000000000000000000000000000000000000000000000000000000000000",
    );

    let result = client.verify_and_mint(&content, &wrong_hash, &owner);
    assert!(!result.success);
    assert!(result.certificate_id.is_none());
}

#[test]
fn test_different_content_different_hash() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, _, client) = register_provenance_pair(&env);
    let owner = Address::generate(&env);
    let content1 = String::from_str(&env, "content1");
    let content2 = String::from_str(&env, "content2");

    let result1 = client.verify_and_mint(&content1, &String::from_str(&env, "x"), &owner);
    let result2 = client.verify_and_mint(&content2, &String::from_str(&env, "x"), &owner);

    assert_ne!(result1.content_hash, result2.content_hash);
}

#[test]
fn test_same_content_same_hash() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, _, client) = register_provenance_pair(&env);
    let owner = Address::generate(&env);
    let content = String::from_str(&env, "same content");

    let result1 = client.verify_and_mint(&content, &String::from_str(&env, "x"), &owner);
    let result2 = client.verify_and_mint(&content, &String::from_str(&env, "x"), &owner);

    assert_eq!(result1.content_hash, result2.content_hash);
}
