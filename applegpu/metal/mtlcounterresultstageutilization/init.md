# init()

*Initializer · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization/init()>

Creates a default stage-utilization result.

## Declaration

```swift
init()
```

## Discussion

Metal creates [MTLCounterResultStageUtilization](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization) instances for you when you resolve the counter set’s data (see [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format)). There’s no reason for you to manually create one in your app.

## See also

### Swift support
- [init(totalCycles:vertexCycles:tessellationCycles:postTessellationVertexCycles:fragmentCycles:renderTargetCycles:)](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization/init(totalcycles:vertexcycles:tessellationcycles:posttessellationvertexcycles:fragmentcycles:rendertargetcycles:)) — Creates a stage-utilization result from utilization values.
