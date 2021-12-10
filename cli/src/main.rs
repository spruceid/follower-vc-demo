use chrono::{SecondsFormat, Utc};
use std::path::Path;
use structopt::{clap::AppSettings, clap::ArgGroup, StructOpt};
use uuid::Uuid;

fn blocks(blocker: String, blockee: String) -> String {
    format!(
        r##"{{
      "@context": [
          "https://www.w3.org/2018/credentials/v1",
          {{
              "blockee": "/",
              "blocker": "/"
          }}
      ],
      "issuanceDate": "{}",
      "id": "urn:uuid:{}",
      "type": ["VerifiableCredential"],
      "credentialSubject": {{
          "blockee": {},
          "blocker": {}
      }},
      "issuer": {}
    }}"##,
        Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        Uuid::new_v4().to_string(),
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
              "followee": "/",
              "follower": "/"
          }}
      ],
      "issuanceDate": "{}",
      "id": "urn:uuid:{}",
      "type": ["VerifiableCredential"],
      "credentialSubject": {{
          "followee": {},
          "follower": {}
      }},
      "issuer": {}
    }}"##,
        Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        Uuid::new_v4().to_string(),
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
              "body": "/",
              "network: "/",
              "poster": "/",
              "topic": "/"
          }}
      ],
      "issuanceDate": "{}",
      "id": "urn:uuid:{}",
      "type": ["VerifiableCredential"],
      "credentialSubject": {{
          "body": {},
          "network": "example platform",
          "poster": {},
          "topic": "example topic"
      }},
      "issuer": {}
    }}"##,
        Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        Uuid::new_v4().to_string(),
        body,
        poster,
        poster,
    )
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
        _ => {
            panic!("Implement!")
        }
    }
}
