# setVisibilityResultMode(_:offset:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setvisibilityresultmode(_:offset:)>

Configures a visibility test for Metal to run, and the destination for any results it generates.

## Declaration

```swift
func setVisibilityResultMode(_ mode: MTLVisibilityResultMode, offset: Int)
```

## Parameters

- **mode** — A [MTLVisibilityResultMode](https://developer.apple.com/documentation/metal/mtlvisibilityresultmode) that configures which visibility test results the render pass saves to a buffer, or disables visibility testing.
- **offset** — A location, in bytes, relative to the start of [visibilityResultBuffer](https://developer.apple.com/documentation/metal/mtl4renderpassdescriptor/visibilityresultbuffer) The GPU stores the result of a visibility test at `offset`, which needs to be a multiple of `8`.

## Discussion

You use the `mode` parameter to enable or disable the visibility test, and determine if it produces a boolean response for passing fragments, or if it counts the number of fragments.
