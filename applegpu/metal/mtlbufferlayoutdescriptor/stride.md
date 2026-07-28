# stride

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/stride>

The number of bytes from one buffer entry to the next.

## Declaration

```swift
var stride: Int { get set }
```

## Discussion

The default value is `1`. Check the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for potential alignment restrictions.

## See also

### Describing fetch behavior
- [stepFunction](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/stepfunction) — Determines how and when compute functions fetch data.
- [stepRate](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/steprate) — How frequently the step function should load data.
- [MTLStepFunction](https://developer.apple.com/documentation/metal/mtlstepfunction) — The frequency and locations at which a function fetches attribute data.
