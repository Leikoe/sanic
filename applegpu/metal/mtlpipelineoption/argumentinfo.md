# argumentInfo

*Type Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpipelineoption/argumentinfo>

An option instance that provides argument information for textures and threadgroup memory.

## Declaration

```swift
static var argumentInfo: MTLPipelineOption { get }
```

## Discussion

This option provides all properties of an [MTLArgument](https://developer.apple.com/documentation/metal/mtlargument) instance, except for [bufferStructType](https://developer.apple.com/documentation/metal/mtlargument/bufferstructtype) and [bufferPointerType](https://developer.apple.com/documentation/metal/mtlargument/bufferpointertype), which are `nil`. To obtain these detailed buffer type properties, retrieve the [bufferTypeInfo](https://developer.apple.com/documentation/metal/mtlpipelineoption/buffertypeinfo) instance.

## See also

### Retrieving argument information
- [bufferTypeInfo](https://developer.apple.com/documentation/metal/mtlpipelineoption/buffertypeinfo) — An option instance that provides detailed buffer type information for buffer arguments.
- [failOnBinaryArchiveMiss](https://developer.apple.com/documentation/metal/mtlpipelineoption/failonbinaryarchivemiss) — An option that instructs the compiler to return an error when a GPU function isn’t in a binary archive.
