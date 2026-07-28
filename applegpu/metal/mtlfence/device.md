# device

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfence/device>

The device object that created the fence.

## Declaration

```swift
var device: any MTLDevice { get }
```

## Discussion

Only the device that created the fence can use it.

## See also

### Identifying a fence
- [label](https://developer.apple.com/documentation/metal/mtlfence/label) — A string that identifies the fence.
