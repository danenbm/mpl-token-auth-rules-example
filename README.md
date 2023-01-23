# Token Authorization Rules Example

This is example code for https://github.com/metaplex-foundation/token-authorization-rules.

## Example
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/mpl-token-auth-rules-example`
RuleSetV1 {
    lib_version: 1,
    rule_set_name: "test rule_set",
    owner: Fuj4PfX1vB9hqnnGDtWeLy3BNXHp1K6NjRFviofcmQAh,
    operations: {
        "OwnerTransfer": All {
            rules: [
                AdditionalSigner {
                    account: 6gT6YrEumwVeqB9BJseYSKgo59qDwuXqx4V2HGVqEkbx,
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
Create tx signature: 65aWtC3LaeudZDQthao18ukyJksMTmHeuMEDJ8ipeqo3TaQm43K7wzMfN4VBHEdn65w2Smd2TQCTDDogZfYPSgE
Validate tx signature: 4sojorL6pPKXiiXUtmkDgHR2ev63tQ8asu9JmanUnxnEMWbsWzu2UcV4anEf77SJoYRac66shS2HQ5DyuF48dcCj
```
