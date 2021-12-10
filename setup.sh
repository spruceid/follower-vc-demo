#!/bin/bash
didkit generate-ed25519-key > follower.jwk
didkit generate-ed25519-key > follpwee.jwk
didkit generate-ed25519-key > blocked.jwk
# vc-follow follow --subject=$(didkit key-to-did key -k follower.jwk) --follows=$(didkit key-to-did key -k followee.jwk) > unsigned_follow.json
# vc-follow block --subject=$(didkit key-to-did key -k follower.jwk) --blocks=$(didkit key-to-did key -k blocked.jwk) > unsigned_block.json
# vc-follow post --subject=$(didkit key-to-did key -k follower.jwk) --text=hello_world > unsigned_post.json

# didkit vc-issue-credential --key-path follower.jwk \
#                              -v "${$(didkit key-to-verification key --key-path follower.jwk)}" -p asserMethod \
#                              <unsigned_follow.json > follow.json

# didkit vc-issue-credential --key-path follower.jwk \
#                              -v "${$(didkit key-to-verification key --key-path follower.jwk)}" -p asserMethod \
#                              <unsigned_block.json > block.json

# didkit vc-issue-credential --key-path follower.jwk \
#                              -v "${$(didkit key-to-verification key --key-path follower.jwk)}" -p asserMethod \
#                              <unsigned_post.json > post.json

# didkit vc-verify-credential < follow.json
# didkit vc-verify-credential < block.json
# didkit vc-verify-credential < post.json
