# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/mpl-token-auth-rules-example`
RuleSet {
    rule_set_name: "test rule_set",
    owner: 6jo3MeqjmspMsFdK7L9QakwjyV3YyVgskUtx2rugAE4j,
    operations: {
        Transfer: All {
            rules: [
                All {
                    rules: [
                        AdditionalSigner {
                            account: 6jo3MeqjmspMsFdK7L9QakwjyV3YyVgskUtx2rugAE4j,
                        },
                        AdditionalSigner {
                            account: 8vqCFPbeBSKQRmafNRQcZ94AksJynXvk9xVqoH1nkGt8,
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
Create tx signature: 4jKBP3AVu7RHT3z3Gcx6tYANSfYq2E9tCJu6EAdLUHjr7ZvCPiuAoRnH9wD9UrmatdokSy7TfKBSd3dzZfNU5qBN
Validate tx signature: 52u9EgLLSpFnEP5yoJcx6uVwP1B6Mu974A2R82oS1SLw5DMQdeCsAycyZc1v5wze1rSTa9tA7ecMPcdp9cQVSqRE
```
