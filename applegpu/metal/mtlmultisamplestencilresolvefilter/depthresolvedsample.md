# MTLMultisampleStencilResolveFilter.depthResolvedSample

*Case · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlmultisamplestencilresolvefilter/depthresolvedsample>

Chooses the stencil sample corresponding to the depth sample selected by the depth resolve filter.

## Declaration

```swift
case depthResolvedSample
```

## Discussion

The resolve filter selects the stencil sample corresponding to the sample that the depth resolve filter would have selected.

## See also

### Stencil resolve filters
- [MTLMultisampleStencilResolveFilter.sample0](https://developer.apple.com/documentation/metal/mtlmultisamplestencilresolvefilter/sample0) — Chooses the first stencil sample in the pixel.
