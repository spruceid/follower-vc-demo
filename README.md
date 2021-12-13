# Social graph data as Verifiable Credentials

## Quickstart

Prerequisites: 
- `cargo` and `didkit`. 
- WSL2 users may need to install `pkg-config` or another standard library for some dependencies like `openssl`.
- `didkit` can be installed by running `cargo install didkit-cli` from the root directory.

This repo contains a small CLI that will need to be installed, will install as `vc-follow`. 

Install it like so from the root of the repo:
```bash
$ cargo install --path ./cli
```

To set up some basic users and relationships:
```bash
$ ./setup.sh
```

If no errors are reported in the `errors` array in the output, then three valid VCs will been created:
- `follow.json` represents a "follow" action as a VC
- `block.json` represents a "block" action as a VC
- `post.json` represents a "post" action as a VC
- In all three, "Alice" and "Bob" are represented by Ethereum accounts: 


Now you can use the CLI to read the VCs:
```bash
$ vc-follow read follow.json
# "${follower.jwk} follows ${followee.jwk}
$ vc-follow read block.json
# "${follower.jwk} blocks ${block.jwk}
$ vc-follow read post.json
# "${follower.jwk} posted hello_world
```

The `/example/` folder contents are provided for reference, and will be overwritten with fresh credentials with updated ephemeral keys and timestamps each time `setup.sh` is run.

## Setup.sh step-by-step

To understand what ths is doing, here is what `setup.sh` does:
1. Make the keys, from the root of the repo:
```bash
$ didkit generate-ed25519-key > follower.jwk
$ didkit generate-ed25519-key > followee.jwk
$ didkit generate-ed25519-key > blocked.jwk
```

2. Make the follow credential statement:
```bash
$ vc-follow follow $(didkit key-to-did key -k follower.jwk) $(didkit key-to-did key -k followee.jwk) > unsigned_follow.json
```
The block credential statement:
```bash
$ vc-follow block $(didkit key-to-did key -k follower.jwk) $(didkit key-to-did key -k blocked.jwk) > unsigned_block.json
```
And the post statement.
```bash
$ vc-follow post $(didkit key-to-did key -k follower.jwk) hello_world > unsigned_post.json
```

3. Then sign them all:
```bash
$ didkit vc-issue-credential --key-path follower.jwk \
                             -v $(didkit key-to-verification-method key --key-path follower.jwk) -p assertionMethod \
                             <unsigned_follow.json > follow.json

$ didkit vc-issue-credential --key-path follower.jwk \
                             -v $(didkit key-to-verification-method key --key-path follower.jwk) -p assertionMethod \
                             <unsigned_block.json > block.json

$ didkit vc-issue-credential --key-path follower.jwk \
                             -v $(didkit key-to-verification-method key --key-path follower.jwk) -p assertionMethod \
                             <unsigned_post.json > post.json

```

4. Verify their authenticity:
```bash
$ didkit vc-verify-credential < follow.json
$ didkit vc-verify-credential < block.json
$ didkit vc-verify-credential < post.json
```
### Schemas
Follow
```json
{
    "type": "https://w3c-ccg.github.io/vc-json-schemas/schema/2.0/schema.json",
    "id": "did:web:demo.spruceid.com?schemaId=76e6e52b-681e-4952-b1f5-9b670144a5ba&version=1.0",
    "version": "1.0",
    "author": "did:web:demo.spruceid.com",
    "authored": "2021-12-07T22:33:04.309Z",
    "name": "Rebase Follow",
    "schema": {
        "$id": "rebase-follow-1.0"
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "description": "Follow",
        "type": "object",
        "properties": {
            "followee": {
                "type": "string",
                 "description": "A followee referenced by URI such as blockchainAddress, DID, etc"
            },
            "follower": {
                "type": "string",
                 "description": "A follower referenced by URI such as blockchainAddress, DID, etc"
            }
        },
        "required": [
            "followee",
            "follower"
        ],
        "additionalProperties": true
    }
}
```

Block
```json
{
    "type": "https://w3c-ccg.github.io/vc-json-schemas/schema/2.0/schema.json",
    "id": "did:web:demo.spruceid.com?schemaId=5957a35f-8af6-4343-83c4-4198e875bad9&version=1.0",
    "version": "1.0",
    "author": "did:web:demo.spruceid.com",
    "authored": "2021-12-07T22:33:04.310Z",
    "name": "Rebase Block",
    "schema": {
        "$id": "rebase-block-1.0"
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "description": "Block",
        "type": "object",
        "properties": {
            "blockee": {
                "type": "string",
                 "description": "A followee referenced by URI such as blockchainAddress, DID, etc"
            },
            "blocker": {
                "type": "string",
                 "description": "A followee referenced by URI such as blockchainAddress, DID, etc"
            }
        },
        "required": [
            "blockee",
            "blocker"
        ],
        "additionalProperties": true
    }
}
```
Post
```json
{
    "type": "https://w3c-ccg.github.io/vc-json-schemas/schema/2.0/schema.json",
    "id": "did:web:demo.spruceid.com?schemaId=76e6e52b-681e-4952-b1f5-9b670144a5ba&version=1.0",
    "version": "1.0",
    "author": "did:web:demo.spruceid.com",
    "authored": "2021-12-07T22:33:04.309Z",
    "name": "Rebase Post",
    "schema": {
        "$id": "rebase-post-1.0"
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "description": "Post",
        "type": "object",
        "properties": {
            "poster": {
                "type": "string",
                 "description": "A poster referenced by URI such as blockchainAddress, DID, etc"
            },
            "body": {
                "type": "string",
                 "description": "The body of the post"
            }
        },
        "required": [
            "poster",
            "body"
        ],
        "additionalProperties": true
    }
}
```

### Further work

To-do list:
- build out proper JSON-LD Context
- replace arbitrary ephemeral keys with session keys derived from `credentialSubject.id`s  expressed as blockchain addresses (i.e. did:pkh instead of did:key)
- add option for additional relations

```bash
$ didkit vc-issue-credential --key-path block.jwk \
                             -v $(didkit key-to-verification-method key --key-path block.jwk) -p assertionMethod \
                             <$(vc-follow block $(didkit key-to-did key -k block.jwk) $(didkit key-to-did key -k follower.jwk)) > block2.json \ 
                             && didkit vc-verify-credential < block2.json && vc-follow read block2.json
```
