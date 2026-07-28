# supportsRasterizationRateMap(layerCount:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/supportsrasterizationratemap(layercount:)>

Returns a Boolean value that indicates whether the GPU can create a rasterization rate map with a specific number of layers.

## Declaration

```swift
func supportsRasterizationRateMap(layerCount: Int) -> Bool
```

## Parameters

- **layerCount** — The number of layers for a rasterization rate map.

## See also

### Creating rasterization rate maps
- [makeRasterizationRateMap(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makerasterizationratemap(descriptor:)) — Creates a rasterization rate map instance.
