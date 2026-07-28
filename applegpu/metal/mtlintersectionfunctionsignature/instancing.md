# instancing

*Type Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature/instancing>

A flag indicating that function signature uses instancing.

## Declaration

```swift
static var instancing: MTLIntersectionFunctionSignature { get }
```

## Discussion

The corresponding MSL function needs to contain the `instancing` tag in its declaration.

## See also

### Specifying the intersection function signature
- [triangleData](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature/triangledata) — A flag indicating that function signature uses triangle data.
- [worldSpaceData](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature/worldspacedata) — A flag indicating that function signature uses world space data.
