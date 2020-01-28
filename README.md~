k4a-sys
-------
A bingden wrapper for the Azure Kinect Sensor SDK, which works on both Linux and Windows.

Vendors the compiled SDK libs from version 1.3.

At the moment I have compiled the sdk on both Linux and Windows
and added the `include`s and dynamic libraries by hand to the 
`vendor` folder. It might be possible to compile libk4a from source in the future,
but it looks time consuming.

This is temporarily missing the `depthengine` library, my plan is to statically compile this into the lib soon.

Raise an issue if you'd like dynamic linking, but since `k4a` is such
a niche library, I think it's very convenient to have it statically compiled
into the lib for now.

I'm also planning on writing a safe Rust wrapper library, `k4a-rs` to allow Rust-idiomatic (not raw pointers!)
access to the libraries. It will probably live in this repo and we'll move `k4a-sys` to a sub-directory here, the
same way [`bzip2-rs` and `bzip2-sys`](https://github.com/alexcrichton/bzip2-rs) work.

I think in the near future we'll remove bindgen from the list of dependencies and only run it every library update,
that is, by hand/script instead of in the build.rs. This is because bindgen has ~60 dependencies, and should always
generate the same output file anyways. (It's possible to make the output cross platform).

