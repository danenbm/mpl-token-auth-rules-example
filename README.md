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
    },
}
TX Length: 679
Buffer tx signature: sLz9aVvLhZJh83V7PAivfvqAWLK4YNWUzQBB3hWKp1bDcrWSWZrvJ1mJX9i9CGDygUZPY5tt9oHY6YhT45f6iuR
TX Length: 679
Buffer tx signature: 5YLnJxPpLk2PRjr32Kj7NKjBsdztELtpC8in4kANbhckQeF3WCpwmyX4b2SrVdWptemosdgQE9zGPqB1Ucdu6tYJ
TX Length: 679
Buffer tx signature: RoztKr6Yz8DMEFfpwQK8BCLxmLmjsXK7xKFfM2oppZX65GVtW5iVG1y65vS2SSK239VQRqVzD4spERC9w1kV13A
TX Length: 495
Buffer tx signature: 3Mchwz81Kz8guRUTi6S5bxmACtG34Ti3uFHnJar1poHaLdancAgtGKh2ZqD7cj5vuM8ckwD5p6G4xE39CVN9ng78
Create tx signature: 2hMrYZ7HBAcvKSvENuRG3DJfByFjacBYfHihPrZvrTVKr7sy1xDkJcPbU7kPHgQMPZeZS76YMxaQE7a5UB7yCDs7
```
