# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/mpl-token-auth-rules-example`
RuleSet {
    operations: {
        Transfer: All {
            rules: [
                All {
                    rules: [
                        AdditionalSigner {
                            account: AXFLbZYT8yhzpbNMe5AmrnEYWmeQGsTjmyuBEaiqXjCb,
                        },
                        AdditionalSigner {
                            account: AXFLbZYT8yhzpbNMe5AmrnEYWmeQGsTjmyuBEaiqXjCb,
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
Create tx signature: 2u2Nso3c8RDGf2Xhp7XFKEy8e9nKAciVBaZsXemZzBGWkxkgmr8pzJdHLG31ieX1K5EDtjoDDh8J8CoUm7aY9h5i
Validate tx signature: 3sfyUh3dXPPKk3dsgQqgPhdmUx6bWE3N9753nFRZNUueyYQhNULWAx1oqtvbB1XHt7rZwX63qrEod2HcKc6cGkoa
```
