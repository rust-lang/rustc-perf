use std::cmp::{min,max};

use util::best;
use ::{Stats,Num,Qty};

type Best = best::Best<Num, Vec<Qty>, best::Largest>;

trait CookieMonitor {
    fn judge(&self, after: usize, left: Qty, acc: &Stats) -> bool;
}

struct Whatever;
impl CookieMonitor for Whatever {
    fn judge(&self, _after: usize, _left: Qty, _acc: &Stats) -> bool {
        true
    }
}

struct CalorieCounter {
    mincals: Vec<Num>,
    maxcals: Vec<Num>,
    target: Num,
}
impl CalorieCounter {
    fn new(stats: &[Stats], target: Num) -> CalorieCounter {
        let c0 = stats[0].calories;
        let (imin, imax) = stats.iter().scan((c0, c0), |state, stat| {
            let (lmin, lmax) = *state;
            *state = (min(lmin, stat.calories), max(lmax, stat.calories));
            Some(*state)
        }).unzip();
        CalorieCounter {
            mincals: imin,
            maxcals: imax,
            target: target
        }
    }
}
impl CookieMonitor for CalorieCounter {
    fn judge(&self, after: usize, left: Qty, acc: &Stats) -> bool {
        let left = Num::from(left);
        let rest = self.target - acc.calories;
        rest >= self.mincals[after] * left && rest <= self.maxcals[after] * left
    }
}

struct ExhCtx<'s> {
    stats: &'s [Stats],
    qtys: Vec<Qty>,
    best: Best,
}

fn exh_recur<M: CookieMonitor>(ctx: &mut ExhCtx, mon: &M, i: usize, left: Qty, acc: Stats) {
    if !mon.judge(i, left, &acc) {
        return;
    }
    if i == 0 {
        ctx.qtys[i] = left;
        ctx.best.add((acc + ctx.stats[i].clone() * left).eval(), &ctx.qtys);
    } else {
        for this in 0..(left+1) {
            ctx.qtys[i] = this;
            exh_recur(ctx, mon, i - 1, left - this, acc.clone() + ctx.stats[i].clone() * this);
        }
    }
}

pub fn exhaustive(stats: &[Stats], total: Qty, opt_cals: Option<Num>) -> (Num, Vec<Qty>) {
    assert!(stats.len() >= 1);
    let mut ctx = ExhCtx {
        stats: stats,
        qtys: vec![!0; stats.len()],
        best: Best::new(best::Largest),
    };
    let n1 = stats.len() - 1;
    match opt_cals {
        None => exh_recur(&mut ctx, &Whatever, n1, total, Stats::zero()),
        Some(cals) => {
            let mon = CalorieCounter::new(stats, cals);
            exh_recur(&mut ctx, &mon, n1, total, Stats::zero())
        },
    };
    ctx.best.unwrap()
}

// Conjecture: this problem is amenable to hill-climbing.  ...without
// the part 2 addendum, at least.  Probably still is even with it,
// with a little more subtlety, since everything except the final
// metric is linear.

#[cfg(test)]
mod tests {
    use super::exhaustive;
    use ::{Stats};

    #[test]
    fn example1() {
        let bt = Stats { capacity: -1, durability: -2, flavor: 6, texture: 3, calories: 8 };
        let cn = Stats { capacity: 2, durability: 3, flavor: -2, texture: -1, calories: 3 };
        assert_eq!(exhaustive(&[bt, cn], 100, None), (62842880, vec![44, 56]));
    }

    #[test]
    fn example2() {
        let bt = Stats { capacity: -1, durability: -2, flavor: 6, texture: 3, calories: 8 };
        let cn = Stats { capacity: 2, durability: 3, flavor: -2, texture: -1, calories: 3 };
        assert_eq!(exhaustive(&[bt, cn], 100, Some(500)), (57600000, vec![40, 60]));
    }
}
