//! Process `-Z time-passes` output

use rustc_perf_collector::{Pass, Stat};

use errors::{Result, ResultExt};

pub fn process_output(name: &str, output: Vec<u8>)
    -> Result<(Vec<Pass>, Vec<Stat>)>
{
    let output = String::from_utf8(output)
        .chain_err(|| format!("unable to convert output of {} to UTF-8", name))?;
    let mut passes = Vec::new();
    let mut stats = Vec::new();

    let mut time_indent = None;
    for line in output.lines() {
        if line.trim().starts_with("time: ") {
            let indent = line.find("time:").unwrap();
            if time_indent.is_some() && Some(indent) != time_indent {
                // XXX: sub passes currently cause problems because their names are inconsistent in
                // incremental runs, especially codegen passes
                //let time = &line["  time: ".len()..line.find(";").unwrap()];
                //let time: f64 = time.parse().chain_err(|| format!("parsed time: {:?}", time))?;

                //let mem = &line[line.find("rss: ").unwrap() + 5..line.find("MB").unwrap()];
                //let mem: u64 = mem.parse().chain_err(|| format!("parsed memory: {:?}", mem))?;

                //let name = line[line.find("MB\t").unwrap() + 3..].to_string();
                //sub_passes.insert(name.clone(), Pass {
                //    name: name,
                //    time: time,
                //    mem: mem,
                //    sub_passes: HashMap::new(),
                //});
                continue
            }
            time_indent = Some(indent);

            let time = &line[indent + "time: ".len()..line.find(";").unwrap()];
            let time: f64 = time.parse().chain_err(|| format!("parsed time: {:?}", time))?;

            let mem = &line[line.find("rss: ").unwrap() + 5..line.find("MB").unwrap()];
            let mem: u64 = mem.parse().chain_err(|| format!("parsed memory: {:?}", mem))?;

            let name = line[line.find("MB\t").unwrap() + 3..].to_string();
            passes.push(Pass {
                name: name,
                time: time,
                mem: mem,
            });
        } else {
            extract_perf_stat(line, &mut stats)?;
            info!("unhandled line: {}", line);
        }
    }

    Ok((passes, stats))
}

fn extract_perf_stat(s: &str, stats: &mut Vec<Stat>) -> Result<()> {
    // https://github.com/torvalds/linux/blob/bc78d646e708/tools/perf/Documentation/perf-stat.txt#L281
    macro_rules! get {
        ($e:expr) => (match $e {
            Some(s) => s,
            None => return Ok(()),
        });
    }
    let mut parts = s.split(';').map(|s| s.trim());
    let cnt = get!(parts.next());
    let _unit = get!(parts.next());
    let name = get!(parts.next());
    let _time = get!(parts.next());
    let pct = get!(parts.next());
    if parts.next().is_some() {
        return Ok(())
    }
    if cnt == "<not supported>" {
        return Ok(())
    }
    if !pct.starts_with("100.") {
        panic!("measurement of `{}` only active for {}% of the time", name, pct);
    }
    stats.push(Stat {
        name: name.to_string(),
        cnt: cnt.parse().chain_err(|| {
            format!("failed to parse `{}` as an float", cnt)
        })?,
    });
    Ok(())
}
