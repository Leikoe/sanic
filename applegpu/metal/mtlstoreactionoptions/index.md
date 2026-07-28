# MTLStoreActionOptions

*Structure · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstoreactionoptions>

Options that modify a store action.

## Declaration

```swift
struct MTLStoreActionOptions
```

## Overview

This property modifies the intended behavior of the store actions in the [MTLStoreAction](https://developer.apple.com/documentation/metal/mtlstoreaction) enumeration.

## Topics

### Using programmable sample positions
- [customSamplePositions](https://developer.apple.com/documentation/metal/mtlstoreactionoptions/customsamplepositions) — An option that stores data in a sample-position–agnostic representation.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlstoreactionoptions/init(rawvalue:)) — Creates a store action option from a raw integer value.

## See also

### Encoding a render pass in parallel
- [MTLParallelRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder) — An instance that splits up a single render pass so that it can be simultaneously encoded from multiple threads.
- [MTLLoadAction](https://developer.apple.com/documentation/metal/mtlloadaction) — Types of actions performed for an attachment at the start of a rendering pass.
- [MTLStoreAction](https://developer.apple.com/documentation/metal/mtlstoreaction) — Types of actions performed for an attachment at the end of a rendering pass.
