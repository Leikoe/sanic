# pipelineIndependent

*Type Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlfunctionoptions/pipelineindependent>

An option that generates the same function handle across all pipeline states that link a function, which lets you share function tables across pipeline states.

## Declaration

```swift
static var pipelineIndependent: MTLFunctionOptions { get }
```

## Discussion

By default, when you link an [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) into a pipeline state, Metal generates a function handle that points to that function’s location in the pipeline’s executable code. Because different pipeline states place functions at different memory addresses, Metal generates different function handles for the same function in each pipeline state. You insert function handles into an [MTLIntersectionFunctionTable](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable) or [MTLVisibleFunctionTable](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable) instance, which means you need separate function tables for each pipeline state by default.

When you compile a function with this option, Metal generates the same function handle for the function across all pipeline states that link it. This consistency lets you create a single function table and use it with multiple pipeline states, which reduces memory overhead and simplifies function table management.

> **Note:**
> This option only works with functions that you compile with the [compileToBinary](https://developer.apple.com/documentation/metal/mtlfunctionoptions/compiletobinary) option.

## See also

### Function compilation options
- [failOnBinaryArchiveMiss](https://developer.apple.com/documentation/metal/mtlfunctionoptions/failonbinaryarchivemiss) — An option that instructs the compiler to return an error when a GPU function isn’t in a binary archive.
- [compileToBinary](https://developer.apple.com/documentation/metal/mtlfunctionoptions/compiletobinary) — An option that instructs the compiler to generate a binary format for dynamic linking.
- [storeFunctionInMetalPipelinesScript](https://developer.apple.com/documentation/metal/mtlfunctionoptions/storefunctioninmetalpipelinesscript) — An option that instructs the compiler to store function information for inspecting binary archives.
- [storeFunctionInMetalScript](https://developer.apple.com/documentation/metal/mtlfunctionoptions/storefunctioninmetalscript) — An option that instructs the compiler to store function information for inspecting binary archives.
