# Deployment

The machines that actually execute the benchmarks ("collectors") are dedicated machines running on [Hetzner](https://www.hetzner.com/dedicated-rootserver/). The [web server](http://perf.rust-lang.org/) runs on [ECS](https://github.com/rust-lang/infra-team/blob/HEAD/service-catalog/rustc-perf/README.md).

## Debugging
This section documents what to do in case benchmarking doesn't work or something is stuck. The status of the collectors can be found on the [status page](https://perf.rust-lang.org/status.html). In particular, it shows the last heartbeat of each collector. If that date is very old (>1 hour), then something bad has probably happened with the collector.

You can SSH into the machines directly and examine what is going on there. The currently active machines have the following domain names:

- `rustc-perf-one.infra.rust-lang.org`
- `rustc-perf-two.infra.rust-lang.org`

The benchmarking process runs as a systemd service called `collector`. You can start/stop/inspect it using the usual commands:
- Start/restart/stop: `sudo systemctl start/restart/stop collector.service`
- See logs: `sudo journalctl --utc -n 10000 -u collector -f`

The user account under which the benchmarks execute is called `collector`, you can switch to it using `su` and examine the `/home/collector/rustc-perf` checkout, from where are the benchmarks executed.

## Hardware
- The collectors run on `AX-42` Hetzner server instances.
- 8-core AMD Ryzen 7 PRO 8700GE with HyperThreading (16 hardware threads total)
    <details>
    <summary>Output of `lscpu`</summary>

    ```
    Architecture:           x86_64
    CPU op-mode(s):         32-bit, 64-bit
    Address sizes:          48 bits physical, 48 bits virtual
    Byte Order:             Little Endian
    CPU(s):                 16
    On-line CPU(s) list:    0-7
    Off-line CPU(s) list:   8-15
    Vendor ID:              AuthenticAMD
    Model name:             AMD Ryzen 7 PRO 8700GE w/ Radeon 780M Graphics
    CPU family:             25
    Model:                  117
    Thread(s) per core:     1
    Core(s) per socket:     8
    Socket(s):              1
    Stepping:               2
    CPU(s) scaling MHz:     88%
    CPU max MHz:            5184.0000
    CPU min MHz:            0.0000
    BogoMIPS:               7300.19
    Flags:                  fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 cl
    flush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm co
    nstant_tsc rep_good amd_lbr_v2 nopl nonstop_tsc cpuid extd_apicid aperfmpe
    rf rapl pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt ae
    s xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4
    a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core p
    erfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate ssbd mba pe
    rfmon_v2 ibrs ibpb stibp ibrs_enhanced vmmcall fsgsbase bmi1 avx2 smep bmi
    2 erms invpcid cqm rdt_a avx512f avx512dq rdseed adx smap avx512ifma clflu
    shopt clwb avx512cd sha_ni avx512bw avx512vl xsaveopt xsavec xgetbv1 xsave
    s cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local user_shstk avx512_bf16
    clzero irperf xsaveerptr rdpru wbnoinvd cppc arat npt lbrv svm_lock nrip_
    save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshol
    d avic vgif x2avic v_spec_ctrl vnmi avx512vbmi umip pku ospke avx512_vbmi2
    gfni vaes vpclmulqdq avx512_vnni avx512_bitalg avx512_vpopcntdq rdpid ove
    rflow_recov succor smca fsrm flush_l1d
    Virtualization features:  
    Virtualization:         AMD-V
    Caches (sum of all):      
    L1d:                    256 KiB (8 instances)
    L1i:                    256 KiB (8 instances)
    L2:                     8 MiB (8 instances)
    L3:                     16 MiB (1 instance)
    NUMA:                     
    NUMA node(s):           1
    NUMA node0 CPU(s):      0-7
    Vulnerabilities:          
    Gather data sampling:   Not affected
    Itlb multihit:          Not affected
    L1tf:                   Not affected
    Mds:                    Not affected
    Meltdown:               Not affected
    Mmio stale data:        Not affected
    Reg file data sampling: Not affected
    Retbleed:               Not affected
    Spec rstack overflow:   Mitigation; Safe RET
    Spec store bypass:      Mitigation; Speculative Store Bypass disabled via prctl
    Spectre v1:             Mitigation; usercopy/swapgs barriers and __user pointer sanitization
    Spectre v2:             Mitigation; Enhanced / Automatic IBRS; IBPB conditional; STIBP always-on;
    RSB filling; PBRSB-eIBRS Not affected; BHI Not affected
    Srbds:                  Not affected
    Tsx async abort:        Not affected
    ```

    </details>
- 64 GiB RAM, swap disabled

## Software
- Kernel: `6.8.0-60-generic`
- OS: Ubuntu `24.04`

## Configuration

The machine is configured via [this](https://github.com/rust-lang/simpleinfra/blob/master/ansible/playbooks/rustc-perf.yml) Ansible script.

### HyperThreading & Turboboost
Both are disabled.

### CPU scaling
Scaling governor set to `performance`.

### ASLR (Address Space Layout Randomization)
Disabled with `kernel.randomize_va_space=0` in `sysctl.conf`.

Note that ASLR is also
[disabled explicitly](../collector/src/bin/rustc-fake.rs) when gathering performance metrics.

### NMI watchdog
Disabled with `kernel.nmi_watchdog=0` in `sysctl.conf`.

### Swap
`vm.swappiness` set to `10`. Swap is disabled anyway though.
