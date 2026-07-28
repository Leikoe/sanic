# getDefaultSamplePositions(sampleCount:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 11.0, macOS 10.13, tvOS 11.0, visionOS*

<https://developer.apple.com/documentation/metal/mtldevice/getdefaultsamplepositions(samplecount:)>

Returns the default sample locations based on the number of samples.

## Declaration

```swift
func getDefaultSamplePositions(sampleCount: Int) -> [MTLSamplePosition]
```

## Parameters

- **sampleCount** — The number of points a GPU can sample from a texture. Ensure the GPU can support the `sampleCount` value by first calling the device’s [supportsTextureSampleCount(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportstexturesamplecount(_:)) method.

## Return Value

An array of [MTLSamplePosition](https://developer.apple.com/documentation/metal/mtlsampleposition) instances.

## Discussion

The default sample positions are the same on all GPUs that support programmable sample positions (see [areProgrammableSamplePositionsSupported](https://developer.apple.com/documentation/metal/mtldevice/areprogrammablesamplepositionssupported)).

> **Note:**
>  GPUs that don’t support programmable sample positions may have different default sample positions that you can’t retrieve.

The default sample position for GPUs that can sample one time is at the pixel’s center.

![image](https://docs-assets.developer.apple.com/published/b82e8ada6eb98fc644df94e8d27c8eb1/positioning-samples-programmatically-2%402x.png)

The default sample positions for GPUs that can sample two times have locations in the center of the pixel’s second quadrant and fourth quadrants.

![image](https://docs-assets.developer.apple.com/published/0326b2b19119cd5568f173d4087b10c4/getDefaultSamplePositions-2%402x.png)

The default sample positions for GPUs that can sample four times have one location in each of the pixel’s quadrants. Each location is at the center of one of that quadrant’s subquadrants.

![image](https://docs-assets.developer.apple.com/published/75241ff22d0b2ee42446c2860eca985d/getDefaultSamplePositions-3%402x.png)

The default sample positions for GPUs that can sample eight times have two locations in each of the pixel’s quadrants.

![image](https://docs-assets.developer.apple.com/published/b0987ba0ce96b5853f689931e9496f22/getDefaultSamplePositions-4%402x.png)

The table lists the indices and default locations for GPUs that support 1, 2, 4, or 8 sample positions.

| Sample count | Position indices | Subpixel coordinates |
|---|---|---|
| 1 | 0 | (0.5, 0.5) |
| 2 | 0 ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) 1 | (0.75, 0.75) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) (0.25, 0.25) |
| 4 | 0 ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) 1 ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) 2 ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) 3 | (0.375, 0.125) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) (0.875, 0.375) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) (0.125, 0.625) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) (0.625, 0.875) |
| 8 | 0 ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) 1 ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) 2 ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) 3 ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) 4 ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) 5 ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) 6 ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) 7 | (0.5625, 0.3125) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) (0.4375, 0.6875) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) (0.8125, 0.5625) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) (0.3125, 0.1875) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) (0.1875, 0.8125) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) (0.0625, 0.4375) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) (0.6875, 0.9375) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) (0.9375, 0.0625) |

## See also

### Creating samplers
- [supportsTextureSampleCount(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportstexturesamplecount(_:)) — Returns a Boolean value that indicates whether the GPU can sample a texture with a specific number of sample points.
- [makeSamplerState(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makesamplerstate(descriptor:)) — Creates a sampler state instance.
