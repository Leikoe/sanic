# Rendering reflections in real time using ray tracing

*Sample Code · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, Xcode 26.3*

<https://developer.apple.com/documentation/metal/rendering-reflections-in-real-time-using-ray-tracing>

Implement realistic real-time lighting by dynamically generating reflection maps by encoding a ray-tracing compute pass.

## Overview

This sample code project relates to multiple WWDC sessions, including:

- [10089: Bring your advanced games to Apple platforms](https://developer.apple.com/wwdc24/10089/)

- [10101: Go bindless with Metal 3](https://developer.apple.com/wwdc22/10101/)

- [10286: Explore bindless rendering in Metal](https://developer.apple.com/wwdc21/10286/)

- [10150: Explore hybrid rendering with Metal ray tracing](https://developer.apple.com/wwdc21/10150/)

### Configure the sample code project

To run this sample app, you need the following:

- A Mac with macOS 13 or later, and Xcode 15.3 or later

- An iOS device with iOS 16 or later

> **Note:**
> This sample doesn’t support running in Simulator.

## See also

### Ray tracing
- [Accelerating ray tracing using Metal](https://developer.apple.com/documentation/metal/accelerating-ray-tracing-using-metal) — Implement ray-traced rendering using GPU-based parallel processing.
- [Control the ray tracing process using intersection queries](https://developer.apple.com/documentation/metal/control-the-ray-tracing-process-using-intersection-queries) — Explicitly enumerate a ray’s intersections with acceleration structures by creating an intersection query object.
- [Accelerating ray tracing and motion blur using Metal](https://developer.apple.com/documentation/metal/accelerating-ray-tracing-and-motion-blur-using-metal) — Generate ray-traced images with motion blur using GPU-based parallel processing.
- [Rendering a curve primitive in a ray tracing scene](https://developer.apple.com/documentation/metal/rendering-a-curve-primitive-in-a-ray-tracing-scene) — Implement ray traced rendering using GPU-based parallel processing.
