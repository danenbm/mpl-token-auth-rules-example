# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/mpl-token-auth-rules-example`
Metaplex Royalty RuleSet Dev: GAGPGnriiYDb4vukXcfkPhr2ErW6udCx3rhx8EAu7J1d
RuleSetV1 {
    lib_version: 1,
    owner: 9bupd3CqcCqEzDcpjQH4xTUJFtaBNdG4kcJn7EdSd2MH,
    rule_set_name: "Metaplex Royalty RuleSet Dev",
    operations: {
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
Buffer tx signature: 52Xb6owPtmjEvHPMQ1AoT8sGHWPank8EQotWr3T2tmBNgyNsQBG7RVw9TKdUTpdMvPkbYHhcDx8u8k3wSdDE9kz5
TX Length: 679
Buffer tx signature: 41GQwghKZyGiupw9QBPfbmaYnrW8btbbuLhvqexTN8BAYeXcw6tvPkfDM386awZ388nFYu5UzFK9qpRVcwPwYLG1
TX Length: 679
Buffer tx signature: ps1BL5jaP91Wn4Yyrf1dfzxnJ9SfPuo4f9whsAdES8CHJauxTr15RnBb3Q1mAgzhim2ufyXY6fS3wtU6NuAmUVy
TX Length: 499
Buffer tx signature: 4vQ1BCBGAxwQtkEd5TWQFm5sV4PbuBz9YH5ZZ37wyuita6sTPqKJRgxPjUQjJf214K53mLauJsFziMjeMNetwvaL
Create tx signature: 33FTEoVHuCFbGjcxYPHpacKNcmJn7uDKJbcafCofLeBWQU5KoibYLs87zGvAgHUoe4vUX68HAo15tyFC1WJQpSJ3
```
