# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/token-authorization-rules-example`
Payer balance: 1000000000
RuleSet {
    operations: {
        Transfer: All {
            rules: [
                All {
                    rules: [
                        AdditionalSigner {
                            account: GRoaBnonZVpr28wdk7aJ4wzcFZNukJ4J33MfPCPyWMHi,
                        },
                        AdditionalSigner {
                            account: GRoaBnonZVpr28wdk7aJ4wzcFZNukJ4J33MfPCPyWMHi,
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
Create tx signature: 5ahDHDjaZsrBEkrjhHRRmNi1RukrqgwXZQKz1tKCbDa7GJ5cEyBufcNsQuP7szRpJsdKXG67QwpsFo5ZaRu7ean9
Validate tx signature: 4QayBBgYF7kqHV7deVZFWuHa6Arkv7mjn3eUGURyFTYDPzvFjepb7p4xW92UTqRsjuncdnfz47nPdDr5dNvFevGi
```
