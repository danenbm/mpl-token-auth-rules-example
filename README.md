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
    owner: 7gnbaVFCHyWv6q9dBkMAcdrGYRiMppGRs3UG983bQfMX,
    operations: {
        "Transfer": All {
            rules: [
                AdditionalSigner {
                    account: 8eNacFuFdSRYH5SvbHD2dTveWZUYFf2brWoGqqudxcr,
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
Create tx signature: wfRh1oidBBe6n5GBdTf5wVWAC9eeXLBPuTZNGhxkyZYpBTo126CmPZkWMKP8JdZ2K71ZjpMcjQv5taJKoHgJf5t
Validate tx signature: 33xNEhn48arkEAH1Si8aJsMRbRFofEws9zy111DXcnxNFoh2jRDhHP3u2jz9YbAqgL6kMs9afkWUmyY4ibz92Heb
```
