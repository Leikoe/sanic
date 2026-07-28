# functions

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 15.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor/functions>

The list of functions for creating the stitched library.

## Declaration

```swift
var functions: [any MTLFunction] { get set }
```

## Discussion

The function objects need to all be created by the same Metal device object that you’ll use to create the library. The MSL functions referenced by these function objects need to be declared with the `stitchable` attribute, as in the example below:

```metal
[[stitchable]]
 float add(float a, float b)
{
    return a + b;
}
```

## See also

### Configuring a stitched library
- [functionGraphs](https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor/functiongraphs) — The function graphs that define the new stitched library’s functions.
