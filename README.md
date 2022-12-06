# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/mpl-token-auth-rules-example`
Payer balance: 1000000000
RuleSet {
    operations: {
        Transfer: All {
            rules: [
                All {
                    rules: [
                        AdditionalSigner {
                            account: AgC3hkhSaiENJEsa1dmZL3XJKGxAiE7LjXUu41umgz2a,
                        },
                        AdditionalSigner {
                            account: AgC3hkhSaiENJEsa1dmZL3XJKGxAiE7LjXUu41umgz2a,
                        },
                    ],
                },
                Amount {
                    amount: 2,
                },
            ],
        },
    },
}
Create tx signature: 5Sp15rxUTL5GcAXHHFX9PzqfdMtUBMEDLComYH3SmzmU9kFCih61JdqJLbQ2K49pGa9h5DhoVDexDSmgH3JfHgt3
Validate tx signature: 4hQvajdxWixK5jupj4ehh9gQELFKJHKarUqLspaNDtuF5em9MayJgPd3VdzjiD2a7hXWqbunxh2ioYykdQBuHmPN
```
