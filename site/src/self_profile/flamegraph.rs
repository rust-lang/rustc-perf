use analyzeme::{collapse_stacks, ProfilingData};
use anyhow::Context;
use inferno::flamegraph::{from_lines, Options as FlamegraphOptions};

#[derive(serde::Deserialize, Debug)]
pub struct Opt {}

pub fn generate(title: &str, profiling_data: ProfilingData, _: Opt) -> anyhow::Result<Vec<u8>> {
    let recorded_stacks = collapse_stacks(&profiling_data)
        .iter()
        .map(|(unique_stack, count)| format!("{unique_stack} {count}"))
        .collect::<Vec<_>>();

    let mut file = Vec::new();
    let mut flamegraph_options = FlamegraphOptions::default();
    flamegraph_options.count_name = "nanoseconds".to_owned();
    flamegraph_options.title = title.to_owned();
    flamegraph_options.min_width = 0.0;

    from_lines(
        &mut flamegraph_options,
        recorded_stacks.iter().map(|s| s.as_ref()),
        &mut file,
    )
    .context("unable to generate a flamegraph from the collapsed stack data")?;
    Ok(file)
}
