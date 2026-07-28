# setSamplerState(_:index:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4argumenttable/setsamplerstate(_:index:)>

Binds a sampler state to a sampler state binding slot.

## Declaration

```swift
func setSamplerState(_ resourceID: MTLResourceID, index bindingIndex: Int)
```

## Parameters

- **resourceID** — The [MTLResourceID](https://developer.apple.com/documentation/metal/mtlresourceid) of the [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instance to bind.
- **bindingIndex** — A valid binding index in the sampler binding range. It is an error for this value to match or exceed the value of property [maxSamplerStateBindCount](https://developer.apple.com/documentation/metal/mtl4argumenttabledescriptor/maxsamplerstatebindcount) on the descriptor from which you created this argument table.
