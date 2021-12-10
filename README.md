# Social graph data as Verifiable Credentials

Prerequisites: Install didkit, through cargo or from source.

This repo contains a small CLI 

The quick way:
```bash
$ ./setup.sh
```

Now you can use the CLI to read the VCs:
```bash
$ demo-cli read follow.json
# "${follower.jwk} follows ${followee.jwk}
$ demo-cli read block.json
# "${follower.jwk} blocks ${block.jwk}
$ demo-cli read post.json
# "${follower.jwk} posted hello_world
```

You can also add additional relations, this would be one way to:
(TODO: Make an easier way)
```bash
$ didkit vc-issue-credential --key-path block.jwk \
                             -v "${$(didkit key-to-verification key --key-path block.jwk)}" -p asserMethod \
                             <$(demo-cli block --subject=$(didkit key-to-did key -k block.jwk) --blocks=$(didkit key-to-did key -k follower.jwk)) > block2.json \ 
                             && didkit vc-verify-credential < block2.json && demo-cli read block2.json
```

To understand what ths is doing, here is what `setup.sh` does:
1. Make the keys, from the root of the repo:
```bash
$ didkit generate-ed25518-key > follwer.jwk
$ didkit generate-ed25518-key > follwee.jwk
$ didkit generate-ed25518-key > blocked.jwk
```

2. Make the follow credential statement:
```bash
$ demo-cli follow --subject=$(didkit key-to-did key -k follower.jwk) --follows=$(didkit key-to-did key -k followee.jwk) > unsigned_follow.json
```
The block credential statement:
```bash
$ demo-cli block --subject=$(didkit key-to-did key -k follower.jwk) --blocks=$(didkit key-to-did key -k blocked.jwk) > unsigned_block.json
```
And the post statement.
```bash
$ demo-cli post --subject=$(didkit key-to-did key -k follower.jwk) --text=hello_world > unsigned_post.json
```

3. Then sign them all:
```bash
$ didkit vc-issue-credential --key-path follower.jwk \
                             -v "${$(didkit key-to-verification key --key-path follower.jwk)}" -p asserMethod \
                             <unsigned_follow.json > follow.json

$ didkit vc-issue-credential --key-path follower.jwk \
                             -v "${$(didkit key-to-verification key --key-path follower.jwk)}" -p asserMethod \
                             <unsigned_block.json > block.json

$ didkit vc-issue-credential --key-path follower.jwk \
                             -v "${$(didkit key-to-verification key --key-path follower.jwk)}" -p asserMethod \
                             <unsigned_post.json > post.json
```

4. Verify their authenticity:
```bash
$ didkit vc-verify-credential < follow.json
$ didkit vc-verify-credential < block.json
$ didkit vc-verify-credential < post.json
```


Or add additional relations. Maybe the blocked key spitefully blocks follower back using this easy one-liner:

### Schemas