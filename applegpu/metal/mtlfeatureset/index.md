# MTLFeatureSet

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfeatureset>

The device feature sets that define specific platform, hardware, and software configurations.

## Declaration

```swift
enum MTLFeatureSet
```

## Overview

If your app is running on an operating system that supports the [supportsFamily(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsfamily(_:)) method, use that method instead. See [Detecting GPU features and Metal software versions](https://developer.apple.com/documentation/metal/detecting-gpu-features-and-metal-software-versions) for more information about [MTLGPUFamily](https://developer.apple.com/documentation/metal/mtlgpufamily) — the replacement for this enumeration —  and the feature set tables. This type doesn’t define constants for GPU families introduced after iOS GPU family 5.

Metal feature sets define the feature availability, implementation limits, and pixel format capabilities for each device. The table shows the GPU families and their corresponding GPU hardware.

| GPU family | GPU hardware |
|---|---|
| iOS GPU family 1 | Apple A7 devices |
| iOS GPU family 2 ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) tvOS GPU family 1 | Apple A8 devices |
| iOS GPU family 3 ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) tvOS GPU family 2 | Apple A9 devices ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) Apple A10 devices |
| iOS GPU family 4 | Apple A11 devices |
| iOS GPU family 5 | Apple A12 devices |
| macOS GPU family 1 | iMac Pro models ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) iMac models from 2012 or later ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) MacBook models from 2015 or later ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) MacBook Pro models from 2012 or later ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) MacBook Air models from 2012 or later ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) Mac mini models from 2012 or later ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) Mac Pro models from late 2013 |
| macOS GPU family 2 | iMac models from 2015 or later ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) MacBook Pro models from 2016 or later ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) MacBook models from 2016 or later ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) iMac Pro models from 2017 or later |

