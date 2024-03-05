# Example 

The following is example of dynamic dispatch in the component model using the `dyna` crate.

There are three parts:
* The host
* The static guest
* The dynamic guest

The host instantiates the dynamic guest which loads the static guest and calls its export. The host knows nothing of the dynamic guest and in fact, the dynamic guest only knows about the dynamic guest at runtime. 

## Running

First, from the root of the dyna project, build the static guest:

```
cargo component build -p static-guest
```

Then move the static guest Wasm binary into the `example` directory.

Next, build the dynamic guest:

```
cargo component build -p dynamic-guest
```

Finally, run the dynamic guest from the host:

```
cargo run -p host -- $PATH_TO_DYNAMIC_GUEST_BINARY
```