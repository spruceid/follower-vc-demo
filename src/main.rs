use anyhow::{anyhow, Result};
use didkit::{JWTOrLDPOptions, ProofFormat};
use siwe::eip4361::*;
use ssi::vc::Credential;
use std::fs::{create_dir_all, read_to_string};
use std::io::prelude::*;
use std::path::Path;
use walkdir::WalkDir;

// TODO: Make a sane default + passed in.
const home: &str = ".";

fn registry_path() -> &Path {
    Path::new(home).join(Path::new("registery")).as_path()
}

fn stat_base(base: &Path) -> Result<()> {
    let blocked = base.join(Path::new("blocked"));
    let followees = base.join(Path::new("followees"));
    let followers = base.join(Path::new("followers"));
    let nonces = base.join(Path::new("nonces"));
    if !base.is_dir() {
        create_dir_all(blocked)?;
        create_dir_all(followees)?;
        create_dir_all(followers)?;
        create_dir_all(nonces)?;
    } else {
        if !blocked.is_dir() {
            create_dir_all(blocked)?;
        }
        if !followees.is_dir() {
            create_dir_all(followees)?;
        }
        if !followers.is_dir() {
            create_dir_all(followers)?;
        }
        if !nonces.is_dir() {
            create_dir_all(nonces)?;
        }
    };

    Ok(())
}

async fn verify_credential(vc_string: String, proof_options: String) -> Result<String, Error> {
    let options: JWTOrLDPOptions = serde_json::from_str(&proof_options)?;
    let proof_format = options.proof_format.unwrap_or_default();
    let resolver = DID_METHODS.to_resolver();
    let result = match proof_format {
        ProofFormat::JWT => {
            Credential::verify_jwt(&vc_string, Some(options.ldp_options), resolver).await
        }
        ProofFormat::LDP => {
            let vc = Credential::from_json_unsigned(&vc_string)?;
            vc.verify(Some(options.ldp_options), resolver).await
        }
        _ => Err(anyhow!(proof_format.to_string()))?,
    };
    let result_json = serde_json::to_string(&result)?;
    Ok(result_json)
}

fn login(message: Message) -> Result<User> {
    Ok(User::from_address(validate_session(message)?)?)
}

fn validate_session(message: Message) -> Result<String> {
    let address = format!(
        "0x{}",
        std::str::from_utf8(&message.address)?.to_lowercase()
    );

    let base = registry_path().join(Path::new(&address)).as_path();
    stat_base(base)?;

    if !message.valid_now() {
        return Err(anyhow!("Invalid SIWE Message"));
    };

    let next_nonce_path = base
        .join(Path::new("nonces"))
        .join(Path::new(format!("{}.txt", &message.nonce)));

    if next_nonce_path.is_file() {
        let raw_message = read_to_string(next_nonce_path)?;
        if message.to_string() != raw_message {
            return Err(anyhow!("Mismatched message"));
        };
    } else {
        let raw_message = message.to_string();
        let mut f = std::fs::File::create(next_nonce_path)?;
        f.write_all(raw_message.as_bytes())?;
    };

    Ok(address)
}

fn list_users(filter_addr: Option<String>) -> Vec<String> {
    WalkDir::new(registry_path())
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| match entry.metadata() {
            Err(_) => false,
            Ok(m) => m.is_dir(),
        })
        .filter_map(|entry| entry.file_name().to_str())
        .map(|s| s.to_string())
        .filter(|s| Some(s) == filter_addr.as_ref())
        .collect()
}

struct User {
    pub address: String,
    pub blocked: Vec<String>,
    pub followed_by: Vec<String>,
    pub follows: Vec<String>,
}

// TODO: Add test for correct type of VC in correct folder.
fn valid_vc_addresses(p: &Path) -> Vec<String> {
    WalkDir::new(p)
        .into_iter()
        .filter_map(|entry| entry.ok())
        // TODO: Turn into filter_map:
        .filter(|entry| match entry.metadata() {
            // TODO: Check for .json ext
            Err(_) => false,
            Ok(m) => match m.is_file() {
                false => false,
                // TODO: Add VC serialization and validation here.
                _ => match read_to_string(p.join(Path::new(entry.file_name()))) {
                    Err(_) => false,
                    Ok(m) => {
                        let vc: Credential = match serde_json::from_str(&m) {
                            Ok(x) => x,
                            Err(_) => return false,
                        };

                        true
                    }
                },
            },
        })
        .filter_map(|entry| entry.file_name().to_str())
        .map(|s| s.trim_end_matches(".json").to_string())
        .collect()
}

impl User {
    fn from_address(address: String) -> Result<Self> {
        let base = registry_path().join(Path::new(&address)).as_path();
        let blocked = base.join(Path::new("blocked"));
        let followees = base.join(Path::new("followees"));
        let followers = base.join(Path::new("followers"));

        Ok(User {
            address,
            blocked: valid_vc_addresses(&blocked),
            followed_by: valid_vc_addresses(&followers),
            follows: valid_vc_addresses(&followees),
        })
    }
}

fn main() {
    println!("Hello, world!");
}
