# setLayer(_:at:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/setlayer(_:at:)>

Sets a configuration for a layer rate map.

## Declaration

```swift
func setLayer(_ layer: MTLRasterizationRateLayerDescriptor?, at layerIndex: Int)
```

## Parameters

- **layer** — A description of a layer to add to the rate map descriptor. Use `nil` to remove the layer at that index.
- **layerIndex** — The index to put the new layer description in.

## Discussion

Calling this method is equivalent to using array subscript syntax.

## See also

### Configuring the rate map layers
- [layerCount](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/layercount) — The number of layers in the rate map.
- [layer(at:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/layer(at:)) — Returns the layer description for a layer in the rate map.
- [layers](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/layers) — The rasterization rates for one or more layers in the rate map.
- [MTLRasterizationRateLayerArray](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerarray) — Descriptions for the rasterization rates to apply to the set of layers in a rate map.
