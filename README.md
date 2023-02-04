# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/mpl-token-auth-rules-example`
Metaplex Royalty RuleSet Dev: 8cZYUi7TSzSWoSGTnhxJdHtKyNskgRfeZQdtsqJkniMS
RuleSetV1 {
    lib_version: 1,
    owner: GydBYBTA4HbvjgNhSxbbeqsZ88ur8m8DGhQvvsh398NJ,
    rule_set_name: "Metaplex Royalty RuleSet Dev",
    operations: {
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
    },
}
TX Length: 679
Buffer tx signature: 623k31cpyDFz5xiYnciwMwRRueTkJfPHDpKWyDNeXEaPx4W87sMzYsoWhmuij5jAY84EH62fofAzvmFtjcu6fWZt
TX Length: 679
Buffer tx signature: 5FcbgDBU9LHU6aCE4bKmcWFP5MLgmVzswCp27txsVmwTThAuaUpu49Uy4WwpQaSwgxCbCXGQF2sFznVc4XsT7isY
TX Length: 679
Buffer tx signature: 4mQxb1fLHzc6CfVFQKgbb9K1xTZXypisNmPGjM3vuAh3nDzZdq6QYEUrMLNhCGSxqe3vvyLeEQuC3CUprFwd3acX
TX Length: 679
Buffer tx signature: U6cCMtXE6e7aGLVCsEpQfa4HeZiT99Vjd9srPCT6wkVJ18RLjAD5kd5YcZHXAYtNj5o5JRiqTKswFoXzWG2bj3q
TX Length: 679
Buffer tx signature: 2Pou3HwpSGhgFBXFtjppXHJJXMJRanBiLBvNQFY6X48YPhkbRaKdv8SRLCny5AhDLvNjtyqRU9kxyDtEcFtnU1mo
TX Length: 679
Buffer tx signature: 3xCZQm1rXQG2hycfc8WUQSdv2XC6yoSjeePRTiCfdk9R5Q22HAyHeEPe1ABMk6NXUrwS7JPGNGmi6eAS1FMDsom7
TX Length: 656
Buffer tx signature: 3X2rde2fDQ8AGLCrZyrPatWys7QuGHsksZFovWeZcjAJkPGPRHhFeNHtwF6mUhbYYGmrfEuus8tjzgcRKb6F1yWN
Create tx signature: 4jBnNkUtDvoynqn5bt22pxY464kQQxf9NVZRuUqBFvZESBkHweYCuDTq3CQX3S4cW3DmjM6amm2MNe1dKq4FaydW
```
