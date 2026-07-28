# MTLVisibilityResultMode.counting

*Case · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvisibilityresultmode/counting>

The result records how many samples passed depth and stencil tests.

## Declaration

```swift
case counting
```

## Discussion

The GPU writes a 64-bit integer to the visibility result buffer that is the number of samples that passed depth and stencil tests; this can be zero. Counting is not supported by all GPUs. Check the following documents to see whether a GPU family supports *counting occlusion* queries:

- [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf)

- [Metal feature set tables (Numbers)](https://developer.apple.com/metal/metal-feature-set-tables.zip)

## See also

### Result modes
- [MTLVisibilityResultMode.disabled](https://developer.apple.com/documentation/metal/mtlvisibilityresultmode/disabled) — The result doesn’t contain any data because visibility testing was disabled.
- [MTLVisibilityResultMode.boolean](https://developer.apple.com/documentation/metal/mtlvisibilityresultmode/boolean) — The result records whether any samples passed depth and stencil tests.
