# binaryLinkedFunctions

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4pipelinestagedynamiclinkingdescriptor/binarylinkedfunctions>

Provides the array of binary functions to link.

## Declaration

```swift
var binaryLinkedFunctions: [any MTL4BinaryFunction]? { get set }
```

## Discussion

Binary functions are shader functions that you compile from Metal IR to machine code ahead of time using instances of [MTL4Compiler](https://developer.apple.com/documentation/metal/mtl4compiler).
