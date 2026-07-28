# privateFunctions

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 15.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtllinkedfunctions/privatefunctions>

An array of function objects to link to the new function, without exporting the functions publicly.

## Declaration

```swift
var privateFunctions: [any MTLFunction]? { get set }
```

## Discussion

The pipeline doesn’t export these functions as [MTLFunctionHandle](https://developer.apple.com/documentation/metal/mtlfunctionhandle) instances because the Metal device doesn’t need to support function pointers to link private functions.

## See also

### Specifying related functions
- [functions](https://developer.apple.com/documentation/metal/mtllinkedfunctions/functions) — An array of function objects to link to the new function.
- [binaryFunctions](https://developer.apple.com/documentation/metal/mtllinkedfunctions/binaryfunctions) — An array of function objects already compiled to a binary representation to link.
- [groups](https://developer.apple.com/documentation/metal/mtllinkedfunctions/groups) — An optional list of groups specifying which functions your shader can call at each call site.
