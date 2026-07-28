# Positioning samples programmatically

*Article*

<https://developer.apple.com/documentation/metal/positioning-samples-programmatically>

Configure the position of samples when rendering to a multisampled render target.

## Overview

When you perform a render pass that uses multisample antialiasing (MSAA) operations, the GPU samples and resolves subpixels using a specific visual pattern. On GPUs that support programmable sample positions, you can change this pattern. Programmable sample positions unlock additional rendering techniques because you can configure them into custom patterns that you reuse or reposition in each render pass.

### Verify support for programmable sample positions

Not all GPUs support programmable sample positions. Check for support by reading the [areProgrammableSamplePositionsSupported](https://developer.apple.com/documentation/metal/mtldevice/areprogrammablesamplepositionssupported) property on a device instance. If this property’s value is [false](https://developer.apple.com/documentation/Swift/false), the device instance uses fixed sample positions that you can’t query or modify.

Additionally, the number of sample positions that the device instance supports may vary. Call the [supportsTextureSampleCount(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportstexturesamplecount(_:)) method to determine if a given number of samples is usable on that device instance.

### Get the default sample positions

Programmable sample positions are set on a 4-bit subpixel grid (16 x 16 subpixels). Floating-point values are in the `[0.0,1.0)` range along each axis, with the origin `(0,0)` defined at the top-left corner.  You can set values from `0/16` up to `15/16`, inclusive, in `1/16` increments along each axis.

![image](https://docs-assets.developer.apple.com/published/7db887d0d8133b2a601258d3215ec9e4/positioning-samples-programmatically-1%402x.png)

Metal uses the same default sample positions on all GPUs that support programmable sample positions. Get the default sample positions for a given sample count by calling the [getDefaultSamplePositions:count:](https://developer.apple.com/documentation/metal/mtldevice/getdefaultsamplepositions:count:) method, as shown in the code below. Programmable sample positions are defined as an array of [MTLSamplePosition](https://developer.apple.com/documentation/metal/mtlsampleposition) values.

```swift
MTLSamplePosition samplePositions[4];
[_device getDefaultSamplePositions:samplePositions count:4];
```

For example, the following table and grid show the position index, values, and placement for the default one-sample position. The complete set of default sample positions is described in [getDefaultSamplePositions:count:](https://developer.apple.com/documentation/metal/mtldevice/getdefaultsamplepositions:count:).

| Position index | Position values |
|---|---|
| 0 | 0.5, 0.5 |

![image](https://docs-assets.developer.apple.com/published/b82e8ada6eb98fc644df94e8d27c8eb1/positioning-samples-programmatically-2%402x.png)

### Set the sample positions in a render pass

To change the sample positions in a render pass, call the [setSamplePositions:count:](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/setsamplepositions:count:) method of an [MTLRenderPassDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor), as shown below, passing in the array of sample positions you want to use.

```objective-c
static const MTLSamplePosition samplePositions[4] = {
    0.25, 0.25,
    0.75, 0.25,
    0.75, 0.75,
    0.25, 0.75,
};
[renderPassDescriptor setSamplePositions:samplePositions count:4];
```

The following grid shows the programmable sample positions in the `samplePositions` array:

![image](https://docs-assets.developer.apple.com/published/880c249cfff2fe84866e541e71a8dc90/positioning-samples-programmatically-3%402x.png)

## See also

### Advanced multisampling
- [Storing data a pass makes with custom sample positions for a subsequent pass](https://developer.apple.com/documentation/metal/storing-data-a-pass-makes-with-custom-sample-positions-for-a-subsequent-pass) — Inform Metal when your app uses programmable sample positions for its depth render targets or copies MSAA depth data.
