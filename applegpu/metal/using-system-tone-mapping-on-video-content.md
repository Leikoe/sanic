# Using system tone mapping on video content

*Article*

<https://developer.apple.com/documentation/metal/using-system-tone-mapping-on-video-content>

Use EDR metadata to apply the default system tone mapping to a layer.

## Overview

When processing video content, you usually want to work in a linear color space. You also want to display content that’s consistent with how playback would appear in AVFoundation or other playback mechanisms. To create content in your app that is consistent with the system behavior, create a Metal layer with a linear color space and attach an [CAEDRMetadata](https://developer.apple.com/documentation/QuartzCore/CAEDRMetadata) object defining how the system should tone map the video content.

The code below creates a Metal layer with an extended linear BT.2020 color space and metadata that applies an HDR10 tone mapping based on the reference display.

```swift
let metalLayer = CAMetalLayer()
metalLayer.wantsExtendedDynamicRangeContent = true
metalLayer.pixelFormat = .rgba16Float

let name = CGColorSpace.extendedLinearITUR_2020
metalLayer.colorspace = CGColorSpace(name: name)

let edrMetadata = CAEDRMetadata(minLuminance: 0.5, maxLuminance: 1000, opticalOutputScale: 100)
metalLayer.edrMetadata = edrMetadata
```

```objective-c
CAMetalLayer *metalLayer = [CAMetalLayer new];
metalLayer.wantsExtendedDynamicRangeContent = YES;
metalLayer.pixelFormat = MTLPixelFormatRGBA16Float;

const CFStringRef name = kCGColorSpaceExtendedLinearITUR_2020;
CGColorSpaceRef colorspace = CGColorSpaceCreateWithName(name);
metalLayer.colorspace = colorspace;

CGColorSpaceRelease(colorspace);
CAEDRMetadata *edrMetaData = [CAEDRMetadata HDR10MetadataWithMinLuminance: 0.005 maxLuminance: 1000 opticalOutputScale: 100];
metalLayer.EDRMetadata = edrMetaData;
```

Your rendering code needs to generate pixel values consistent with the EDR metadata object. For example, in the above code, the `opticalOutputScale` was set to `100`, so a pixel value of `1.0` corresponds to `100` nits. For more information, see [CAEDRMetadata](https://developer.apple.com/documentation/QuartzCore/CAEDRMetadata).

## See also

### High dynamic range content
- [Processing HDR images with Metal](https://developer.apple.com/documentation/metal/processing-hdr-images-with-metal) — Implement a post-processing pipeline using the latest features on Apple GPUs.
- [Displaying HDR content in a Metal layer](https://developer.apple.com/documentation/metal/displaying-hdr-content-in-a-metal-layer) — Bring your high dynamic range (HDR) content to compatible Mac displays.
- [Determining support for EDR values](https://developer.apple.com/documentation/metal/determining-support-for-edr-values) — Check whether a display supports EDR.
- [Using color spaces to display HDR content](https://developer.apple.com/documentation/metal/using-color-spaces-to-display-hdr-content) — Use a color space when you don’t need to edit or process the pixel data.
- [Performing your own tone mapping](https://developer.apple.com/documentation/metal/performing-your-own-tone-mapping) — Apply your own tone mapping to get the exact behavior you want.
- [Implementing tone mapping on reference displays](https://developer.apple.com/documentation/metal/implementing-tone-mapping-on-reference-displays) — Detect reference displays and keep your content within the capabilities of the display hardware.
