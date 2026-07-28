# setTileAccelerationStructure(_:bufferIndex:)

*Instance Method · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settileaccelerationstructure(_:bufferindex:)>

Assigns an acceleration structure to an entry in the tile shader argument table.

## Declaration

```swift
func setTileAccelerationStructure(_ accelerationStructure: (any MTLAccelerationStructure)?, bufferIndex: Int)
```

## Parameters

- **accelerationStructure** — An [MTLAccelerationStructure](https://developer.apple.com/documentation/metal/mtlaccelerationstructure) instance the command assigns to an entry in the tile shader argument table for acceleration structures.
- **bufferIndex** — An integer that represents the entry in the tile shader argument table for acceleration structures that stores a record of `accelerationStructure`.

## Discussion

By default, the acceleration structure at each index is `nil`.
