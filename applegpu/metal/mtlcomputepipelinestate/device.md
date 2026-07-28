# device

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/device>

The device instance that created the pipeline state.

## Declaration

```swift
var device: any MTLDevice { get }
```

## Discussion

This compute state instance is only usable on the device set in this property.

## See also

### Identifying a pipeline state
- [gpuResourceID](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/gpuresourceid) — An unique identifier that represents the pipeline state, which you can add to an argument buffer.
- [label](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/label) — A string that helps you identify the compute pipeline state during debugging.
