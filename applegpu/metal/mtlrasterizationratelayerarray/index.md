# MTLRasterizationRateLayerArray

*Class · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratelayerarray>

Descriptions for the rasterization rates to apply to the set of layers in a rate map.

## Declaration

```swift
class MTLRasterizationRateLayerArray
```

## Topics

### Accessing members of the array
- [subscript(_:)](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerarray/subscript(_:)) — Retrieves the sample value at the specified index.
- [MTLRasterizationRateLayerDescriptor](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor) — The minimum rasterization rates to apply to sections of a layer in the render target.

## See also

### Configuring the rate map layers
- [layerCount](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/layercount) — The number of layers in the rate map.
- [layer(at:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/layer(at:)) — Returns the layer description for a layer in the rate map.
- [setLayer(_:at:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/setlayer(_:at:)) — Sets a configuration for a layer rate map.
- [layers](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/layers) — The rasterization rates for one or more layers in the rate map.
