# shouldMaximizeConcurrentCompilation

*Instance Property · macOS 13.3*

<https://developer.apple.com/documentation/metal/mtldevice/shouldmaximizeconcurrentcompilation>

A Boolean value that indicates whether the device uses additional CPU threads for compilation tasks.

## Declaration

```swift
var shouldMaximizeConcurrentCompilation: Bool { get set }
```

## Discussion

The property’s default value is [false](https://developer.apple.com/documentation/Swift/false). You can retrieve the number of concurrent CPU threads the device is currently using by checking the [maximumConcurrentCompilationTaskCount](https://developer.apple.com/documentation/metal/mtldevice/maximumconcurrentcompilationtaskcount) property.

> **Note:**
> The number of additional CPU threads automatically scales with the system’s hardware capabilities.
