# compareFunction

*Instance Property · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/comparefunction>

The sampler comparison function used when performing a sample compare operation on a depth texture.

## Declaration

```swift
var compareFunction: MTLCompareFunction { get set }
```

## Discussion

The default value is [MTLCompareFunction.never](https://developer.apple.com/documentation/metal/mtlcomparefunction/never).

The [MTLFeatureSet.iOS_GPUFamily3_v1](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily3_v1) and [MTLFeatureSet.iOS_GPUFamily1_v1](https://developer.apple.com/documentation/metal/mtlfeatureset/ios_gpufamily1_v1) feature sets allow you to define a framework-side sampler comparison function for an [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instance. All feature sets support shader-side sampler comparison functions, as described in the [Metal Shading Language Specification](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf).

## See also

### Declaring the depth comparison mode
- [MTLCompareFunction](https://developer.apple.com/documentation/metal/mtlcomparefunction) — Options used to specify how a sample compare operation should be performed on a depth texture.
