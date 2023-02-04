# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/mpl-token-auth-rules-example`
Metaplex Royalty RuleSet Dev: FHF4ydp2QbDT5uYzmzKcerKWW6Eyqtj1riD8hFrj6yPE
RuleSetV1 {
    lib_version: 1,
    owner: 33tzvRFNBzwt9fkyRc14zPWXq8iHCd86y7HgEZQd8YCm,
    rule_set_name: "Metaplex Royalty RuleSet Dev",
    operations: {
        "Transfer:MigrationDelegate": All {
            rules: [
                Amount {
                    amount: 1,
                    operator: Eq,
                    field: "Amount",
                },
                Any {
                    rules: [
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Source",
                        },
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Destination",
                        },
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Authority",
                        },
                    ],
                },
            ],
        },
        "Delegate:Use": All {
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
                    field: "Delegate",
                },
            ],
        },
        "Delegate:Sale": All {
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
                    field: "Delegate",
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
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Source",
                        },
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Destination",
                        },
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Authority",
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
        "Delegate:Transfer": All {
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
                    field: "Delegate",
                },
            ],
        },
        "Delegate:Staking": All {
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
                    field: "Delegate",
                },
            ],
        },
        "Delegate:Utility": All {
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
                    field: "Delegate",
                },
            ],
        },
        "Delegate:LockedTransfer": All {
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
                    field: "Delegate",
                },
            ],
        },
        "Delegate:Update": All {
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
                    field: "Delegate",
                },
            ],
        },
        "Delegate:Authority": All {
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
                    field: "Delegate",
                },
            ],
        },
        "Transfer:TransferDelegate": All {
            rules: [
                Amount {
                    amount: 1,
                    operator: Eq,
                    field: "Amount",
                },
                Any {
                    rules: [
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Source",
                        },
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Destination",
                        },
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Authority",
                        },
                    ],
                },
            ],
        },
        "Delegate:Collection": All {
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
                    field: "Delegate",
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
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Source",
                        },
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Destination",
                        },
                        ProgramOwnedList {
                            programs: [
                                metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s,
                                Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz,
                            ],
                            field: "Authority",
                        },
                    ],
                },
            ],
        },
    },
}
TX Length: 679
Buffer tx signature: 4JLzswP5i4HZFLDzjjkRiLsJfVVX2jR6exFeHuJx8RtRLbKfmqMsH2E24wWHLDUghzup3dqUHa1xfYCwrzUjjriF
TX Length: 679
Buffer tx signature: 5bdMt4Wb9B79cmpYkifzCby2MKhptVVigMiCbQvzCz3scFPLTQHhZEk5y7AAksvCLfLUKxF98jgjdtUqyYuxXqq4
TX Length: 679
Buffer tx signature: 3617kdTgfmJEaujKCxVJsCGJKwYYza9M7ZdhP9T6gmmK72Z3F2i4TPfVdYy4BVBC72Wzb86v4ZFCP48NRQ7oVQA6
TX Length: 679
Buffer tx signature: 3MG7B7BHaZa8F66ButsnbqJbA5rdmrV1beWG7UN9ZnqPhNiP2Dx5fitKPF7SnafcMviEDwL1hcnLDdMXv4G6ZFU3
TX Length: 679
Buffer tx signature: 5dpNxtYFYjBTTMfqjnJi9yZsoMfSUL8FCpKF3ZimRRzyd4uwH8yaw4EZXGjNE2PnDRdQB7YFLtmYJQhsBN4JDkNf
TX Length: 679
Buffer tx signature: 2VB5CQiuEsWbYLXqhS3PxHGPpAcBCnqHKfpn43QtdTKqupDuzfCtMmvDfKxvwDAyUiiJ76nF1gUGtEe4YYh6pdES
TX Length: 661
Buffer tx signature: 3RQm28pRKN2dhcsNKjbL6voFAb4yRvZH7u6Y1VSEzxfPcujrJv4hTA1qMvgN6ovk9Jqcot6xYHBB5oxg3BcDxc76
Create tx signature: w8SywTi1sqys4m4NPbsEG2guuAm4TyXi1ui3mp4ycVSjKzgZDGpCbjQu7cjZo2pJFmPEFoCaf6GcB8QSZi1ePgn
```
