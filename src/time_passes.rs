//! Process `-Z time-passes` output

use rustc_perf_collector::Pass;

use errors::{Result, ResultExt};

pub fn process_output(name: &str, output: Vec<u8>) -> Result<Vec<Pass>> {
    let output = String::from_utf8(output)
        .chain_err(|| format!("unable to convert output of {} to UTF-8", name))?;
    let mut passes = Vec::new();

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
            //info!("unhandled line: {}", line);
        }
    }

    Ok(passes)
}

pub struct PassAverager {
    pub state: Vec<Pass>,
    runs: u64
}

impl PassAverager {
    pub fn new(state: Vec<Pass>) -> Self {
        Self {
            state: state,
            runs: 0,
        }
    }

    pub fn average_with(
        &mut self,
        b: Vec<Pass>,
    ) -> Result<()> {
        self.runs += 1;

        for a in &mut self.state {
            let b = match b.iter().find(|p| p.name == a.name) {
                Some(b) => b,
                None => bail!("expected name {} to exist in both a and b", a.name),
            };
            a.time = a.time + ((b.time - a.time) / (self.runs as f64));
            a.mem = a.mem + ((b.mem - b.mem) / self.runs);
        }

        Ok(())
    }
}
