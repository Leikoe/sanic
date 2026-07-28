# label

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcountersamplebuffer/label>

A string that identifies the counter sample buffer.

## Declaration

```swift
var label: String { get }
```

## Discussion

Object and command labels are useful identifiers at runtime or when profiling and debugging your app using any Metal tool. See [Naming resources and commands](https://developer.apple.com/documentation/Xcode/Naming-resources-and-commands).

## See also

### Inspecting the counter sample buffer’s configuration
- [device](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer/device) — The GPU device instance that owns the counter sample buffer.
- [sampleCount](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer/samplecount) — The number of samples in the buffer.
