# storeFunctionInMetalPipelinesScript

*Type Property · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlfunctionoptions/storefunctioninmetalpipelinesscript>

An option that instructs the compiler to store function information for inspecting binary archives.

## Declaration

```swift
static var storeFunctionInMetalPipelinesScript: MTLFunctionOptions { get }
```

## Discussion

Set this option when you want to inspect or consume binary archives with the `metal-source` tool. You don’t need this option when you recompile functions or store them in binary archives.

## See also

### Function compilation options
- [failOnBinaryArchiveMiss](https://developer.apple.com/documentation/metal/mtlfunctionoptions/failonbinaryarchivemiss) — An option that instructs the compiler to return an error when a GPU function isn’t in a binary archive.
- [compileToBinary](https://developer.apple.com/documentation/metal/mtlfunctionoptions/compiletobinary) — An option that instructs the compiler to generate a binary format for dynamic linking.
- [pipelineIndependent](https://developer.apple.com/documentation/metal/mtlfunctionoptions/pipelineindependent) — An option that generates the same function handle across all pipeline states that link a function, which lets you share function tables across pipeline states.
- [storeFunctionInMetalScript](https://developer.apple.com/documentation/metal/mtlfunctionoptions/storefunctioninmetalscript) — An option that instructs the compiler to store function information for inspecting binary archives.
