# Artifact post action: automatically assign a contributor

This Function automatically assigns the creator of an artifact as the contributor/assignee.
The contributor/assignee is only set when the artifact is created if no other contributors
have been designed.

The code expects the tracker to have the [contributor/assignee semantic](https://docs.tuleap.org/user-guide/trackers/administration/configuration/semantics.html#contributor-assignee) set.

## How to test

First you need to run `nix-shell` to have all needed tools.

Then build the Tuleap Function with:
```shell
make prepare
make build
```

You can then run it with:

```shell
wasmtime run ./dist/function.wasm < sample.json
```

You can run the typechecking and unit tests with:

```shell
make tests
```

## How to use

Create a tracker and define the [contributor/assignee semantic](https://docs.tuleap.org/user-guide/trackers/administration/configuration/semantics.html#contributor-assignee).

Then build the function:

```shell
make prepare
make build
```

Then upload the binary result file (`dist/function.wasm`) to your Tracker
administration (Administration > Workflow > Tuleap Functions).

And finally, create an artifact, shortly after the creation, the contributor field will automatically be set to yourself.