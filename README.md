# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/mpl-token-auth-rules-example`
RuleSet {
    rule_set_name: "test rule_set",
    owner: 5fDgWEkum2epqbo1d7DRnMqkq7mJj1tFtg3tXEZykUKH,
    operations: {
        0: All {
            rules: [
                All {
                    rules: [
                        AdditionalSigner {
                            account: 5fDgWEkum2epqbo1d7DRnMqkq7mJj1tFtg3tXEZykUKH,
                        },
                        AdditionalSigner {
                            account: AroD4vkrA27iXWpVvMiYZjVNWLNf5hTFvDUCsXYjJVof,
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
Create tx signature: aQttfiotAkzdAUAoNJWGztdVeskhX4oduiNQ7kq4tSb9bXVQspk4DRKsF9Cd3ogvjx6TV72RUE86TwD3XkyEZfA
Validate tx signature: 2K6sRG5X4V6DbVYvgZsaRv5NFp3ZEvA32NFDiSfMWAvYqNpXCNg1QogJcJKXV6gY6dyBGRWPaDKeFg5qgZw5RNkP
```
