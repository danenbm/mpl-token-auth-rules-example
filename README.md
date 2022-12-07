# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/mpl-token-auth-rules-example`
RuleSet {
    operations: {
        Transfer: All {
            rules: [
                All {
                    rules: [
                        AdditionalSigner {
                            account: HnZiVuY53RbCg8B8PGkrfm5sCxw5uk62DwVYpACCTUdv,
                        },
                        AdditionalSigner {
                            account: A9yxnHeGooo8XQs4MgzmhvWECvR42HjcJhnz3SxgeCr7,
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
Create tx signature: H5gVgBVjRkuvb6KZcp3SD3QiKvuYUwNcbeW2faQ6MJN283WWWaV1gTXKx9yNGXFVssDZpK9UbHezR3swHqWkbMh
Validate tx signature: 39yktoynizTmhoH2CLHTBqRnqTzsqH5LuUs7rhZWSGfnqLMEMqkyA8s3MPiWLE5VbmTnToMCtY2hKn14zzAteJwH
```
