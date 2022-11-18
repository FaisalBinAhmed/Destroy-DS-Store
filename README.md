# Destroy DS_Store

Delete the annoying .DS_Store files from the current folder and all its sub-folders.

To execute without installing run `cargo run`. By default, it will scan the root project directory.

You can optionally pass arguments to the app like this: `cargo run -- --help` for more information.

To run it silently, pass the s flag like this `cargo run -- -s`. This will prevent the app from printing the file names as it scans.

To delete all .ds_store files without asking, pass the y flag like this `cargo run -- -y`

The app optionally takes the path of a directory to be scanned as an argument, to specify a path, run `cargo run -- PATH`

After installing, you do not need `cargo run` anymore. Just run `destroy_ds_store PATH [FLAGS]` like `destroy_ds_store --help`

More examples: `destroy_ds_store` scan current directory.

`destroy_ds_store ../` scan parent directory.
