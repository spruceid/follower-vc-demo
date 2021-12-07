use anyhow::{anyhow, Result};
use siwe::eip4361::*;
use std::fs::create_dir_all;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    println!("Hello, world!");
}

fn login(message: Message) -> Result<(User, Vec<String>)> {
    if !message.valid_now() {
        return Err(anyhow!("Invalid SIWE Message"));
    };

    let address = format!(
        "0x{}",
        std::str::from_utf8(&message.address)?.to_lowercase()
    );

    create_dir_all(Path::new(format!("./registry/{}", address)))?;
    // TODO: Create + Save Session VC here

    Ok((
        User::from_address(address)?,
        list_users().into_iter().filter(|a| a == &address).collect(),
    ))
}

fn list_users() -> Vec<String> {
    WalkDir::new("./registry")
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| match entry.metadata() {
            Err(_) => false,
            Ok(m) => m.is_dir(),
        })
        .filter_map(|entry| entry.file_name().to_str())
        .map(|s| s.to_string())
        .collect()
}

struct User {
    pub address: String,
    pub followed_by: Vec<String>,
    pub follows: Vec<String>,
}

impl User {
    fn from_address(address: String) -> Result<Self> {
        let mut u = User {
            address,
            followed_by: Vec::new(),
            follows: Vec::new(),
        };

        let rels: Vec<String> = WalkDir::new("./followers")
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| match entry.metadata() {
                Err(_) => false,
                Ok(m) => m.is_file(),
            })
            .filter_map(|entry| entry.file_name().to_str())
            .map(|s| s.to_string())
            .collect();

        for rel in rels {
            let trimmed = rel.trim_end_matches(".json");
            let v: Vec<&str> = trimmed.split("_").collect();
            if v.len() != 2 {
                return Err(anyhow!("Malformed follower data detected, name: {}", rel));
            };

            let follower = v[0].to_lowercase();
            let followee = v[1].to_lowercase();

            if follower == u.address || followee == u.address {
                // TODO: DIDKit check VC here.
                if follower == u.address {
                    u.follows.push(followee);
                } else if followee == u.address {
                    u.followed_by.push(follower);
                };
            }
        }

        Ok(u)
    }
}
