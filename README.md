k4a-sys
-------
A bingden wrapper for the Azure Kinect Sensor SDK, which should work on both Linux and Windows.

Vendors the compiled SDK libs from version 1.3.

On linux you need to install `libk4a1.3` to be able to use `libk4a.so` and `libdepthengine.so`.

On windows you need to install the Sensor SDK to able to have the `libdepthengine.dll`.

One future wish is to statically compile the SDK into this library, which would require modifying
the SDK's build to output a static library we could link against.

It also might be possible to compile libk4a from source in the future,
but it looks time consuming.

Examples
------
There are two examples that match the SDK's. You can run them with eg:
```
cargo run --example enumerate
cargo run --example streaming
```


Future plans
------
I've started also planning on writing a safe Rust wrapper library, `k4a-rs` to allow Rust-idiomatic (not raw pointers!)
access to the libraries. It will probably live in this repo and we'll move `k4a-sys` to a sub-directory here, the
same way [`bzip2-rs` and `bzip2-sys`](https://github.com/alexcrichton/bzip2-rs) work.

I think in the near future we'll remove bindgen from the list of dependencies and only run it every library update,
that is, by hand/script instead of in the build.rs. This is because bindgen has ~60 dependencies, and should always
generate the same output file anyways. (It's possible to make the output cross platform).
