# groups

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4staticlinkingdescriptor/groups>

Assigns groups of functions to match call-site attributes in shader code.

## Declaration

```swift
var groups: [String : [MTL4FunctionDescriptor]]? { get set }
```

## Discussion

Function groups help the compiler reduce the number of candidate functions it needs to evaluate for shader function calls, potentially increasing runtime performance.
