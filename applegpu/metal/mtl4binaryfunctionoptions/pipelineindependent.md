# pipelineIndependent

*Type Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4binaryfunctionoptions/pipelineindependent>

Compiles the function to have its function handles return a constant MTLResourceID across all pipeline states. The function needs to be linked to the pipeline that will use this function.

## Declaration

```swift
static var pipelineIndependent: MTL4BinaryFunctionOptions { get }
```
