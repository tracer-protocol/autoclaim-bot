# Perpetual Pools Autoclaim Bot #

Rust implementation of an off-chain autoclaim bot. The bot accepts new blocks as
input, filtering for `CreateCommit` events with the autoclaim flag set. It then
produces relevant calls to `multiPaidClaim` as output.

```
                                    BLOCKCHAIN
-------------------------------------------------------------------------------
        |                                                    ^
        |                                                    |
        | ( blocks )                                         | ( calls )
        |                                                    |
        |                                                    |
        \                                                    |
         \                                                  /
          \                    +---------------+           /
           ------------------->| Autoclaim Bot |-----------
                               |               |
                               |               |
                               +---------------+
```

# Usage #

In order to invoke the bot in read-only mode,

```
$ cargo run -- --read-only true https://rpc.example.com
```

This will write JSON-formatted blocks from the RPC endpoint to standard output
as they are confirmed.

Note that only HTTP RPC interaction is supported currently (this will change
shortly).

# Development #

## Building ##

The full build chain is currently defined in the CI pipeline. To run locally,

```
$ cargo build && cargo test && cargo fmt -- --check && cargo clippy && cargo check && cargo audit
```

## Documentation ##

To view Rustdoc-formatted documentation,

```
$ cargo doc --open
```