For more information on Mac support for Metal, see [Mac computers that support Metal](https://support.apple.com/en-us/HT205073).

## Topics

### iOS GPU family 5
- [MTLFeatureSet.iOS_GPUFamily5_v1](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily5_v1) — The GPU family 5, version 1 feature set for iOS.

### iOS GPU family 4
- [MTLFeatureSet.iOS_GPUFamily4_v2](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily4_v2) — The GPU family 4, version 2 feature set for iOS.
- [MTLFeatureSet.iOS_GPUFamily4_v1](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily4_v1) — The GPU family 4, version 1 feature set for iOS.

### iOS GPU family 3
- [MTLFeatureSet.iOS_GPUFamily3_v4](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily3_v4) — The GPU family 3, version 4 feature set for iOS.
- [MTLFeatureSet.iOS_GPUFamily3_v3](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily3_v3) — The GPU family 3, version 3 feature set for iOS.
- [MTLFeatureSet.iOS_GPUFamily3_v2](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily3_v2) — The GPU family 3, version 2 feature set for iOS.
- [MTLFeatureSet.iOS_GPUFamily3_v1](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily3_v1) — The GPU family 3, version 1 feature set for iOS.

### iOS GPU family 2
- [MTLFeatureSet.iOS_GPUFamily2_v5](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily2_v5) — The GPU family 2, version 5 feature set for iOS.
- [MTLFeatureSet.iOS_GPUFamily2_v4](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily2_v4) — The GPU family 2, version 4 feature set for iOS.
- [MTLFeatureSet.iOS_GPUFamily2_v3](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily2_v3) — The GPU family 2, version 3 feature set for iOS.
- [MTLFeatureSet.iOS_GPUFamily2_v2](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily2_v2) — The GPU family 2, version 2 feature set for iOS.
- [MTLFeatureSet.iOS_GPUFamily2_v1](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily2_v1) — The GPU family 2, version 1 feature set for iOS.

### iOS GPU family 1
- [MTLFeatureSet.iOS_GPUFamily1_v5](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily1_v5) — The GPU family 1, version 5 feature set for iOS.
- [MTLFeatureSet.iOS_GPUFamily1_v4](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily1_v4) — The GPU family 1, version 4 feature set for iOS.
- [MTLFeatureSet.iOS_GPUFamily1_v3](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily1_v3) — The GPU family 1, version 3 feature set for iOS.
- [MTLFeatureSet.iOS_GPUFamily1_v2](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily1_v2) — The GPU family 1, version 2 feature set for iOS.
- [MTLFeatureSet.iOS_GPUFamily1_v1](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily1_v1) — The GPU family 1, version 1 feature set for iOS.

### tvOS GPU family 2
- [MTLFeatureSet.tvOS_GPUFamily2_v2](https://developer.apple.com/documentation/metal/mtlfeatureset/tvos_gpufamily2_v2) — The GPU family 2, version 2 feature set for tvOS.
- [MTLFeatureSet.tvOS_GPUFamily2_v1](https://developer.apple.com/documentation/metal/mtlfeatureset/tvos_gpufamily2_v1) — The GPU family 2, version 1 feature set for tvOS.

### tvOS GPU family 1
- [MTLFeatureSet.tvOS_GPUFamily1_v4](https://developer.apple.com/documentation/metal/mtlfeatureset/tvos_gpufamily1_v4) — The GPU family 1, version 4 feature set for tvOS.
- [MTLFeatureSet.tvOS_GPUFamily1_v3](https://developer.apple.com/documentation/metal/mtlfeatureset/tvos_gpufamily1_v3) — The GPU family 1, version 3 feature set for tvOS.
- [MTLFeatureSet.tvOS_GPUFamily1_v2](https://developer.apple.com/documentation/metal/mtlfeatureset/tvos_gpufamily1_v2) — The GPU family 1, version 2 feature set for tvOS.
- [MTLFeatureSet.tvOS_GPUFamily1_v1](https://developer.apple.com/documentation/metal/mtlfeatureset/tvos_gpufamily1_v1-swift.enum.case) — The GPU family 1, version 1 feature set for tvOS.

### macOS GPU family 2
- [MTLFeatureSet.macOS_GPUFamily2_v1](https://developer.apple.com/documentation/metal/mtlfeatureset/macos_gpufamily2_v1) — The GPU family 2, version 1 feature set for macOS.

### macOS GPU family 1
- [MTLFeatureSet.macOS_GPUFamily1_v4](https://developer.apple.com/documentation/metal/mtlfeatureset/macos_gpufamily1_v4) — The GPU family 1, version 4 feature set for macOS.
- [MTLFeatureSet.macOS_GPUFamily1_v3](https://developer.apple.com/documentation/metal/mtlfeatureset/macos_gpufamily1_v3) — The GPU family 1, version 3 feature set for macOS.
- [MTLFeatureSet.macOS_GPUFamily1_v2](https://developer.apple.com/documentation/metal/mtlfeatureset/macos_gpufamily1_v2) — The GPU family 1, version 2 feature set for macOS.
- [MTLFeatureSet.macOS_GPUFamily1_v1](https://developer.apple.com/documentation/metal/mtlfeatureset/macos_gpufamily1_v1) — The GPU family 1, version 1 feature set for macOS.

### macOS tier 2
- [MTLFeatureSet.macOS_ReadWriteTextureTier2](https://developer.apple.com/documentation/metal/mtlfeatureset/macos_readwritetexturetier2) — The read-write texture, tier 2 feature set for macOS.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlfeatureset/init(rawvalue:))

### Type Properties
- [osx_GPUFamily1_v1](https://developer.apple.com/documentation/metal/mtlfeatureset/osx_gpufamily1_v1)
- [osx_GPUFamily1_v2](https://developer.apple.com/documentation/metal/mtlfeatureset/osx_gpufamily1_v2)
- [osx_ReadWriteTextureTier2](https://developer.apple.com/documentation/metal/mtlfeatureset/osx_readwritetexturetier2)
- [tvos_GPUFamily1_v1](https://developer.apple.com/documentation/metal/mtlfeatureset/tvos_gpufamily1_v1-swift.type.property)

## See also

### Checking a GPU device’s feature support
- [supportsFamily(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsfamily(_:)) — Returns a Boolean value that indicates whether the GPU device supports the feature set of a specific GPU family.
- [MTLGPUFamily](https://developer.apple.com/documentation/metal/mtlgpufamily) — Represents the functionality for families of GPUs.
- [supportsFeatureSet(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsfeatureset(_:)) — Returns a Boolean value that indicates whether the GPU device supports a specific feature set.
