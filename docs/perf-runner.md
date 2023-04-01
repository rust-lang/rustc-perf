# CI benchmarking machine
The machine that actually executes the benchmarks in CI has the following configuration.

## Hardware
- 6-core AMD Ryzen 5 3600 with HyperThreading (12 hardware threads total)
    <details>
    <summary>Output of `lscpu`</summary>

    ```
    Architecture:        x86_64
    CPU op-mode(s):      32-bit, 64-bit
    Byte Order:          Little Endian
    CPU(s):              12
    On-line CPU(s) list: 0-11
    Thread(s) per core:  2
    Core(s) per socket:  6
    Socket(s):           1
    NUMA node(s):        1
    Vendor ID:           AuthenticAMD
    CPU family:          23
    Model:               113
    Model name:          AMD Ryzen 5 3600 6-Core Processor
    Stepping:            0
    CPU MHz:             3819.020
    CPU max MHz:         3600.0000
    CPU min MHz:         2200.0000
    BogoMIPS:            7186.58
    Virtualization:      AMD-V
    L1d cache:           32K
    L1i cache:           32K
    L2 cache:            512K
    L3 cache:            16384K
    NUMA node0 CPU(s):   0-11
    Flags:               fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
    ```

    </details>
- 64 GiB RAM, 32 GiB of swap

## Software
- Kernel: `4.15.0-191-generic`
- OS: Ubuntu `18.04`

## Configuration
### HyperThreading & Turboboost
As of November 2022, both are disabled.

Prior to October 2022, both were enabled. During October 2022 some
[experimentation](https://github.com/rust-lang/rustc-perf/issues/1450) was done
which found that disabling both reduced variance significantly.

### CPU scaling
Scaling governor set to `performance`.

### ASLR (Address Space Layout Randomization)
Disabled with `kernel.randomize_va_space=0` in `sysctl.conf`.

Note that ASLR is also
[disabled explicitly](../collector/src/bin/rustc-fake.rs) when gathering performance metrics.

### NMI watchdog
Disabled with `kernel.nmi_watchdog=0` in `sysctl.conf`.

### Swap
`vm.swappiness` set to `60`.
