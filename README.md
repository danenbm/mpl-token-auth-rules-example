# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/mpl-token-auth-rules-example`
RuleSet {
    version: 1,
    rule_set_name: "test rule_set",
    owner: 8qTkAVU5A4nwtudA1ggfq4CAKb341a9tUfdBF34pxnD4,
    operations: {
        "Transfer": All {
            rules: [
                AdditionalSigner {
                    account: 4QCHqUtYVhkqeN21uWhsEa7UFDGW1XmKx9pNNYkskrMS,
                },
                Amount {
                    amount: 1,
                    operator: LtEq,
                },
            ],
        },
    },
}
Create tx signature: 5Q6Rtit3cmdFoa54CkgtxH7MS2ywp2pUgmxf9UFMh1QCYGcL5sisQGDakU3hpWcJQnVpoC1axcxBnZToRJK7MAWs
Validate tx signature: aQbFbcbPdipw9noqWCb4KCePojmDM4Mzx1RCL3W5SQPNCxwMfK1qN8a8DFUBGDVQ1DA2gsJsamMcdegjkAg4AVc
```
