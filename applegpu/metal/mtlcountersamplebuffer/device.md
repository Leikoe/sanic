# device

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcountersamplebuffer/device>

The GPU device instance that owns the counter sample buffer.

## Declaration

```swift
var device: any MTLDevice { get }
```

## Discussion

You can store a GPU device’s counter set data only with a counter sample buffer that you create from the same device.

## See also

### Inspecting the counter sample buffer’s configuration
- [label](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer/label) — A string that identifies the counter sample buffer.
- [sampleCount](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer/samplecount) — The number of samples in the buffer.
