# label

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/label>

An optional name that can help you identify the command buffer.

## Declaration

```swift
var label: String? { get set }
```

## Discussion

Set labels to help you quickly identify a command buffer at runtime in the Metal debugging and profiling tools. See [Naming resources and commands](https://developer.apple.com/documentation/Xcode/Naming-resources-and-commands).

## See also

### Identifying the command buffer
- [commandQueue](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commandqueue) — The command queue that creates the command buffer.
- [device](https://developer.apple.com/documentation/metal/mtlcommandbuffer/device) — The GPU device that indirectly owns the command buffer because you create it from a command queue the device also owns.
