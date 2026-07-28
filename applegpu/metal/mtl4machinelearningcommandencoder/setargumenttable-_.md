# setArgumentTable(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder/setargumenttable(_:)>

Sets an argument table for the command encoder’s machine learning shader stage.

## Declaration

```swift
func setArgumentTable(_ argumentTable: (any MTL4ArgumentTable)?)
```

## Parameters

- **argumentTable** — An argument table to set on the command encoder’s Machine Learning stage.

## Discussion

The argument table provides inputs to all subsequent Machine Learning dispatches.

## See also

### Configuring the pass
- [setPipelineState(_:)](https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder/setpipelinestate(_:)) — Configures the encoder with a machine learning pipeline state instance.
