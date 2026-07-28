# supportsFamily(_:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/supportsfamily(_:)>

Returns a Boolean value that indicates whether the GPU device supports the feature set of a specific GPU family.

## Declaration

```swift
func supportsFamily(_ gpuFamily: MTLGPUFamily) -> Bool
```

## Parameters

- **gpuFamily** — An [MTLGPUFamily](https://developer.apple.com/documentation/metal/mtlgpufamily) instance.

## See also

### Checking a GPU device’s feature support
- [MTLGPUFamily](https://developer.apple.com/documentation/metal/mtlgpufamily) — Represents the functionality for families of GPUs.
- [supportsFeatureSet(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsfeatureset(_:)) — Returns a Boolean value that indicates whether the GPU device supports a specific feature set.
- [MTLFeatureSet](https://developer.apple.com/documentation/metal/mtlfeatureset) — The device feature sets that define specific platform, hardware, and software configurations.
