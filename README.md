k4a-sys
-------
A Bingden Wrapper for the Azure Kinect Sensor SDK, which works on both Linux and Windows.

Uses a vendored version of the precompiled SDK at version 1.3.

At the moment I have compiled the sdk on both linux and windows
and added the `include`s and dynamic libraries by hand to the 
`vendor` folder. This works for now.

It might be possible to compile libk4a from source in the future,
but it looks time consuming at the moment.

This is temporarily missing the `depthengine` library, my plan is to statically compile this into the lib soon.

Raise an issue if you'd like dynamic linking, but since k4a is such
a niche library, I think it's very convenient to have it statically compiled
into the lib for now.
