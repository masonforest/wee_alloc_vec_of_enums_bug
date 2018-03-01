This is a [MCVE](https://stackoverflow.com/help/mcve) of bug
[wee_alloc#29](https://github.com/fitzgen/wee_alloc/issues/29)

To run:

    $ cargo rustc --target wasm32-unknown-unknown --lib -- -O && node run.js
    (node:6028) UnhandledPromiseRejectionWarning: RuntimeError: unreachable
    ...
