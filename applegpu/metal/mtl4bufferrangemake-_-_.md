# MTL4BufferRangeMake(_:_:)

*Function · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtl4bufferrangemake(_:_:)>

## Declaration

```swift
func MTL4BufferRangeMake(_ bufferAddress: MTLGPUAddress, _ length: UInt64) -> MTL4BufferRange
```

## Discussion

Create a buffer range from a buffer’s GPU address (given by the gpuAddress property) and length. A length of (uint64_t)-1 represents the the range from the given address to the end of the buffer.

## See also

### Supporting types
- [MTLAxisAlignedBoundingBox](https://developer.apple.com/documentation/metal/mtlaxisalignedboundingbox-swift.typealias) — The bounds for an axis-aligned bounding box.
- [MTLPackedFloat3](https://developer.apple.com/documentation/metal/mtlpackedfloat3-swift.typealias) — }
- [MTLPackedFloat4x3](https://developer.apple.com/documentation/metal/mtlpackedfloat4x3-swift.typealias) — A structure that contains the top three rows of a 4x4 matrix of 32-bit floating-point values, in column-major order.
- [MTLPackedFloat3Make(_:_:_:)](https://developer.apple.com/documentation/metal/mtlpackedfloat3make(_:_:_:)) — Returns a new packed vector with three floating-point values.
- [MTL4BufferRange](https://developer.apple.com/documentation/metal/mtl4bufferrange)
