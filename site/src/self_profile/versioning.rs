//! This module wraps `analyzeme` APIs for versions 0.7.* (v7) and 9.0.* (v9):
//! - `analyzeme` structs, with data referenced in the `crox` and `flamegraph` modules
//!   are kept as-is, with an additional inner field versioning the data when needed.
//! - the API referenced in other modules is kept very similar to `analyzeme`
//!   with additions where owning the impls allowed using more methods. The
//!   implementations are delegated to the real `analyzeme` impls, regardless of how trivial
//!   they are, via versioned enums wrapping the original structs.

use anyhow::Context;
use hashbrown::HashMap;
use std::borrow::Cow;
use std::cmp;
use std::fmt;
use std::io::Read;
use std::time::{Duration, SystemTime};

pub enum Pieces {
    V7(PiecesV7),
    V9(PiecesV9),
}

pub struct PiecesV7 {
    pub string_data: Vec<u8>,
    pub string_index: Vec<u8>,
    pub events: Vec<u8>,
}

pub struct PiecesV9 {
    pub data: Vec<u8>,
}

pub struct ProfilingData {
    pub metadata: Metadata,
    inner: VersionedProfilingData,
}

#[derive(Debug)]
pub struct Metadata {
    pub start_time: SystemTime,
    pub process_id: u32,
    pub cmd: String,
}

enum VersionedProfilingData {
    V7(analyzeme_7::ProfilingData),
    V9(analyzeme_9::ProfilingData),
}

#[derive(Clone, Debug)]
pub struct LightweightEvent<'a> {
    pub event_index: usize,
    pub thread_id: u32,
    pub timestamp: Timestamp,
    inner: VersionedLightweightEvent<'a>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Timestamp {
    V7(analyzeme_7::Timestamp),
    V9(analyzeme_9::Timestamp),
}

#[derive(Clone, Debug)]
enum VersionedLightweightEvent<'a> {
    V7(analyzeme_7::LightweightEvent<'a>),
    V9(analyzeme_9::LightweightEvent<'a>),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Event<'a> {
    pub event_kind: Cow<'a, str>,
    pub label: Cow<'a, str>,
    pub additional_data: Vec<Cow<'a, str>>,
    pub timestamp: Timestamp,
    pub thread_id: u32,
}

impl Pieces {
    /// Tries to extract data for `analyzeme` versions 7 or 9 from a tarball:
    /// - if the expected v9 `.mm_profdata` file is found in the tarball, v9 will be the chosen version
    /// - otherwise, we fall back to the v7 3-file format
    pub fn from_tarball<R: std::io::Read>(mut tarball: tar::Archive<R>) -> anyhow::Result<Pieces> {
        let mut pieces_v7 = PiecesV7 {
            string_data: Vec::new(),
            string_index: Vec::new(),
            events: Vec::new(),
        };

        for entry in tarball.entries().context("entries")? {
            let mut entry = entry.context("tarball entry")?;
            let path = entry.path_bytes();
            if *path == *b"self-profile.string_index" {
                entry
                    .read_to_end(&mut pieces_v7.string_index)
                    .context("reading v7 string index")?;
            } else if *path == *b"self-profile.string_data" {
                entry
                    .read_to_end(&mut pieces_v7.string_data)
                    .context("reading v7 string data")?;
            } else if *path == *b"self-profile.events" {
                entry
                    .read_to_end(&mut pieces_v7.events)
                    .context("reading v7 events")?;
            } else if *path == *b"self-profile.mm_profdata" {
                let mut pieces_v9 = PiecesV9 { data: Vec::new() };
                entry
                    .read_to_end(&mut pieces_v9.data)
                    .context("reading v9 data")?;
                return Ok(Pieces::V9(pieces_v9));
            }
        }

        Ok(Pieces::V7(pieces_v7))
    }

    pub fn into_collapsed_stacks(self) -> anyhow::Result<Vec<String>> {
        match self {
            Pieces::V7(pieces) => {
                let profiling_data = analyzeme_7::ProfilingData::from_buffers(
                    pieces.string_data,
                    pieces.string_index,
                    pieces.events,
                )
                .map_err(|e| anyhow::format_err!("{:?}", e))?;

                let recorded_stacks = analyzeme_7::collapse_stacks(&profiling_data)
                    .iter()
                    .map(|(unique_stack, count)| format!("{} {}", unique_stack, count))
                    .collect::<Vec<_>>();
                Ok(recorded_stacks)
            }
            Pieces::V9(pieces) => {
                let profiling_data = analyzeme_9::ProfilingData::from_paged_buffer(pieces.data)
                    .map_err(|e| anyhow::format_err!("{:?}", e))?;

                let recorded_stacks = analyzeme_9::collapse_stacks(&profiling_data)
                    .iter()
                    .map(|(unique_stack, count)| format!("{} {}", unique_stack, count))
                    .collect::<Vec<_>>();
                Ok(recorded_stacks)
            }
        }
    }

    pub fn into_profiling_data(self) -> anyhow::Result<ProfilingData> {
        match self {
            Pieces::V7(pieces) => {
                let profiling_data = analyzeme_7::ProfilingData::from_buffers(
                    pieces.string_data,
                    pieces.string_index,
                    pieces.events,
                )
                .map_err(|e| anyhow::format_err!("{:?}", e))?;
                Ok(profiling_data.into())
            }
            Pieces::V9(pieces) => {
                let profiling_data = analyzeme_9::ProfilingData::from_paged_buffer(pieces.data)
                    .map_err(|e| anyhow::format_err!("{:?}", e))?;
                Ok(profiling_data.into())
            }
        }
    }
}

impl fmt::Debug for Pieces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pieces::V7(pieces) => f
                .debug_struct("Pieces (V7)")
                .field("string_data", &pieces.string_data.len())
                .field("string_index", &pieces.string_index.len())
                .field("events", &pieces.events.len())
                .finish(),
            Pieces::V9(pieces) => f
                .debug_struct("Pieces (V9)")
                .field("data", &pieces.data.len())
                .finish(),
        }
    }
}

