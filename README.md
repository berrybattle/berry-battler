# berry-battler
Berry Battler is a user defined application that uses AI algorithm(s) executing on a Raspberry Pi device to compete against others in a game simulation.

## Build and Run
The application is configured to be executed on the same machine as the berry-battle-server and publishes it's endpoints to localhost.

To configure the application to run on a different machine, the ```main.rs``` file should be updated with the correct IP address in place of "[::1]".

For the most accurate benchmarking results the binary should be build with the ```release``` flag.
```console
$ cargo build --release
$ cargo run --release
```


