#!/bin/bash
didkit generate-ed25519-key > follower.jwk
didkit generate-ed25519-key > followee.jwk
didkit generate-ed25519-key > blocked.jwk
vc-follow follow $(didkit key-to-did key -k follower.jwk) $(didkit key-to-did key -k followee.jwk) > unsigned_follow.json
vc-follow block $(didkit key-to-did key -k follower.jwk) $(didkit key-to-did key -k blocked.jwk) > unsigned_block.json
vc-follow post $(didkit key-to-did key -k follower.jwk) hello_world > unsigned_post.json

didkit vc-issue-credential --key-path follower.jwk \
                             -v $(didkit key-to-verification-method key --key-path follower.jwk) -p assertionMethod \
                             <unsigned_follow.json > follow.json

# didkit vc-issue-credential --key-path follower.jwk \
#                              -v "${$(didkit key-to-verification key --key-path follower.jwk)}" -p asserMethod \
#                              <unsigned_block.json > block.json

# didkit vc-issue-credential --key-path follower.jwk \
#                              -v "${$(didkit key-to-verification key --key-path follower.jwk)}" -p asserMethod \
#                              <unsigned_post.json > post.json

# didkit vc-verify-credential < follow.json
# didkit vc-verify-credential < block.json
# didkit vc-verify-credential < post.json
