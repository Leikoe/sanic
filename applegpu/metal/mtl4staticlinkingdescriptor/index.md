# MTL4StaticLinkingDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4staticlinkingdescriptor>

Groups together properties to drive a static linking process.

## Declaration

```swift
class MTL4StaticLinkingDescriptor
```

## Topics

### Instance Properties
- [functionDescriptors](https://developer.apple.com/documentation/metal/mtl4staticlinkingdescriptor/functiondescriptors) — Provides an array of functions to link at the Metal IR level.
- [groups](https://developer.apple.com/documentation/metal/mtl4staticlinkingdescriptor/groups) — Assigns groups of functions to match call-site attributes in shader code.
- [privateFunctionDescriptors](https://developer.apple.com/documentation/metal/mtl4staticlinkingdescriptor/privatefunctiondescriptors) — Provides an array of private functions to link at the Metal IR level.

## See also

### Pipeline compilation
- [MTL4BlendState](https://developer.apple.com/documentation/metal/mtl4blendstate) — Enumeration for controlling the blend state of a pipeline state object.
- [MTL4FunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4functiondescriptor) — Base interface for describing a Metal 4 shader function.
- [MTL4IndirectCommandBufferSupportState](https://developer.apple.com/documentation/metal/mtl4indirectcommandbuffersupportstate) — Enumeration for controlling support for [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer).
- [MTL4LibraryDescriptor](https://developer.apple.com/documentation/metal/mtl4librarydescriptor) — Serves as the base descriptor for creating a Metal library.
- [MTL4LibraryFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4libraryfunctiondescriptor) — Describes a shader function from a Metal library.
- [MTL4LogicalToPhysicalColorAttachmentMappingState](https://developer.apple.com/documentation/metal/mtl4logicaltophysicalcolorattachmentmappingstate) — Enumerates possible behaviors of how a pipeline maps its logical outputs to its color attachments.
- [MTL4NewBinaryFunctionCompletionHandler](https://developer.apple.com/documentation/metal/mtl4newbinaryfunctioncompletionhandler) — Provides a signature for a callback block that Metal calls when the compiler finishes a build task for a binary function.
- [MTL4NewMachineLearningPipelineStateCompletionHandler](https://developer.apple.com/documentation/metal/mtl4newmachinelearningpipelinestatecompletionhandler) — Provides a signature for a callback block that Metal calls when the compiler finishes a build task for a machine learning pipeline state.
- [MTL4ShaderReflection](https://developer.apple.com/documentation/metal/mtl4shaderreflection) — Option mask for requesting reflection information at pipeline build time.
- [MTL4SpecializedFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4specializedfunctiondescriptor) — Groups together properties to configure and create a specialized function by passing it to a factory method.
- [MTL4AlphaToCoverageState](https://developer.apple.com/documentation/metal/mtl4alphatocoveragestate) — Enumeration for controlling alpha-to-coverage state of a pipeline state object.
- [MTL4AlphaToOneState](https://developer.apple.com/documentation/metal/mtl4alphatoonestate) — Enumeration for controlling alpha-to-one state of a pipeline state object.
- [MTL4StitchedFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4stitchedfunctiondescriptor) — Groups together properties that describe a shader function suitable for stitching.
- [MTLFunctionReflection](https://developer.apple.com/documentation/metal/mtlfunctionreflection) — Represents a reflection object containing information about a function in a Metal library.
- [MTLNewDynamicLibraryCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewdynamiclibrarycompletionhandler)
