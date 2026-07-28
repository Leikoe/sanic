# imageblockMemoryLength(forDimensions:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/imageblockmemorylength(fordimensions:)>

Returns the length of reserved memory for an imageblock of a given size.

## Declaration

```swift
func imageblockMemoryLength(forDimensions imageblockDimensions: MTLSize) -> Int
```

## Parameters

- **imageblockDimensions** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the dimensions of an imageblock.

## Return Value

The length, in bytes, occupied by the image block in memory.
