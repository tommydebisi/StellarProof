#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, BytesN, Env, String, Vec};

mod provenance {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/provenance.wasm"
    );
}

#[contract]
pub struct StellarProofContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationResult {
    pub success: bool,
    pub content_hash: String,
    pub certificate_id: Option<u64>,
}

#[contractimpl]
impl StellarProofContract {
    /// Initialize the contract with the provenance contract address.
    /// Provenance must be initialized with this contract's address as the oracle.
    pub fn initialize(env: Env, provenance_address: Address) {
        env.storage()
            .instance()
            .set(&symbol_short!("PROV_ADR"), &provenance_address);
    }

    /// Verify content and mint a provenance certificate when the hash matches.
    pub fn verify_and_mint(
        env: Env,
        content: String,
        expected_hash: String,
        owner: Address,
    ) -> VerificationResult {
        let computed_hash_string = Self::compute_hash(&env, &content);
        let verification_success = computed_hash_string == expected_hash;

        if !verification_success {
            return VerificationResult {
                success: false,
                content_hash: computed_hash_string,
                certificate_id: None,
            };
        }

        let certificate_id = Self::mint_certificate(&env, &computed_hash_string, &owner);

        match certificate_id {
            Ok(cert_id) => VerificationResult {
                success: true,
                content_hash: computed_hash_string,
                certificate_id: Some(cert_id),
            },
            Err(_) => VerificationResult {
                success: true,
                content_hash: computed_hash_string,
                certificate_id: None,
            },
        }
    }

    /// Compute SHA-256 hash of content and return as a 64-character lowercase hex string.
    fn compute_hash(env: &Env, content: &String) -> String {
        let content_bytes = content.to_bytes();
        let hash: BytesN<32> = env.crypto().sha256(&content_bytes).into();
        Self::bytes_to_hex(env, &hash.to_array())
    }

    fn bytes_to_hex(env: &Env, bytes: &[u8; 32]) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut result_bytes: [u8; 64] = [0; 64];
        for (i, byte) in bytes.iter().enumerate() {
            result_bytes[i * 2] = HEX[(byte >> 4) as usize];
            result_bytes[i * 2 + 1] = HEX[(byte & 0x0f) as usize];
        }
        String::from_bytes(env, &result_bytes)
    }

    fn mint_certificate(env: &Env, content_hash: &String, owner: &Address) -> Result<u64, ()> {
        let provenance_addr: Address = match env
            .storage()
            .instance()
            .get(&symbol_short!("PROV_ADR"))
        {
            Some(addr) => addr,
            None => return Err(()),
        };

        let provenance_client = provenance::Client::new(env, &provenance_addr);
        let details = provenance::CertificateDetails {
            content_hash: content_hash.clone(),
            metadata: String::from_str(env, ""),
        };

        env.authorize_as_current_contract(Vec::new(env));
        match provenance_client.try_mint(owner, &details) {
            Ok(result) => result.map_err(|_| ()),
            Err(_) => Err(()),
        }
    }
}

mod test;
