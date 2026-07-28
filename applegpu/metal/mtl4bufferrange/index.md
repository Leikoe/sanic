# MTL4BufferRange

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtl4bufferrange>

## Declaration

```swift
struct MTL4BufferRange
```

## Overview

A struct representing a range of a Metal buffer. The offset into the buffer is included in the address. The length is generally optional, which a value of (uint64_t)-1 representing the range from the given address to the end of the buffer. However, providing the length can enable more accurate API validation, especially when sub-allocating ranges of a buffer.

## Topics

### Initializers
- [init()](https://developer.apple.com/documentation/metal/mtl4bufferrange/init())
- [init(bufferAddress:length:)](https://developer.apple.com/documentation/metal/mtl4bufferrange/init(bufferaddress:length:))

### Instance Properties
- [bufferAddress](https://developer.apple.com/documentation/metal/mtl4bufferrange/bufferaddress)
- [length](https://developer.apple.com/documentation/metal/mtl4bufferrange/length)

## See also

### Supporting types
- [MTLAxisAlignedBoundingBox](https://developer.apple.com/documentation/metal/mtlaxisalignedboundingbox-swift.typealias) — The bounds for an axis-aligned bounding box.
- [MTLPackedFloat3](https://developer.apple.com/documentation/metal/mtlpackedfloat3-swift.typealias) — }
- [MTLPackedFloat4x3](https://developer.apple.com/documentation/metal/mtlpackedfloat4x3-swift.typealias) — A structure that contains the top three rows of a 4x4 matrix of 32-bit floating-point values, in column-major order.
- [MTLPackedFloat3Make(_:_:_:)](https://developer.apple.com/documentation/metal/mtlpackedfloat3make(_:_:_:)) — Returns a new packed vector with three floating-point values.
- [MTL4BufferRangeMake(_:_:)](https://developer.apple.com/documentation/metal/mtl4bufferrangemake(_:_:))
