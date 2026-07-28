# device

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.14, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsharedtexturehandle/device>

The device object that created the texture.

## Declaration

```swift
var device: any MTLDevice { get }
```

## Discussion

A texture is always associated with the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) that created it and can be used only with that device.

## See also

### Identifying the shared texture handle
- [label](https://developer.apple.com/documentation/metal/mtlsharedtexturehandle/label) — A string that identifies the texture.
