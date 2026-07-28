# label

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandqueue/label>

An optional name that can help you identify the command queue.

## Declaration

```swift
var label: String? { get set }
```

## Discussion

Set labels to help you quickly identify a GPU at runtime in the Metal debugging and profiling tools. See [Naming resources and commands](https://developer.apple.com/documentation/Xcode/Naming-resources-and-commands).

## See also

### Identifying the command queue
- [device](https://developer.apple.com/documentation/metal/mtlcommandqueue/device) — The GPU device that creates the command queue.
