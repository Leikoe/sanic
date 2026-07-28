# Rendering a scene with forward plus lighting using tile shaders

*Sample Code · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, Xcode 26.3*

<https://developer.apple.com/documentation/metal/rendering-a-scene-with-forward-plus-lighting-using-tile-shaders>

Implement a forward plus renderer using the latest features on Apple GPUs.

## Overview

> **Note:**
> This sample code project is associated with WWDC 2019 session [601: Modern Rendering with Metal](https://developer.apple.com/videos/play/wwdc19/601/).

### Configure the sample code project

To run the app:

- Build the project with Xcode 11 or later.

- Target an iOS device with an A11 chip or later and iOS 11 or later.

## See also

### Lighting techniques
- [Rendering a scene with deferred lighting in Objective-C](https://developer.apple.com/documentation/metal/rendering-a-scene-with-deferred-lighting-in-objective-c) — Avoid expensive lighting calculations by implementing a deferred lighting renderer optimized for immediate mode and tile-based deferred renderer GPUs.
- [Rendering a scene with deferred lighting in Swift](https://developer.apple.com/documentation/metal/rendering-a-scene-with-deferred-lighting-in-swift) — Avoid expensive lighting calculations by implementing a deferred lighting renderer optimized for immediate mode and tile-based deferred renderer GPUs.
- [Rendering a scene with deferred lighting in C++](https://developer.apple.com/documentation/metal/rendering-a-scene-with-deferred-lighting-in-c++) — Avoid expensive lighting calculations by implementing a deferred lighting renderer optimized for immediate mode and tile-based deferred renderer GPUs.
- [Rendering reflections with fewer render passes](https://developer.apple.com/documentation/metal/rendering-reflections-with-fewer-render-passes) — Use layer selection to reduce the number of render passes needed to generate an environment map.
