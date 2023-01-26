# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/mpl-token-auth-rules-example`
Metaplex Royalty RuleSet Dev: 4JvrhxRWDVp7vrZqPSqKYVmbvhbdXVEpCMkbjZaD8obB
RuleSetV1 {
    lib_version: 1,
    owner: 9jh2vGnz7y3DABkPUpFBCJQVzEaTNvKHjjtPQoFdjrvi,
    rule_set_name: "Metaplex Royalty RuleSet Dev",
    operations: {
        "Transfer:MigrationDelegate": Any {
            rules: [
                All {
                    rules: [
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
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
                    ],
                },
                All {
                    rules: [
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
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
                    ],
                },
            ],
        },
        "Transfer:Owner": Any {
            rules: [
                All {
                    rules: [
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
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
                    ],
                },
                All {
                    rules: [
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
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
                    ],
                },
            ],
        },
        "Transfer:TransferDelegate": Any {
            rules: [
                All {
                    rules: [
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
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
                    ],
                },
                All {
                    rules: [
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
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
                    ],
                },
            ],
        },
        "Transfer:SaleDelegate": Any {
            rules: [
                All {
                    rules: [
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
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
                    ],
                },
                All {
                    rules: [
                        Amount {
                            amount: 1,
                            operator: Eq,
                            field: "Amount",
                        },
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
                    ],
                },
            ],
        },
        "Transfer:WalletToWallet": All {
            rules: [
                Amount {
                    amount: 1,
                    operator: Eq,
                    field: "Amount",
                },
                IsWallet {
                    field: "Source",
                },
                IsWallet {
                    field: "Destination",
                },
            ],
        },
    },
}
TX Length: 679
Buffer tx signature: 5fsFzCy59gTr6MiFeM7UF7ZRv7HDai743jgcUsZkQCkJctgUeyMJMr8hKof4PNbJnHziYJH5UCPPEvCkLeomYsDi
TX Length: 679
Buffer tx signature: 5kne4P9qmN1uSvfpGxou61iPoij9bpcVvTY639wL7JGA1uZ6HL9tsYhgYLWEKzXckpbfbRtmKsZw7ZzQTD6xM4kp
TX Length: 679
Buffer tx signature: 4feL5cohnH4TDvw8y33DaTpZdz6pk7UgrYWkbBwa2nPu7zbgc11u1SUzTy11PBzzkdNEpniyvX7brdUvpqTjxMYu
TX Length: 499
Buffer tx signature: CvviHi7iWnAruUXrKXSk5o7kBzV3ZkPDkZ71Tvn4bjY3BNksQXS9wrBbUM9jqh3A3ryrn8t7jaPA1BS7MXSBUwZ
Create tx signature: cBEyAiUNQBdCW5cuTYgkmbAxyQYEhdBDbzHedtAY353VqaA3vsvt9PsoduG1Ddr6Kk1Pu6z16qZVBsKdjQ3TS2x
```
