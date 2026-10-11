use std::hint::black_box as bb;
use std::rc::Rc;
use std::rc::Weak as WeakRc;
use std::sync::Arc;
use std::sync::Weak as WeakArc;

use benchlib::benchmark::run_benchmark_group;

fn main() {
    let count = 500_000;
    const BUF: &[u8] = [0u8; 1024].as_slice();

    run_benchmark_group(|group| {
        // Crude benchmark for calculating time to compute the layout. In practice this is mostly
        // checking allocator speed but should still be sensitive to layout.
        //
        // Slices are tested as well since they use a somewhat different algorithm.
        group.register_benchmark("rc_alloc", || {
            let mut v = Vec::with_capacity(count);
            move || {
                for i in 0..count {
                    v.push(Rc::new(i));
                }
                v // drop the vec outside of the measurement interval
            }
        });
        group.register_benchmark("arc_alloc", || {
            let mut v = Vec::with_capacity(count);
            move || {
                for i in 0..count {
                    v.push(Arc::new(i));
                }
                v // drop the vec outside of the measurement interval
            }
        });
        group.register_benchmark("rc_alloc_slice", || {
            let mut v: Vec<Rc<[u8]>> = Vec::with_capacity(count);
            move || {
                for _ in 0..count {
                    v.push(Rc::from(bb(BUF)));
                }
                v // drop the vec outside of the measurement interval
            }
        });
        group.register_benchmark("arc_alloc_slice", || {
            let mut v: Vec<Arc<[u8]>> = Vec::with_capacity(count);
            move || {
                for _ in 0..count {
                    v.push(Arc::from(bb(BUF)));
                }
                v // drop the vec outside of the measurement interval
            }
        });

        // Time to drop completely. Decrements are in a separate test.
        //
        // Similar to the allocation tests this is mostly testing the allocator, but should give us
        // some insight into the data we want.
        group.register_benchmark("rc_drop", || {
            let mut v = Vec::with_capacity(count);
            for i in 0..count {
                v.push(Rc::new(i));
            }

            move || {
                v.clear();
                v // drop the vec outside of the measurement interval
            }
        });
        group.register_benchmark("arc_drop", || {
            let mut v = Vec::with_capacity(count);
            for i in 0..count {
                v.push(Arc::new(i));
            }

            move || {
                v.clear();
                v // drop the vec outside of the measurement interval
            }
        });

        // Deref is the most common operation and currently involves a small computation, but this
        // may become zero-cost. Attempt to capture that.
        group.register_benchmark("rc_deref", || {
            let rc = Rc::new(bb(u128::MAX));
            move || {
                let rc_ref: &Rc<u128> = &rc;
                for _ in 0..count {
                    let _x: &u128 = bb(rc_ref);
                }
                rc // drop the rc outside of the measurement interval
            }
        });
        group.register_benchmark("arc_deref", || {
            let rc = Arc::new(bb(u128::MAX));
            move || {
                let rc_ref: &Arc<u128> = &rc;
                for _ in 0..count {
                    let _x: &u128 = bb(rc_ref);
                }
                rc // drop the rc outside of the measurement interval
            }
        });
        group.register_benchmark("rc_deref_slice", || {
            let rc: Rc<[u8]> = Rc::from(BUF);
            move || {
                let rc_ref: &Rc<[u8]> = &rc;
                for _ in 0..count {
                    let _x: &[u8] = bb(rc_ref);
                }
                rc // drop the rc outside of the measurement interval
            }
        });
        group.register_benchmark("arc_deref_slice", || {
            let rc: Arc<[u8]> = Arc::from(BUF);
            move || {
                let rc_ref: &Arc<[u8]> = &rc;
                for _ in 0..count {
                    let _x: &[u8] = bb(rc_ref);
                }
                rc // drop the rc outside of the measurement interval
            }
        });

        // Time to clone
        group.register_benchmark("rc_increment", || {
            let mut v: Vec<Rc<u128>> = Vec::with_capacity(count);
            let orig_rc = Rc::new(bb(u128::MAX));
            move || {
                for _ in 0..count {
                    v.push(Rc::clone(bb(&orig_rc)));
                }
                // drop the original rc and its clones outside of the measurement interval
                (orig_rc, v)
            }
        });
        group.register_benchmark("arc_increment", || {
            let mut v: Vec<Arc<u128>> = Vec::with_capacity(count);
            let orig_rc = Arc::new(bb(u128::MAX));
            move || {
                for _ in 0..count {
                    v.push(Arc::clone(bb(&orig_rc)));
                }
                // drop the original rc and its clones outside of the measurement interval
                (orig_rc, v)
            }
        });

        // Time to decrement without fully dropping
        group.register_benchmark("rc_decrement", || {
            let mut v: Vec<Rc<u128>> = Vec::with_capacity(count);
            let orig_rc = Rc::new(bb(u128::MAX));
            for _ in 0..count {
                v.push(Rc::clone(bb(&orig_rc)));
            }

            move || {
                v.clear();
                // drop the original rc and the vec itself outside of the measurement interval
                (orig_rc, v)
            }
        });
        group.register_benchmark("arc_decrement", || {
            let mut v: Vec<Arc<u128>> = Vec::with_capacity(count);
            let orig_rc = Arc::new(bb(u128::MAX));
            for _ in 0..count {
                v.push(Arc::clone(bb(&orig_rc)));
            }

            move || {
                v.clear();
                // drop the original rc and the vec itself outside of the measurement interval
                (orig_rc, v)
            }
        });

        // Time for a downgrade-upgrade cycle
        group.register_benchmark("rc_downgrade_upgrade", || {
            let mut v_weak: Vec<WeakRc<u128>> = Vec::with_capacity(count);
            let mut v_rc: Vec<Rc<u128>> = Vec::with_capacity(count);
            let rc = Rc::new(bb(u128::MAX));
            move || {
                for _ in 0..count {
                    v_weak.push(Rc::downgrade(bb(&rc)));
                }
                for weak in &v_weak {
                    v_rc.push(weak.upgrade().unwrap());
                }
                (rc, v_weak, v_rc)
            }
        });
        group.register_benchmark("arc_downgrade_upgrade", || {
            let mut v_weak: Vec<WeakArc<u128>> = Vec::with_capacity(count);
            let mut v_rc: Vec<Arc<u128>> = Vec::with_capacity(count);
            let rc = Arc::new(bb(u128::MAX));
            move || {
                for _ in 0..count {
                    v_weak.push(Arc::downgrade(bb(&rc)));
                }
                for weak in &v_weak {
                    v_rc.push(weak.upgrade().unwrap());
                }
                (rc, v_weak, v_rc)
            }
        });
    });
}
