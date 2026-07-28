# maximumConcurrentCompilationTaskCount

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 13.3, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtldevice/maximumconcurrentcompilationtaskcount>

The maximum number of concurrent compilation tasks the device is running.

## Declaration

```swift
var maximumConcurrentCompilationTaskCount: Int { get }
```

## Discussion

The property’s value can change when you set the [shouldMaximizeConcurrentCompilation](https://developer.apple.com/documentation/metal/mtldevice/shouldmaximizeconcurrentcompilation) property to a new value.
