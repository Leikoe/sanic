# setArgumentTable(_:stages:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setargumenttable(_:stages:)>

Associates an argument table with a set of render stages.

## Declaration

```swift
func setArgumentTable(_ argumentTable: (any MTL4ArgumentTable)?, stages: MTLRenderStages)
```

## Parameters

- **argumentTable** — [MTL4ArgumentTable](https://developer.apple.com/documentation/metal/mtl4argumenttable) to set.
- **stages** — A [MTLRenderStages](https://developer.apple.com/documentation/metal/mtlrenderstages) bitmask that specifies the shader stages with visibility over the table.

## Discussion

Metal takes a snapshot of the resources in the argument table when you encode a draw, dispatch, or execute command. This snapshot becomes available to the `stages` you specify to this method.
