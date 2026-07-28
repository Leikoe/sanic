# subscript(_:)

*Instance Subscript · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratelayerarray/subscript(_:)>

Retrieves the sample value at the specified index.

## Declaration

```swift
subscript(layerIndex: Int) -> MTLRasterizationRateLayerDescriptor? { get set }
```

## Parameters

- **layerIndex** — The index of the sample you want to retrieve.

## Return Value

An [NSNumber](https://developer.apple.com/documentation/Foundation/NSNumber) instance describing the value of the sample at the specified index, or `0` if the index is out of range.

## See also

### Accessing members of the array
- [MTLRasterizationRateLayerDescriptor](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor) — The minimum rasterization rates to apply to sections of a layer in the render target.
