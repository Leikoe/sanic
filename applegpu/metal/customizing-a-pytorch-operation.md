# Customizing a PyTorch operation

*Sample Code*

<https://developer.apple.com/documentation/metal/customizing-a-pytorch-operation>

Implement a custom operation in PyTorch that uses Metal kernels to improve performance.

## Overview

> **Note:**
> This sample code project is associated with WWDC23 session 10050: [Optimize machine learning for Metal apps](https://developer.apple.com/wwdc23/10050).

### Configure the sample code project

Before you run the sample code project:

1. Follow the instructions in [Accelerated PyTorch training on Mac](https://developer.apple.com/metal/pytorch/).

2. Install PyTorch nightly (Python 3.7 or later is required).

```shell
pip3 install --pre torch --index-url https://download.pytorch.org/whl/nightly/cpu
```

1. Install Ninja

```shell
pip3 install Ninja
```

1. Run the sample.

```shell
python3 run_sample.py
```

## See also

### Compute workflows
- [Performing calculations on a GPU](https://developer.apple.com/documentation/metal/performing-calculations-on-a-gpu) — Use Metal to find GPUs and perform calculations on them.
- [Selecting device objects for compute processing](https://developer.apple.com/documentation/metal/selecting-device-objects-for-compute-processing) — Switch dynamically between multiple GPUs to efficiently execute a compute-intensive simulation.
- [Customizing a TensorFlow operation](https://developer.apple.com/documentation/metal/customizing-a-tensorflow-operation) — Implement a custom operation that uses Metal kernels to accelerate neural-network training performance.
