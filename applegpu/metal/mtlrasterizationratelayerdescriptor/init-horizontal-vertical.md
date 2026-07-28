# init(horizontal:vertical:)

*Initializer · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/init(horizontal:vertical:)>

Initializes a layer rate map with a set of horizontal and vertical rasterization rates.

## Declaration

```swift
convenience init(horizontal: [Float], vertical: [Float])
```

## Parameters

- **horizontal** — An array of the horizontal rates to apply across the grid.
- **vertical** — An array of the vertical rates to apply across the grid.

## Return Value

A layer descriptor whose width is the number of horizontal rates and whose height is the number of vertical rates. The layer descriptor copies the values from the input parameters.

## See also

### Creating a layer rasterization rate descriptor
- [init(sampleCount:)](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/init(samplecount:)) — Initializes the layer map with an empty grid.
