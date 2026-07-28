# device

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandqueue/device>

The GPU device that creates the command queue.

## Declaration

```swift
var device: any MTLDevice { get }
```

## Discussion

The command queue can submit work only to the GPU the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance represents.

## See also

### Identifying the command queue
- [label](https://developer.apple.com/documentation/metal/mtlcommandqueue/label) — An optional name that can help you identify the command queue.
