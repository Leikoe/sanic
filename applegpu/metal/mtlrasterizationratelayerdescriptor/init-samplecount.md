# init(sampleCount:)

*Initializer · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/init(samplecount:)>

Initializes the layer map with an empty grid.

## Declaration

```swift
init(sampleCount: MTLSize)
```

## Parameters

- **sampleCount** — The size of the grid. Specify the width and height to determine the number of columns and rows in the layer map. The initializer ignores the depth component.

## Return Value

A layer descriptor with a grid of the specified size. All of the rasterization rates are set to `0.0`.

## See also

### Creating a layer rasterization rate descriptor
- [init(horizontal:vertical:)](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerdescriptor/init(horizontal:vertical:)) — Initializes a layer rate map with a set of horizontal and vertical rasterization rates.
