# MTLVisibilityResultMode.boolean

*Case · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvisibilityresultmode/boolean>

The result records whether any samples passed depth and stencil tests.

## Declaration

```swift
case boolean
```

## Discussion

The GPU writes a 64-bit integer to the visibility result buffer that is nonzero if at least one fragment passed depth and stencil tests, and zero if no fragments passed the tests.

## See also

### Result modes
- [MTLVisibilityResultMode.disabled](https://developer.apple.com/documentation/metal/mtlvisibilityresultmode/disabled) — The result doesn’t contain any data because visibility testing was disabled.
- [MTLVisibilityResultMode.counting](https://developer.apple.com/documentation/metal/mtlvisibilityresultmode/counting) — The result records how many samples passed depth and stencil tests.
