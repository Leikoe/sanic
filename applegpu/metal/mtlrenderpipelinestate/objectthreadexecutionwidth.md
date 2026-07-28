# objectThreadExecutionWidth

*Instance Property · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/objectthreadexecutionwidth>

The number of threads the render pass applies to a SIMD group for an object shader.

## Declaration

```swift
var objectThreadExecutionWidth: Int { get }
```

## Discussion

You can access the value of this property in your shader code by adding an integer parameter with the `[[threads_per_simdgroup]]` attribute. For more information about this attribute, see the [Metal Shading Language Specification (PDF)](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf).

## See also

### Checking object shader memory requirements
- [maxTotalThreadsPerObjectThreadgroup](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/maxtotalthreadsperobjectthreadgroup) — The largest number of threads the pipeline state can have in a single object shader threadgroup.
