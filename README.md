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
           --------------------| Autoclaim Bot |-----------
                               |               |
                               |               |
                               +---------------+
```

