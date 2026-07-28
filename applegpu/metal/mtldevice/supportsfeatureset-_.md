# supportsFeatureSet(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/supportsfeatureset(_:)>

Returns a Boolean value that indicates whether the GPU device supports a specific feature set.

## Declaration

```swift
func supportsFeatureSet(_ featureSet: MTLFeatureSet) -> Bool
```

## Parameters

- **featureSet** — An [MTLFeatureSet](https://developer.apple.com/documentation/metal/mtlfeatureset) instance.

## See also

### Related Documentation
- [Detecting GPU features and Metal software versions](https://developer.apple.com/documentation/metal/detecting-gpu-features-and-metal-software-versions) — Use the device object’s properties to determine how you perform tasks in Metal.

### Checking a GPU device’s feature support
- [supportsFamily(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsfamily(_:)) — Returns a Boolean value that indicates whether the GPU device supports the feature set of a specific GPU family.
- [MTLGPUFamily](https://developer.apple.com/documentation/metal/mtlgpufamily) — Represents the functionality for families of GPUs.
- [MTLFeatureSet](https://developer.apple.com/documentation/metal/mtlfeatureset) — The device feature sets that define specific platform, hardware, and software configurations.
