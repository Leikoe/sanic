# MTLColorWriteMask

*Structure · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcolorwritemask>

Values used to specify a mask to permit or restrict writing to color channels of a color value.

## Declaration

```swift
struct MTLColorWriteMask
```

## Overview

The values [red](https://developer.apple.com/documentation/metal/mtlcolorwritemask/red), [green](https://developer.apple.com/documentation/metal/mtlcolorwritemask/green), [blue](https://developer.apple.com/documentation/metal/mtlcolorwritemask/blue), and [alpha](https://developer.apple.com/documentation/metal/mtlcolorwritemask/alpha) select one color channel each, and they can be bitwise combined.

## Topics

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlcolorwritemask/init(rawvalue:)) — Returns a new color write mask from a specified raw value.

### Type Properties
- [all](https://developer.apple.com/documentation/metal/mtlcolorwritemask/all) — All color channels are enabled.
- [alpha](https://developer.apple.com/documentation/metal/mtlcolorwritemask/alpha) — The alpha color channel is enabled.
- [blue](https://developer.apple.com/documentation/metal/mtlcolorwritemask/blue) — The blue color channel is enabled.
- [green](https://developer.apple.com/documentation/metal/mtlcolorwritemask/green) — The green color channel is enabled.
- [red](https://developer.apple.com/documentation/metal/mtlcolorwritemask/red) — The red color channel is enabled.
- [unspecialized](https://developer.apple.com/documentation/metal/mtlcolorwritemask/unspecialized) — Defers assigning the color write mask.

## See also

### Configuring render pipeline states
- [pixelFormat](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/pixelformat) — The pixel format of the color attachment’s texture.
- [writeMask](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/writemask) — A bitmask that restricts which color channels are written into the texture.
