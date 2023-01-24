# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/mpl-token-auth-rules-example`
Metaplex Royalty RuleSet Dev: 1CfDY5sYBnspaXvjnN3y9WRdaoD5v3HXrZDrWhjZZTN
RuleSetV1 {
    lib_version: 1,
    rule_set_name: "Metaplex Royalty RuleSet Dev",
    owner: G4Pfn349VYQ4WFU1HMXCxKXnXNdxx1s1byoqXSrnq4Fa,
    operations: {
        "Transfer:TransferDelegate": All {
            rules: [
                Amount {
                    amount: 1,
                    operator: Eq,
                    field: "Amount",
                },
                Any {
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
                            ],
                        },
                        All {
                            rules: [
                                IsWallet {
                                    field: "Source",
                                },
                                IsWallet {
                                    field: "Destination",
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        "Transfer:MigrationDelegate": All {
            rules: [
                Amount {
                    amount: 1,
                    operator: Eq,
                    field: "Amount",
                },
                Any {
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
                            ],
                        },
                        All {
                            rules: [
                                IsWallet {
                                    field: "Source",
                                },
                                IsWallet {
                                    field: "Destination",
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        "Transfer:Owner": All {
            rules: [
                Amount {
                    amount: 1,
                    operator: Eq,
                    field: "Amount",
                },
                Any {
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
                            ],
                        },
                        All {
                            rules: [
                                IsWallet {
                                    field: "Source",
                                },
                                IsWallet {
                                    field: "Destination",
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        "Transfer:SaleDelegate": All {
            rules: [
                Amount {
                    amount: 1,
                    operator: Eq,
                    field: "Amount",
                },
                Any {
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
                            ],
                        },
                        All {
                            rules: [
                                IsWallet {
                                    field: "Source",
                                },
                                IsWallet {
                                    field: "Destination",
                                },
                            ],
                        },
                    ],
                },
            ],
        },
    },
}
TX Length: 679
Buffer tx signature: pER31zTKqLkHUfeagRX68KgLxLRGTbGM8u1dnBjuGjSotbHLAxLvdqtHJTCWKJhXHRvfrwHbQGQqRL5HNX1wTDt
TX Length: 679
Buffer tx signature: 5vsS75nBYmomJxJa3JNRf4FQK6YMqMFk4SQvdNhMM6kzANgQnUvLhWtaTz1NfpkqwG4As2Ysd9wvDEf8XTZhUSQT
TX Length: 679
Buffer tx signature: 3187x2rk5shQbogY9iB8iwQgcjr21ymQw6n3eaTkYmnKD4ETYdpQt5KyMxtM5NrGpMzr6kW81WJvQQCWtasTJruF
TX Length: 543
Buffer tx signature: 4SqkWiuQebndQ1j5DvaLZdPnGrgBanLsvLZoFneJWjhAjX818N1CCmUvpKs7sEivCqwHeR4Bd8KxgvZhS2h8cDCz
Create tx signature: QHZJVNaRFENsZQ3TQBYNSU48mU2rDz55U5puVgtYgHxPx3YXYEG8o8P56SPM4dD89AkLekLsRwZBtAY2EYSqMPA
```
