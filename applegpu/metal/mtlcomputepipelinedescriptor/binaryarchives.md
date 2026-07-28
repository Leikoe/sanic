# binaryArchives

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/binaryarchives>

The binary archives that contain any precompiled shader functions to link.

## Declaration

```swift
var binaryArchives: [any MTLBinaryArchive]? { get set }
```

## Discussion

The default value is `nil`.

When you create a Metal library, Metal compiles shader functions into an intermediate representation. When you create the pipeline state object, the GPU compiles this intermediate code.

By providing a set of binary archives, when Metal creates the pipeline state object, it first checks the archives to see if there’s already a compiled function. If so, Metal uses it instead.

## See also

### Loading binary archives
- [supportAddingBinaryFunctions](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/supportaddingbinaryfunctions) — A Boolean value that indicates whether you can use the pipeline to create new pipelines by adding binary functions to its callable functions list.
