//! Process `-Z time-passes` output

use rustc_perf_collector::Stat;

use errors::{Result, ResultExt};

pub fn process_output(name: &str, output: Vec<u8>) -> Result<Vec<Stat>> {
    let output = String::from_utf8(output)
        .chain_err(|| format!("unable to convert output of {} to UTF-8", name))?;
    let mut stats = Vec::new();

    for line in output.lines() {
        // https://github.com/torvalds/linux/blob/bc78d646e708/tools/perf/Documentation/perf-stat.txt#L281
        macro_rules! get {
            ($e:expr) => (match $e {
                Some(s) => s,
                None => {
                    warn!("unhandled line: {}", line);
                    continue;
                },
            });
        }
        let mut parts = line.split(';').map(|s| s.trim());
        let cnt = get!(parts.next());
        let _unit = get!(parts.next());
        let name = get!(parts.next());
        let _time = get!(parts.next());
        let pct = get!(parts.next());
        if cnt == "<not supported>" {
            continue;
        }
        if !pct.starts_with("100.") {
            panic!(
                "measurement of `{}` only active for {}% of the time",
                name,
                pct
            );
        }
        stats.push(Stat {
            name: name.to_string(),
            cnt: cnt.parse()
                .chain_err(|| format!("failed to parse `{}` as an float", cnt))?,
        });
    }

    Ok(stats)
}
