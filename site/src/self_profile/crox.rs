//! Imported from measureme (rust-lang/measureme:crox/src/main.rs), and modified for use as a
//! library.

use hashbrown::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use analyzeme::{ProfilingData, Timestamp};

use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use serde_json::json;
use std::cmp;

fn as_micros<S: Serializer>(d: &Duration, s: S) -> Result<S::Ok, S::Error> {
    let v = (d.as_secs() * 1_000_000) + (d.subsec_nanos() as u64 / 1_000);
    s.serialize_u64(v)
}

#[derive(Clone, Copy, Eq, PartialEq, Serialize)]
enum EventType {
    #[serde(rename = "X")]
    Complete,
}

#[derive(Serialize)]
struct Event {
    name: String,
    #[serde(rename = "cat")]
    category: String,
    #[serde(rename = "ph")]
    event_type: EventType,
    #[serde(rename = "ts", serialize_with = "as_micros")]
    timestamp: Duration,
    #[serde(rename = "dur", serialize_with = "as_micros")]
    duration: Duration,
    #[serde(rename = "pid")]
    process_id: u32,
    #[serde(rename = "tid")]
    thread_id: u32,
    args: Option<HashMap<String, String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Opt {
    /// collapse threads without overlapping events
    #[serde(default)]
    pub collapse_threads: bool,
    /// filter out events with shorter duration (in microseconds)
    #[serde(default)]
    pub minimum_duration: Option<u128>,
}

// generate mapping from thread_id to collapsed thread_id or an empty map
fn generate_thread_to_collapsed_thread_mapping(
    collapse_threads: bool,
    data: &ProfilingData,
) -> HashMap<u32, u32> {
    let mut thread_to_collapsed_thread: HashMap<u32, u32> = HashMap::default();

    if collapse_threads {
        // collect start and end times for all threads
        let mut thread_start_and_end: HashMap<u32, (SystemTime, SystemTime)> = HashMap::default();
        for event in data.iter() {
            let timestamp = if let Some(t) = event.timestamp() {
                t
            } else {
                continue;
            };
            thread_start_and_end
                .entry(event.thread_id)
                .and_modify(|(thread_start, thread_end)| {
                    let (event_min, event_max) = timestamp_to_min_max(timestamp);
                    *thread_start = cmp::min(*thread_start, event_min);
                    *thread_end = cmp::max(*thread_end, event_max);
                })
                .or_insert_with(|| timestamp_to_min_max(timestamp));
        }
        // collect the the threads in order of the end time
        let mut end_and_thread = thread_start_and_end
            .iter()
            .map(|(&thread_id, &(_start, end))| (end, thread_id))
            .collect::<Vec<_>>();

        end_and_thread.sort_unstable_by_key(|&(end, _thread_id)| end);
        let mut next_end_iter = end_and_thread.iter().peekable();

        // collect the the threads in order of the start time
        let mut start_and_thread = thread_start_and_end
            .iter()
            .map(|(&thread_id, &(start, _end))| (start, thread_id))
            .collect::<Vec<_>>();

        start_and_thread.sort_unstable_by_key(|&(start, _thread_id)| start);

        let mut current_thread_id = 0; // use new thread_ids to avoid strange gaps in the numbers
        for &(start, thread_id) in start_and_thread.iter() {
            // safe to unwrap due to end_and_thread and start_and_thread have the same length
            let (next_end, next_thread_id) = next_end_iter.peek().unwrap();
            if start > *next_end {
                next_end_iter.next();
                // need to lookup the thread_id due to new and collapsed threads
                let mapped_thread_id = *thread_to_collapsed_thread
                    .get(next_thread_id)
                    .unwrap_or(next_thread_id);

                thread_to_collapsed_thread.insert(thread_id, mapped_thread_id);
            } else {
                thread_to_collapsed_thread.insert(thread_id, current_thread_id);
                current_thread_id += 1;
            }
        }
    }
    thread_to_collapsed_thread
}

fn get_args(full_event: &analyzeme::Event) -> Option<HashMap<String, String>> {
    if !full_event.additional_data.is_empty() {
        Some(
            full_event
                .additional_data
                .iter()
                .enumerate()
                .map(|(i, arg)| (format!("arg{}", i), arg.to_string()))
                .collect(),
        )
    } else {
        None
    }
}

/// Returns JSON blob fit, `chrome_profiler.json`.
pub fn generate(self_profile_data: Vec<u8>, opt: Opt) -> anyhow::Result<Vec<u8>> {
    let mut serializer = serde_json::Serializer::new(Vec::new());

    let mut seq = serializer.serialize_seq(None)?;

    let data = ProfilingData::from_paged_buffer(self_profile_data, None)
        .map_err(|e| anyhow::format_err!("{:?}", e))?;

    let thread_to_collapsed_thread =
        generate_thread_to_collapsed_thread_mapping(opt.collapse_threads, &data);

    // Chrome does not seem to like how many QueryCacheHit events we generate
    // only handle Interval events for now
    for event in data
        .iter()
        .filter(|e| e.timestamp().map_or(false, |t| !t.is_instant()))
    {
        let duration = event.duration().unwrap();
        if let Some(minimum_duration) = opt.minimum_duration {
            if duration.as_micros() < minimum_duration {
                continue;
            }
        }
        let full_event = data.to_full_event(&event);
        let crox_event = Event {
            name: full_event.label.clone().into_owned(),
            category: full_event.event_kind.clone().into_owned(),
            event_type: EventType::Complete,
            timestamp: event
                .timestamp()
                .unwrap()
                .start()
                .duration_since(UNIX_EPOCH)
                .unwrap(),
            duration,
            process_id: data.metadata().process_id,
            thread_id: *thread_to_collapsed_thread
                .get(&event.thread_id)
                .unwrap_or(&event.thread_id),
            args: get_args(&full_event),
        };
        seq.serialize_element(&crox_event)?;
    }
    // add crate name for the process_id
    let index_of_crate_name = data
        .metadata()
        .cmd
        .find(" --crate-name ")
        .map(|index| index + 14);
    if let Some(index) = index_of_crate_name {
        let (_, last) = data.metadata().cmd.split_at(index);
        let (crate_name, _) = last.split_at(last.find(' ').unwrap_or(last.len()));

        let process_name = json!({
            "name": "process_name",
            "ph" : "M",
            "ts" : 0,
            "tid" : 0,
            "cat" : "",
            "pid" : data.metadata().process_id,
            "args": {
                "name" : crate_name
            }
        });
        seq.serialize_element(&process_name)?;
    }
    // sort the processes after start time
    let process_name = json!({
        "name": "process_sort_index",
        "ph" : "M",
        "ts" : 0,
        "tid" : 0,
        "cat" : "",
        "pid" : data.metadata().process_id,
        "args": {
            "sort_index" : data.metadata().start_time.duration_since(UNIX_EPOCH).unwrap().as_micros() as u64
        }
    });
    seq.serialize_element(&process_name)?;

    seq.end()?;

    Ok(serializer.into_inner())
}

fn timestamp_to_min_max(timestamp: Timestamp) -> (SystemTime, SystemTime) {
    match timestamp {
        Timestamp::Instant(t) => (t, t),
        Timestamp::Interval { start, end } => {
            // Usually start should always be greater than end, but let's not
            // choke on invalid data here.
            (cmp::min(start, end), cmp::max(start, end))
        }
    }
}
