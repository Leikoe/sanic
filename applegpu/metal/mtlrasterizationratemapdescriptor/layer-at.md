# layer(at:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/layer(at:)>

Returns the layer description for a layer in the rate map.

## Declaration

```swift
func layer(at layerIndex: Int) -> MTLRasterizationRateLayerDescriptor?
```

## Parameters

- **layerIndex** — The entry to return.

## Return Value

The [MTLRasterizationRateLayerDescriptor](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor) instance for the given index, or `nil` if you haven’t set an instance for this index.

## Discussion

Calling this method is equivalent to using array subscript syntax.

## See also

### Configuring the rate map layers
- [layerCount](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/layercount) — The number of layers in the rate map.
- [setLayer(_:at:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/setlayer(_:at:)) — Sets a configuration for a layer rate map.
- [layers](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/layers) — The rasterization rates for one or more layers in the rate map.
- [MTLRasterizationRateLayerArray](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerarray) — Descriptions for the rasterization rates to apply to the set of layers in a rate map.
