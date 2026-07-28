# MTLGPUFamily

*Enumeration · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlgpufamily>

Represents the functionality for families of GPUs.

## Declaration

```swift
enum MTLGPUFamily
```

## Overview

Check whether a GPU supports the features of a specific family by calling the [supportsFamily(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsfamily(_:)) method of a GPU’s [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance.

## Topics

### Checking for Metal family GPU support
- [MTLGPUFamily.metal4](https://developer.apple.com/documentation/metal/mtlgpufamily/metal4)
- [MTLGPUFamily.metal3](https://developer.apple.com/documentation/metal/mtlgpufamily/metal3) — Represents the Metal 3 features.

### Checking for Apple family GPU support
- [MTLGPUFamily.apple9](https://developer.apple.com/documentation/metal/mtlgpufamily/apple9) — Represents the Apple family 9 GPU features that correspond to the Apple A17, M3, and M4 GPUs.
- [MTLGPUFamily.apple8](https://developer.apple.com/documentation/metal/mtlgpufamily/apple8) — Represents the Apple family 8 GPU features that correspond to the Apple A15, A16, and M2 GPUs.
- [MTLGPUFamily.apple7](https://developer.apple.com/documentation/metal/mtlgpufamily/apple7) — Represents the Apple family 7 GPU features that correspond to the Apple A14 and M1 GPUs.
- [MTLGPUFamily.apple6](https://developer.apple.com/documentation/metal/mtlgpufamily/apple6) — Represents the Apple family 6 GPU features that correspond to the Apple A13 GPUs.
- [MTLGPUFamily.apple5](https://developer.apple.com/documentation/metal/mtlgpufamily/apple5) — Represents the Apple family 5 GPU features that correspond to the Apple A12 GPUs.
- [MTLGPUFamily.apple4](https://developer.apple.com/documentation/metal/mtlgpufamily/apple4) — Represents the Apple family 4 GPU features that correspond to the Apple A11 GPUs.
- [MTLGPUFamily.apple3](https://developer.apple.com/documentation/metal/mtlgpufamily/apple3) — Represents the Apple family 3 GPU features that correspond to the Apple A9 and A10 GPUs.
- [MTLGPUFamily.apple2](https://developer.apple.com/documentation/metal/mtlgpufamily/apple2) — Represents the Apple family 2 GPU features that correspond to the Apple A8 GPUs.
- [MTLGPUFamily.apple1](https://developer.apple.com/documentation/metal/mtlgpufamily/apple1) — Represents the Apple family 1 GPU features that correspond to the Apple A7 GPUs.

### Checking for common GPU support
- [MTLGPUFamily.common3](https://developer.apple.com/documentation/metal/mtlgpufamily/common3) — Represents the Common family 3 GPU features.
- [MTLGPUFamily.common2](https://developer.apple.com/documentation/metal/mtlgpufamily/common2) — Represents the Common family 2 GPU features.
- [MTLGPUFamily.common1](https://developer.apple.com/documentation/metal/mtlgpufamily/common1) — Represents the Common family 1 GPU features.

### Checking for macOS family GPU support
- [MTLGPUFamily.mac2](https://developer.apple.com/documentation/metal/mtlgpufamily/mac2) — Represents the Mac family 2 GPU features.
- [MTLGPUFamily.mac1](https://developer.apple.com/documentation/metal/mtlgpufamily/mac1) — Represents the Mac family 1 GPU features.

### Checking for Mac Catalyst family GPU support
- [MTLGPUFamily.macCatalyst2](https://developer.apple.com/documentation/metal/mtlgpufamily/maccatalyst2) — Represents a family 2 Mac GPU when running an app you built with Mac Catalyst.
- [MTLGPUFamily.macCatalyst1](https://developer.apple.com/documentation/metal/mtlgpufamily/maccatalyst1) — Represents a family 1 Mac GPU when running an app you built with Mac Catalyst.

### Swift support
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlgpufamily/init(rawvalue:)) — Creates a GPU family instance from a raw value.

### Enumeration Cases
- [MTLGPUFamily.apple10](https://developer.apple.com/documentation/metal/mtlgpufamily/apple10)

## See also

### Checking a GPU device’s feature support
- [supportsFamily(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsfamily(_:)) — Returns a Boolean value that indicates whether the GPU device supports the feature set of a specific GPU family.
- [supportsFeatureSet(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsfeatureset(_:)) — Returns a Boolean value that indicates whether the GPU device supports a specific feature set.
- [MTLFeatureSet](https://developer.apple.com/documentation/metal/mtlfeatureset) — The device feature sets that define specific platform, hardware, and software configurations.
