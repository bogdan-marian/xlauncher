# xlauncher

# dev notes

[cargo install cargo-edit](https://www.reddit.com/r/rust/comments/u6qrbd/cargo_now_has_native_support_for_the_cargo_add/)
This might be useful but for now it failed the installation on my workstation

```bash
# update rust
erdpy deps install rust --overwrite
cargo clean && cargo build

```

# dev log

- current active develop branch: `dev-kent-1`

# feedback session 1

# xswap

## rename modules:

- lib -> xswap //this is different
- admin -> admin
- config -> config
- event -> event
- state -> data //this is different
- storage -> storage
- admin -> admin
- user -> user

## we need a system to know this

- how many lottery token should be sent related to Offer creator
- how many lottery token should be sent to person that accepts offer.
- Extra info: We send tokens only in integer incremental steps
- how do we handle this
    - for each 0.5 egld you get 1 RTOKEN
    - maker A create an offer for 4 egld, (swhould receive 8 tokens)
        - taker a accepts 1 egld
        - taker b accepts 0.4
        - taker c accepts 0.1
        - We send the token at the end when the offer is filed or canceled.

## actor name alternative that participate in a swap

- [from, to] // this is clearly the best
- [maker, taker]
- [creator, acceptor]

## create interactions and tests

We need interactions and tests

## available tokens

It is possible that we will need to create a second token list that will be pared with the tokens from available_tokens
list.
Name sudjestion for this list: ?

- available_pairing_tokens

# xraffle

## module names rename

- lib -> xraffle //this is different
- state -> data //this is different

## end_timestamp

we keep end_timestamp only for display purposes

## how the lottery selects the winner

Notes for Bogdan: For now one user can only have 1 winning ticket.

## create interactions and tests

We need interactions and tests


## XRF tokens (ticket tokens) will be sent to Treasury Wallet