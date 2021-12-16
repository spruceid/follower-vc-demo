use chrono::{SecondsFormat, Utc};
use serde;
use serde_json::Value;
use ssi::{one_or_many::OneOrMany, vc::Credential};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use structopt::{clap::AppSettings, clap::ArgGroup, StructOpt};
use uuid::Uuid;

fn blocks(blocker: String, blockee: String) -> String {
    format!(
        r##"{{
      "@context": [
          "https://www.w3.org/2018/credentials/v1",
          {{
              "blockee": "http://example.com/blockee",
              "blocker": "http://example.com/blocker"
          }}
      ],
      "issuanceDate": "{}",
      "id": "urn:uuid:{}",
      "type": ["VerifiableCredential"],
      "credentialSubject": {{
          "id": "{}",
          "blockee": "{}",
          "blocker": "{}"
      }},
      "issuer": "{}"
    }}"##,
        Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        Uuid::new_v4().to_string(),
        blocker,
        blockee,
        blocker,
        blocker,
    )
}

fn follows(follower: String, followee: String) -> String {
    format!(
        r##"{{
      "@context": [
          "https://www.w3.org/2018/credentials/v1",
          {{
              "followee": "http://example.com/followee",
              "follower": "http://example.com/follower"
          }}
      ],
      "issuanceDate": "{}",
      "id": "urn:uuid:{}",
      "type": ["VerifiableCredential"],
      "credentialSubject": {{
          "id": "{}",
          "followee": "{}",
          "follower": "{}"
      }},
      "issuer": "{}"
    }}"##,
        Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        Uuid::new_v4().to_string(),
        follower,
        followee,
        follower,
        follower,
    )
}

fn post(poster: String, body: String) -> String {
    format!(
        r##"{{
      "@context": [
          "https://www.w3.org/2018/credentials/v1",
          {{
              "body": "http://example.com/body",
              "network": "http://example.com/network",
              "poster": "http://example.com/poster",
              "topic": "http://example.com/topic"
          }}
      ],
      "issuanceDate": "{}",
      "id": "urn:uuid:{}",
      "type": ["VerifiableCredential"],
      "credentialSubject": {{
          "id": "{}",
          "body": "{}",
          "network": "example platform",
          "poster": "{}",
          "topic": "example topic"
      }},
      "issuer": "{}"
    }}"##,
        Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        Uuid::new_v4().to_string(),
        poster,
        body,
        poster,
        poster,
    )
}

fn read(p: &Path) -> Result<(), ()> {
    let mut f = File::open(p).map_err(|_e| ())?;
    let mut d = String::new();
    f.read_to_string(&mut d).map_err(|_e| ())?;

    let j: Credential = serde_json::from_str(&d).map_err(|_e| ())?;
    match j.credential_subject {
        OneOrMany::One(cs) => match cs.property_set {
            None => Err(()),
            Some(ps) => read_property_set(ps),
        },
        OneOrMany::Many(_) => return Err(()),
    }
}

fn read_property_set(ps: HashMap<String, Value>) -> Result<(), ()> {
    match ps.get("follower") {
        Some(fr) => match ps.get("followee") {
            Some(fe) => {
                println!("{} follows {}", fr, fe);
                return Ok(());
            }
            None => return Err(()),
        },
        None => {}
    };

    match ps.get("blocker") {
        Some(br) => match ps.get("blockee") {
            Some(be) => {
                println!("{} blocks {}", br, be);
                return Ok(());
            }
            None => return Err(()),
        },
        None => {}
    };

    match ps.get("poster") {
        Some(pr) => match ps.get("body") {
            Some(bo) => {
                println!("{} posts {}", pr, bo);
                return Ok(());
            }
            None => return Err(()),
        },
        None => {}
    };

    return Err(());
}

#[derive(StructOpt, Debug)]
pub enum VCFollower {
    Block { subject: String, blockee: String },
    Follow { subject: String, followee: String },
    Post { subject: String, body: String },
    Read { path: String },
}

fn main() {
    let opt = VCFollower::from_args();
    match opt {
        VCFollower::Block { subject, blockee } => {
            println!("{}", blocks(subject, blockee))
        }
        VCFollower::Follow { subject, followee } => {
            println!("{}", follows(subject, followee))
        }
        VCFollower::Post { subject, body } => {
            println!("{}", post(subject, body))
        }
        VCFollower::Read { path } => {
            match read(Path::new(&path)) {
                Ok(_) => {}
                Err(_) => println!("Failed to read VC"),
            };
        }
    }
}
