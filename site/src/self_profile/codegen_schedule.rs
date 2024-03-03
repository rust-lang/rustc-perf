use analyzeme::ProfilingData;
use anyhow::Context;
use hashbrown::HashMap;
use serde::Serializer;
use std::time::Duration;

#[derive(serde::Deserialize, Debug)]
pub struct Opt {
    #[serde(default)]
    force_width: Option<String>,
}

fn is_interesting(name: &str) -> bool {
    matches!(
        name,
        "codegen_module" | "codegen_module_optimize" | "codegen_module_perform_lto"
    )
}

fn by_thread(self_profile_data: Vec<u8>) -> anyhow::Result<(u64, HashMap<u32, Vec<Event>>)> {
    let data = ProfilingData::from_paged_buffer(self_profile_data, None)
        .map_err(|e| anyhow::format_err!("{:?}", e))?;

    let mut start = None;
    for event in data
        .iter()
        .filter(|e| e.timestamp().map_or(false, |t| !t.is_instant()))
    {
        let full_event = data.to_full_event(&event);
        if is_interesting(&full_event.label) {
            if start.is_some() {
                start = std::cmp::min(start, Some(event.timestamp().unwrap().start()));
            } else {
                start = Some(event.timestamp().unwrap().start());
            }
        }
    }
    let start = start.ok_or(anyhow::format_err!(
        "no interesting events. does this profile run codegen?"
    ))?;

    let mut end = start;
    let mut by_thread = HashMap::new();
    for event in data
        .iter()
        .filter(|e| e.timestamp().map_or(false, |t| !t.is_instant()))
    {
        let full_event = data.to_full_event(&event);

        if is_interesting(&full_event.label) {
            by_thread
                .entry(event.thread_id)
                .or_insert_with(Vec::new)
                .push(Event {
                    name: full_event.label.into(),
                    start: event
                        .timestamp()
                        .unwrap()
                        .start()
                        .duration_since(start)
                        .unwrap(),
                    end: event
                        .timestamp()
                        .unwrap()
                        .end()
                        .duration_since(start)
                        .unwrap(),
                });
            end = std::cmp::max(end, event.timestamp().unwrap().end());
        }
    }

    Ok((
        end.duration_since(start).unwrap().as_millis() as u64,
        by_thread,
    ))
}

pub fn generate(
    title: &str,
    self_profile_base_data: Option<Vec<u8>>,
    self_profile_data: Vec<u8>,
    opt: Opt,
) -> anyhow::Result<Vec<u8>> {
    let (total_duration_new, by_thread_new) = by_thread(self_profile_data)?;
    let mut total_duration = total_duration_new;
    let prev = if let Some(self_profile_base_data) = self_profile_base_data {
        let (total_duration_prev, by_thread_prev) = by_thread(self_profile_base_data)?;
        total_duration = std::cmp::max(total_duration_new, total_duration_prev);
        Some((total_duration_prev, by_thread_prev))
    } else {
        None
    };

    if let Some(force_width) = opt.force_width {
        let forced = force_width
            .parse::<u64>()
            .context(anyhow::format_err!("{} does not parse", force_width))?;
        total_duration = std::cmp::max(forced, total_duration);
    }

    Ok(HTML_TEMPLATE
        .replace("{{title}}", title)
        .replace("{{TOTAL_DURATION_BOTH}}", &total_duration.to_string())
        .replace(
            "{{TOTAL_DURATION_BASE}}",
            &prev
                .as_ref()
                .map(|p| p.0.to_string())
                .unwrap_or_else(|| String::from("null")),
        )
        .replace("{{TOTAL_DURATION_NEW}}", &total_duration_new.to_string())
        .replace(
            "{{BY_THREAD_BASE}}",
            &serde_json::to_string(&prev.map(|p| p.1)).unwrap(),
        )
        .replace(
            "{{BY_THREAD}}",
            &serde_json::to_string(&by_thread_new).unwrap(),
        )
        .into_bytes())
}

#[derive(serde::Serialize, Debug)]
pub struct Event {
    pub name: String,
    #[serde(serialize_with = "as_millis")]
    pub start: std::time::Duration,
    #[serde(serialize_with = "as_millis")]
    pub end: std::time::Duration,
}

fn as_millis<S: Serializer>(d: &Duration, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u64(d.as_millis().try_into().unwrap())
}

static HTML_TEMPLATE: &str = include_str!("codegen_schedule.html");
