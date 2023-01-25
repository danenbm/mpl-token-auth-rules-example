# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/mpl-token-auth-rules-example`
Metaplex Royalty RuleSet Dev: A6VicqkGd4J9Bk5B3j8isT5yWHJ2yPMeGBDWM6PY1Zgf
RuleSetV1 {
    lib_version: 1,
    owner: 7XWRDjCtgqoMXfQxEjzE7jURHYHQ1AKJti5vYNpuqNDZ,
    rule_set_name: "Metaplex Royalty RuleSet Dev",
    operations: {
        "Transfer:WalletToWallet": All {
            rules: [
                IsWallet {
                    field: "Source",
                },
                IsWallet {
                    field: "Destination",
                },
                Amount {
                    amount: 1,
                    operator: Eq,
                    field: "Amount",
                },
            ],
        },
        "Transfer:TransferDelegate": Any {
            rules: [
                All {
                    rules: [
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Source",
                        },
                        PDAMatch {
                            program: None,
                            pda_field: "Source",
                            seeds_field: "SourceSeeds",
                        },
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
                    ],
                },
                All {
                    rules: [
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Destination",
                        },
                        PDAMatch {
                            program: None,
                            pda_field: "Destination",
                            seeds_field: "DestinationSeeds",
                        },
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
                    ],
                },
            ],
        },
        "Transfer:Owner": Any {
            rules: [
                All {
                    rules: [
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Source",
                        },
                        PDAMatch {
                            program: None,
                            pda_field: "Source",
                            seeds_field: "SourceSeeds",
                        },
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
                    ],
                },
                All {
                    rules: [
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Destination",
                        },
                        PDAMatch {
                            program: None,
                            pda_field: "Destination",
                            seeds_field: "DestinationSeeds",
                        },
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
                    ],
                },
            ],
        },
        "Transfer:SaleDelegate": Any {
            rules: [
                All {
                    rules: [
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Source",
                        },
                        PDAMatch {
                            program: None,
                            pda_field: "Source",
                            seeds_field: "SourceSeeds",
                        },
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
                    ],
                },
                All {
                    rules: [
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Destination",
                        },
                        PDAMatch {
                            program: None,
                            pda_field: "Destination",
                            seeds_field: "DestinationSeeds",
                        },
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
                    ],
                },
            ],
        },
        "Transfer:MigrationDelegate": Any {
            rules: [
                All {
                    rules: [
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Source",
                        },
                        PDAMatch {
                            program: None,
                            pda_field: "Source",
                            seeds_field: "SourceSeeds",
                        },
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
                    ],
                },
                All {
                    rules: [
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Destination",
                        },
                        PDAMatch {
                            program: None,
                            pda_field: "Destination",
                            seeds_field: "DestinationSeeds",
                        },
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
                    ],
                },
            ],
        },
    },
}
TX Length: 679
Buffer tx signature: 2pm3VvGgKzGU5b6EUodF11bVTFv2x6xJgzRKRjuFCL99TgznytvAaoQy3gycWnBxYMEQ9UCHdxpgieDqSsgG6c9s
TX Length: 679
Buffer tx signature: 41GMj6JXairCBA4DmZMRB77kFBezFkRk43hY3qYwan3HcomzMpuVPPyXq9ib9JmmhB9DbZS3XUw4aRuCoBQvy2c4
TX Length: 679
Buffer tx signature: f6PK9BAGTbZFhcZNMyEv3sTzb4x773i74iTrprZbiSfgiUrzGjUHCRv47h8ZZpB4spAWKPzkmazCsnneNWwb9BM
TX Length: 497
Buffer tx signature: 67ZZ1Wvb2XdXmMX741ErCiH7YQVjyUu9emaXhJswFjR6uPD3mFyS3rcwPSWMucdQ94PSWauxCC8AKLXTdGdGh2Zq
Create tx signature: 43CxHM5nnWfUr3AWNJW5uWKdQYCY1a3rGCkJnKcwLhpFVkagUucYXyFECcKUJzbhDEMKYPJ92mmr71ahPWnknjzP
```
