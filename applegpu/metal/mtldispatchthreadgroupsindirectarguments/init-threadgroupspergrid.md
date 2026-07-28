# init(threadgroupsPerGrid:)

*Initializer · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments/init(threadgroupspergrid:)>

Returns a new data layout for dispatching threadgroups over indirect buffer calls, with specified threadgroups per grid.

## Declaration

```swift
init(threadgroupsPerGrid: (UInt32, UInt32, UInt32))
```

## Parameters

- **threadgroupsPerGrid** — The number of threadgroups for the grid, in each dimension.

## See also

### Specifying the size of the threadgroup
- [init()](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments/init()) — Returns a new data layout for dispatching threadgroups over indirect buffer calls.
- [threadgroupsPerGrid](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments/threadgroupspergrid) — The number of threadgroups for the grid, in each dimension.
