# device

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsamplerstate/device>

The device object that created the sampler.

## Declaration

```swift
var device: any MTLDevice { get }
```

## Discussion

A sampler is always associated with the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) that created it and can be used only with that device.

## See also

### Identifying the sampler
- [label](https://developer.apple.com/documentation/metal/mtlsamplerstate/label) — A string that identifies the sampler.
