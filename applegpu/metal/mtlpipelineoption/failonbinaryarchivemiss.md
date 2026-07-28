# failOnBinaryArchiveMiss

*Type Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpipelineoption/failonbinaryarchivemiss>

An option that instructs the compiler to return an error when a GPU function isn’t in a binary archive.

## Declaration

```swift
static var failOnBinaryArchiveMiss: MTLPipelineOption { get }
```

## Discussion

By default, Metal compiles the functions for a pipeline state if they aren’t in a binary archive. When you set this option, Metal returns an error instead of compiling a missing function.

## See also

### Retrieving argument information
- [bufferTypeInfo](https://developer.apple.com/documentation/metal/mtlpipelineoption/buffertypeinfo) — An option instance that provides detailed buffer type information for buffer arguments.
- [argumentInfo](https://developer.apple.com/documentation/metal/mtlpipelineoption/argumentinfo) — An option instance that provides argument information for textures and threadgroup memory.
