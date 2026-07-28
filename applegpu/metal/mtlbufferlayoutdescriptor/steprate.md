# stepRate

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/steprate>

How frequently the step function should load data.

## Declaration

```swift
var stepRate: Int { get set }
```

## Discussion

The interpretation of this value depends on the setting of `stepFunction`.

## See also

### Describing fetch behavior
- [stride](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/stride) — The number of bytes from one buffer entry to the next.
- [stepFunction](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/stepfunction) — Determines how and when compute functions fetch data.
- [MTLStepFunction](https://developer.apple.com/documentation/metal/mtlstepfunction) — The frequency and locations at which a function fetches attribute data.
