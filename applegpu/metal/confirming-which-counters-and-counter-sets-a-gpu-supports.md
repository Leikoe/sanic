# Confirming which counters and counter sets a GPU supports

*Article*

<https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports>

Check whether a GPU produces the runtime performance data you want to sample.

## Overview

You can check to see which counters a GPU device supports before attempting to programmatically sample data from them. A GPU *counter* is typically a hardware feature that tracks a specific performance metric, such as timestamps before and after an important rendering stage. Checking which counters a GPU supports at runtime avoids assumptions that may trigger an error or a crash on a person’s device.

A *counter set* is a collection of related counters. The [MTLCommonCounterSet](https://developer.apple.com/documentation/metal/mtlcommoncounterset) type defines the name of each set, and the [MTLCommonCounter](https://developer.apple.com/documentation/metal/mtlcommoncounter) type defines the name of each individual counter. Check which counter sets an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance supports before you request one by checking its [counterSets](https://developer.apple.com/documentation/metal/mtldevice/countersets) property.

```swift
func getCounterSet(_ commonCounterSetName: MTLCommonCounterSet,
                   from device: MTLDevice) -> MTLCounterSet? {
    let counterSetName = commonCounterSetName.rawValue

    guard let counterSets = device.counterSets else {
        print("GPU device \"\(device.name)\" doesn't support any counter sets.")
        return nil
    }

    for counterSet in counterSets {
        guard counterSet.name == counterSetName else { continue }

        print("GPU device \"\(device.name)\" supports the \"\(counterSetName)\" counter set.")
        return counterSet
    }

    print("GPU device \"\(device.name)\" doesn't support the \"\(counterSetName)\" counter set.")
    return nil
}
```

```objective-c
+ (nullable id<MTLCounterSet>) getCounterSet:(MTLCommonCounterSet)counterSetName
                                  fromDevice:(id<MTLDevice>)device
{
    for (id<MTLCounterSet> counterSet in device.counterSets ){
        if ([counterSetName isEqualToString:counterSetName])
        {
            printf("GPU device \"%s\" supports the \"%s\" counter set.\n",
                   device.name.UTF8String,
                   counterSetName.UTF8String);

            return counterSet;
        }
    }

    printf("GPU device \"%s\" doesn't support the \"%s\" counter set.\n",
           device.name.UTF8String,
           counterSetName.UTF8String);

    return nil;
}
```

The method iterates through each counter set the device supports, and compares its name to the [MTLCommonCounterSet](https://developer.apple.com/documentation/metal/mtlcommoncounterset) parameter.

Even though a GPU device supports a counter set, it may only support some of the counters in that set. You can check which individual counters a device supports by checking the elements of a counter set’s [counters](https://developer.apple.com/documentation/metal/mtlcounterset/counters) property.

```swift
static func counterSet(_ counterSet: MTLCounterSet,
                       contains commonCounterName: MTLCommonCounter) -> Bool {

    let counterName = commonCounterName.rawValue
    for counter in counterSet.counters {
        guard counter.name == counterName else { continue }

        print("Counter set \"\(counterSet.name)\" contains the \"\(counterName)\" counter.")
        return true
    }

    print("Counter set \"\(counterSet.name)\" doesn't contain the \"\(counterName)\" counter.")
    return false
}
```

```objective-c
+ (bool)counterSet:(id<MTLCounterSet>)counterSet
          contains:(MTLCommonCounter)counterName
{
    for (id<MTLCounter> counter in counterSet.counters )
    {
        if ([counter.name isEqualToString:counterName])
        {
            printf("Counter set \"%s\" supports the \"%s\" counter set.\n",
                   counterSet.name.UTF8String,
                   counterName.UTF8String);
            return YES;
        }
    }

    printf("Counter set \"%s\" doesn't support the \"%s\" counter set.\n",
           counterSet.name.UTF8String,
           counterName.UTF8String);

    return NO;
}
```

The method iterates through each counter in the device’s counter set, and compares its name to the [MTLCommonCounter](https://developer.apple.com/documentation/metal/mtlcommoncounter) property.

When you know a GPU supports a counter, your app can create and safely use an [MTLCounterSampleBuffer](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer) instance to store that counter’s data. See [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass) for more information and next steps.

## See also

### Counters and counter sets
- [MTLCounterSet](https://developer.apple.com/documentation/metal/mtlcounterset) — A collection of individual counters a GPU device supports for a counter set.
- [MTLCommonCounterSet](https://developer.apple.com/documentation/metal/mtlcommoncounterset) — The name of a specific counter set that a GPU device can support.
- [MTLCounter](https://developer.apple.com/documentation/metal/mtlcounter) — An individual counter a GPU device lists within one of its counter sets.
- [MTLCommonCounter](https://developer.apple.com/documentation/metal/mtlcommoncounter) — The name of a specific counter that can appear in a GPU device’s counter sets.
