# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/mpl-token-auth-rules-example`
Metaplex All Pass RuleSet: D4YHFZPWASGpvBDJSUrPtqZqxTgTm7eL5rikBY9Y5dwf
RuleSetV1 {
    lib_version: 1,
    owner: 9jh2vGnz7y3DABkPUpFBCJQVzEaTNvKHjjtPQoFdjrvi,
    rule_set_name: "Metaplex All Pass RuleSet",
    operations: {
        "Transfer:MigrationDelegate": Pass,
        "Transfer:SaleDelegate": Pass,
        "Transfer:Owner": Pass,
        "Transfer:WalletToWallet": Pass,
        "Transfer:TransferDelegate": Pass,
    },
}
TX Length: 401
Buffer tx signature: 4HjRQcmv5JC1H7CA9ERGpRSoWre7oAREvJjfyRV5UrK34oU2zM5BBq5KMCRH7W4ktHKk2MQ6rPAoK6QPpbT657uy
Create tx signature: 39bHo4wqFq8GsScwwdhkeYjADm4cN4Diggt8YwoxS6Ab6RZe5qFHj1rJmqsv2raNJyK4ffuqFStBwPxSCzRjDAoJ
```
