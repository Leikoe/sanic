# cpuCacheMode

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltensordescriptor/cpucachemode>

A value that configures the cache mode of CPU mapping of tensors you create with this descriptor.

## Declaration

```swift
var cpuCacheMode: MTLCPUCacheMode { get set }
```

## Discussion

The default value of this property is [MTLCPUCacheMode.defaultCache](https://developer.apple.com/documentation/metal/mtlcpucachemode/defaultcache).
