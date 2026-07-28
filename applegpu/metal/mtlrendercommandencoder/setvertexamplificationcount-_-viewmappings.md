# setVertexAmplificationCount(_:viewMappings:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexamplificationcount(_:viewmappings:)>

Configures the number of output vertices the render pipeline produces for each input vertex, optionally with render target and viewport offsets.

## Declaration

```swift
func setVertexAmplificationCount(_ count: Int, viewMappings: UnsafePointer<MTLVertexAmplificationViewMapping>?)
```

## Parameters

- **count** — The number of outputs to create.
- **viewMappings** — An optional pointer to a C array that has at least `count` [MTLVertexAmplificationViewMapping](https://developer.apple.com/documentation/metal/mtlvertexamplificationviewmapping) elements. Each element in the array provides per-output offsets to a specific render target and viewport.

## Discussion

With *vertex amplification*, you can encode drawing commands that process the same vertex multiple times, one per render target. You can configure the render pipeline’s vertex amplification multiplier by calling this method with a `count` argument that’s greater than `1`.

> **Note:**
>  Render pipelines don’t apply vertex amplification by default.

For more information about vertex amplification and how to use the `viewMappings` parameter, see [Improving rendering performance with vertex amplification](https://developer.apple.com/documentation/metal/improving-rendering-performance-with-vertex-amplification).
