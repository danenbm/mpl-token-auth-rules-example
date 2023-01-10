# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/mpl-token-auth-rules-example`
RuleSet {
    version: 1,
    rule_set_name: "test rule_set",
    owner: GHRrH7iriu1RX7r73nS9ZkLpTkpD4tio8pebUhXej7CK,
    operations: {
        "OwnerTransfer": All {
            rules: [
                AdditionalSigner {
                    account: 8CDpTdPtCN7mLXJn6uekVYxzyYui71NsDX47xXXEqJaF,
                },
                Amount {
                    amount: 1,
                    operator: LtEq,
                    field: "Amount",
                },
            ],
        },
    },
}
Create tx signature: 2ZhCxWEromuJSzxtcB4738NULB62DsiaViMswTPqRzVwsRVm8NgqQKVeXkVGjLW5naUFahaHYRqUFNndJKnyi24i
Validate tx signature: 4bGUSi4uVtPU3eWjbnDTUik7tf96eeb6Ju3Y4iBb1UJtDFGfayypcbpNuRBHa1nj4SA7D2KCRvqJi9fvcSHjz5UN
```
