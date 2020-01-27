k4a-sys
-------
A Bingden Wrapper for the Azure Kinect Sensor SDK.

Builds on both Linux and Windows. Vendors the SDK at version 1.3.

At the moment I have compiled the sdk on both linux and windows
and added the `include`s and dynamic libraries by hand to the 
`vendor` folder. This works for now.

It might be possible to compile libk4a from source in the future,
but it looks time consuming at the moment.
