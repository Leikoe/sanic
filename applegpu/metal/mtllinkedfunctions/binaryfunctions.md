# binaryFunctions

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtllinkedfunctions/binaryfunctions>

An array of function objects already compiled to a binary representation to link.

## Declaration

```swift
var binaryFunctions: [any MTLFunction]? { get set }
```

## See also

### Specifying related functions
- [functions](https://developer.apple.com/documentation/metal/mtllinkedfunctions/functions) — An array of function objects to link to the new function.
- [groups](https://developer.apple.com/documentation/metal/mtllinkedfunctions/groups) — An optional list of groups specifying which functions your shader can call at each call site.
- [privateFunctions](https://developer.apple.com/documentation/metal/mtllinkedfunctions/privatefunctions) — An array of function objects to link to the new function, without exporting the functions publicly.
