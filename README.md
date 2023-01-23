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
    owner: AQsrrJYz5HcQ5xekq6C4pXHJfsQGveidmtcsw7jXwWTn,
    operations: {
        "OwnerTransfer": All {
            rules: [
                AdditionalSigner {
                    account: 5UJXb5DzKT9FFdCYpLkF3SJAwFfvoowhK73ikxURwrXY,
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
Create tx signature: b42kC74Nr37KtsM2vRRELxcK9GdiFvwYpypqc8QTJCmQdAoXYrSAxnEREpQRbQfV3xggt5UQ1oW9v2exNT4PZ9X
Validate tx signature: 4tn9LMVtXyYaugT4iLDHCBmSrkg4dwaGA83LpkoTyJbF8YbRFksBwdPLuAYQmhgf1vTuGQ85s5zQwfLUSSy2fkhm
```
