# bufferTypeInfo

*Type Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpipelineoption/buffertypeinfo>

An option instance that provides detailed buffer type information for buffer arguments.

## Declaration

```swift
static var bufferTypeInfo: MTLPipelineOption { get }
```

## Discussion

This option provides the [bufferStructType](https://developer.apple.com/documentation/metal/mtlargument/bufferstructtype) and [bufferPointerType](https://developer.apple.com/documentation/metal/mtlargument/bufferpointertype) properties for the [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) stored in [argumentInfo](https://developer.apple.com/documentation/metal/mtlpipelineoption/argumentinfo).

## See also

### Retrieving argument information
- [failOnBinaryArchiveMiss](https://developer.apple.com/documentation/metal/mtlpipelineoption/failonbinaryarchivemiss) — An option that instructs the compiler to return an error when a GPU function isn’t in a binary archive.
- [argumentInfo](https://developer.apple.com/documentation/metal/mtlpipelineoption/argumentinfo) — An option instance that provides argument information for textures and threadgroup memory.
