# label

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/label>

A string that helps you identify the compute pipeline state during debugging.

## Declaration

```swift
var label: String? { get }
```

## Discussion

Labels are useful identifiers at runtime or when profiling and debugging your app using any Metal tool. See [Naming resources and commands](https://developer.apple.com/documentation/Xcode/Naming-resources-and-commands).

## See also

### Identifying a pipeline state
- [device](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/device) — The device instance that created the pipeline state.
- [gpuResourceID](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/gpuresourceid) — An unique identifier that represents the pipeline state, which you can add to an argument buffer.
