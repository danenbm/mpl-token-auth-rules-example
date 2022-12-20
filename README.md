# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/mpl-token-auth-rules-example`
RuleSet {
    rule_set_name: "test rule_set",
    owner: 3ksj6s23RLGEoanvEdnr3G7BQb2G93pMEavGbswQdgpt,
    operations: {
        0: All {
            rules: [
                All {
                    rules: [
                        AdditionalSigner {
                            account: 3ksj6s23RLGEoanvEdnr3G7BQb2G93pMEavGbswQdgpt,
                        },
                        AdditionalSigner {
                            account: HDG8JNhh5jkVVBzYcRTLJhRPLre8eBt4yHbxtp5dqX1q,
                        },
                    ],
                },
                Amount {
                    amount: 2,
                    operator: Eq,
                },
            ],
        },
    },
}
Create tx signature: 3S7rTpV4tqaArFKVazvHBmqpjRSLd5sURyEGUsaaqhPVdDFNoonfbmcgDQEdHKkzETSPozbVmWCcQVm3puuoXwDo
Validate tx signature: 3eWNWVemPgoGmhztLfQaz2WgZsk4XreyUbBqWEppZ1QxqdVWHkEZzu9rbPwTTn9adNb8ZBjBKoj9ggxXC3vo79Nq
```
