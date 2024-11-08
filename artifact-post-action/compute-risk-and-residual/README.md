# Artifact post action: compute risk & residual risk

This Function will compute a `Risk` based on a `Severity` (select box with values from 1 to 4) and 
a `Probability` (select box with values from 1 to 4). The result is Severity x Probability and is
selected in a select box list from 1 to 16. The Function also compute `Residual risk` with the
same algorithm on `Residual probability` and `Residual severity` fields

This sample introduce usage of Rust data structures instead of raw parsing of json.

## How to test

First you need to run `nix-shell` to have all needed tools.

Then you can test the module with:

```shell
cat sample.json | cargo run
```

And ouput a json representing new modifications. The structure must be identical to the one for REST
API `PUT /api/artifacts/:id`. For example:

```json
{"values":[{"bind_value_ids":[15050],"field_id":25307}]}
```

* `field_id` corrresponds to the id of the field with "Risk" label
* `bind_values_ids` is an array with only one value, the list value id that corresponds to the result of
  the computation.

## How to release

Then build the function:

```shell
cargo build --target wasm32-wasi --release
```