impl ProfilingData {
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = LightweightEvent<'a>> + 'a> {
        match &self.inner {
            VersionedProfilingData::V7(data) => Box::new(data.iter().map(|e| e.into())),
            VersionedProfilingData::V9(data) => Box::new(data.iter().map(|e| e.into())),
        }
    }
}

impl From<analyzeme_7::ProfilingData> for ProfilingData {
    fn from(data: analyzeme_7::ProfilingData) -> Self {
        ProfilingData {
            metadata: Metadata {
                start_time: data.metadata.start_time,
                process_id: data.metadata.process_id,
                cmd: data.metadata.cmd.clone(),
            },
            inner: VersionedProfilingData::V7(data),
        }
    }
}

impl From<analyzeme_9::ProfilingData> for ProfilingData {
    fn from(data: analyzeme_9::ProfilingData) -> Self {
        ProfilingData {
            metadata: Metadata {
                start_time: data.metadata.start_time,
                process_id: data.metadata.process_id,
                cmd: data.metadata.cmd.clone(),
            },
            inner: VersionedProfilingData::V9(data),
        }
    }
}

impl<'a> LightweightEvent<'a> {
    pub fn duration(&self) -> Option<Duration> {
        match &self.inner {
            VersionedLightweightEvent::V7(lightweight_event) => lightweight_event.duration(),
            VersionedLightweightEvent::V9(lightweight_event) => lightweight_event.duration(),
        }
    }

    pub fn to_event(&self) -> Event<'a> {
        match &self.inner {
            VersionedLightweightEvent::V7(lightweight_event) => lightweight_event.to_event().into(),
            VersionedLightweightEvent::V9(lightweight_event) => lightweight_event.to_event().into(),
        }
    }
}

impl<'a> Event<'a> {
    pub fn get_args(&self) -> Option<HashMap<String, String>> {
        if !self.additional_data.is_empty() {
            Some(
                self.additional_data
                    .iter()
                    .enumerate()
                    .map(|(i, arg)| (format!("arg{}", i).to_string(), arg.to_string()))
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl Timestamp {
    pub fn is_instant(&self) -> bool {
        match *self {
            Timestamp::V7(timestamp) => timestamp.is_instant(),
            Timestamp::V9(timestamp) => timestamp.is_instant(),
        }
    }

    pub fn start(&self) -> SystemTime {
        match *self {
            Timestamp::V7(timestamp) => timestamp.start(),
            Timestamp::V9(timestamp) => timestamp.start(),
        }
    }

    pub fn to_min_max(self) -> (SystemTime, SystemTime) {
        match self {
            Timestamp::V7(analyzeme_7::Timestamp::Instant(t))
            | Timestamp::V9(analyzeme_9::Timestamp::Instant(t)) => (t, t),
            Timestamp::V7(analyzeme_7::Timestamp::Interval { start, end })
            | Timestamp::V9(analyzeme_9::Timestamp::Interval { start, end }) => {
                // Usually start should always be greater than end, but let's not
                // choke on invalid data here.
                (cmp::min(start, end), cmp::max(start, end))
            }
        }
    }
}

impl<'a> From<analyzeme_7::LightweightEvent<'a>> for LightweightEvent<'a> {
    fn from(lightweight_event: analyzeme_7::LightweightEvent<'a>) -> Self {
        LightweightEvent {
            event_index: lightweight_event.event_index,
            thread_id: lightweight_event.thread_id,
            timestamp: lightweight_event.timestamp.into(),
            inner: VersionedLightweightEvent::V7(lightweight_event),
        }
    }
}

impl<'a> From<analyzeme_9::LightweightEvent<'a>> for LightweightEvent<'a> {
    fn from(lightweight_event: analyzeme_9::LightweightEvent<'a>) -> Self {
        LightweightEvent {
            event_index: lightweight_event.event_index,
            thread_id: lightweight_event.thread_id,
            timestamp: lightweight_event.timestamp.into(),
            inner: VersionedLightweightEvent::V9(lightweight_event),
        }
    }
}

impl<'a> From<analyzeme_7::Event<'a>> for Event<'a> {
    fn from(event: analyzeme_7::Event<'a>) -> Self {
        Event {
            additional_data: event.additional_data,
            event_kind: event.event_kind,
            label: event.label,
            timestamp: event.timestamp.into(),
            thread_id: event.thread_id,
        }
    }
}

impl<'a> From<analyzeme_9::Event<'a>> for Event<'a> {
    fn from(event: analyzeme_9::Event<'a>) -> Self {
        Event {
            additional_data: event.additional_data,
            event_kind: event.event_kind,
            label: event.label,
            timestamp: event.timestamp.into(),
            thread_id: event.thread_id,
        }
    }
}

impl From<analyzeme_7::Timestamp> for Timestamp {
    fn from(timestamp: analyzeme_7::Timestamp) -> Self {
        Timestamp::V7(timestamp)
    }
}

impl From<analyzeme_9::Timestamp> for Timestamp {
    fn from(timestamp: analyzeme_9::Timestamp) -> Self {
        Timestamp::V9(timestamp)
    }
}
