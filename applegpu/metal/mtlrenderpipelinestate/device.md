# device

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/device>

The device instance that creates the pipeline state.

## Declaration

```swift
var device: any MTLDevice { get }
```

## Discussion

You can only use the pipeline state object with this device object.

## See also

### Identifying a pipeline state
- [label](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/label) — A string that helps you identify the render pipeline state during debugging.
- [gpuResourceID](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/gpuresourceid) — An unique identifier that represents the pipeline state, which you can add to an argument buffer.
