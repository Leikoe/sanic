# privateFunctionDescriptors

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4staticlinkingdescriptor/privatefunctiondescriptors>

Provides an array of private functions to link at the Metal IR level.

## Declaration

```swift
var privateFunctionDescriptors: [MTL4FunctionDescriptor]? { get set }
```

## Discussion

You specify private functions to link separately from [functionDescriptors](https://developer.apple.com/documentation/metal/mtl4staticlinkingdescriptor/functiondescriptors) because pipelines don’t export private functions as [MTLFunctionHandle](https://developer.apple.com/documentation/metal/mtlfunctionhandle) instances.

> **Note:**
> You can link private functions even when your [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) doesn’t support function pointers.
