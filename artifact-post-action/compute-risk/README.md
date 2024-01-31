# Artifact post action: compute risk

This Function will compute a Risk based on a Severity (select box with values from 1 to 4) and 
a Probability (select box with values from 1 to 4). The result is Severity x Probability and is
selected in a select box list from 1 to 16.

The code relies on the tracker structure to pick-up the values according to the labels in the
different select box.

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

## How to use

You can create a tracker with the [XML template](./Tracker_action.xml) provided as example. You can also
use the module as is, in any Tracker with following constraints:
- A select box field with "Severity" as label
  - The select box has 4 static values "1", "2", "3", "4" (label must be exact "1 - low" will not work for instance). But you can set fancy colors.
- A select box field with "Probability" as label
  - The select box has 4 static values "1", "2", "3", "4"
- A select box field with "Risk" as label
  - The select box has 16 static values "1", "2", ..., "15", "16"
  - You can set the field read only so nobody but the Tuleap Function will be able to change the value

Then build the function:

```shell
cargo build --target wasm32-wasi --release
```

Then upload the binary result file (`target/wasm32-wasi/release/post-action-compute-risk.wasm`) to your Tracker
administration (Administration > Workflow > Tuleap Functions).

And finally, update an artifact, shortly after the creation, it will be automatically updated with computed risk.