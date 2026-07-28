# makeRasterizationRateMap(descriptor:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makerasterizationratemap(descriptor:)>

Creates a rasterization rate map instance.

## Declaration

```swift
func makeRasterizationRateMap(descriptor: MTLRasterizationRateMapDescriptor) -> (any MTLRasterizationRateMap)?
```

## Parameters

- **descriptor** — An [MTLRasterizationRateMapDescriptor](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor) instance.

## Return Value

A new [MTLRasterizationRateMapDescriptor](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor) instance if the method completes successfully; otherwise `nil`.

## See also

### Creating rasterization rate maps
- [supportsRasterizationRateMap(layerCount:)](https://developer.apple.com/documentation/metal/mtldevice/supportsrasterizationratemap(layercount:)) — Returns a Boolean value that indicates whether the GPU can create a rasterization rate map with a specific number of layers.
