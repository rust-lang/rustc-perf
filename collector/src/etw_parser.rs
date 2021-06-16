use std::{collections::HashMap, io::BufRead};

/*
File format looks like this:

```
BeginHeader
{Event Name}, {Column0}, {Column1}, ...
EndHeader
OS Version: {Windows version}, Trace Size: {size}KB, Events Lost: {events lost}, Buffers Lost: {buffers lost}, Trace Start: {trace start}, Trace Length: {time} sec, PointerSize: 8, Trace Name: pmc_counters_merged.etl
FirstReliableEventTimeStamp, {time stamp}
FirstReliableCSwitchEventTimeStamp, {time stamp}
{Event Name}, {Column0 Value}, {Column1 Value}, ...
```

Some events we care about specifically:

```
P-Start,     104743,        rustc.exe (10612),        480,          1, 0x0000938192a10300, 0x000000049f5be000, 0x00000000, S-1-12-1-2346571520-1185420729-3708355771-3596251678, "\\?\D:\code\rust\build\x86_64-pc-windows-msvc\stage1\bin\rustc.exe" --crate-name regex src\lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=3e524b9e4d4e3569 -C extra-filename=-3e524b9e4d4e3569 --out-dir C:\Users\WESLEY~1\AppData\Local\Temp\.tmpeAXOco\target\debug\deps -L dependency=C:\Users\WESLEY~1\AppData\Local\Temp\.tmpeAXOco\target\debug\deps --extern aho_corasick=C:\Users\WESLEY~1\AppData\Local\Temp\.tmp, <none>, <none>
T-Start,     106028,        rustc.exe (10612),      15340, 0xfffffe8891310000, 0xfffffe8891309000, 0x000000a73d6ff000, 0x000000a73d700000, 0x000000a73d503000,    0x00000000,       8,       5,     2,  0x00,        rustc.exe!0x00007ff73b6112f4,  "Unknown"
Pmc,     106082,      15340, 3184489, 3416818
CSwitch,     106082,        rustc.exe (10612),      15340,    8,   -1,          55,       55,             Idle (   0),          0,    0,   -1,         Running,        Executive,  NonSwap,   2216,   0,   0,          0,    0,    1,   Important,   Important
T-End,    1342679,        rustc.exe (10612),      14800, 0xfffffe8893530000, 0xfffffe8893529000, 0x000000a73e3ff000, 0x000000a73e400000, 0x000000a73d50d000,    0x00000000,       8,       5,     2,  0x00, std-7ee3052c0f4158a9.dll!0x00007ffb573a2560,  "Unknown"
P-End,    1359642,        rustc.exe (10612),        480,          1, 0x0000938192a10300, 0x00000000, 0x000000049f5be000, 0x00000000, S-1-12-1-2346571520-1185420729-3708355771-3596251678, "\\?\D:\code\rust\build\x86_64-pc-windows-msvc\stage1\bin\rustc.exe" --crate-name regex src\lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=3e524b9e4d4e3569 -C extra-filename=-3e524b9e4d4e3569 --out-dir C:\Users\WESLEY~1\AppData\Local\Temp\.tmpeAXOco\target\debug\deps -L dependency=C:\Users\WESLEY~1\AppData\Local\Temp\.tmpeAXOco\target\debug\deps --extern aho_corasick=C:\Users\WESLEY~1\AppData\Local\Temp\.tmp, <none>, <none>
```

Pmc events increase monotonically per CPU (core) and are cumulative. Their data is associated
with the following CSwitch event (they will have the same timestamp). We need to keep track of
when the rustc process has been scheduled onto a CPU (CSwitch to rustc.exe) and when it has been
scheduled off of that same CPU and then calculate the difference of the PMC and add it to the
running total.

Algorithm:
- Read in the header and determine which columns correspond to the values we care about
- Validate the OS Version line does not have lost events or buffers
- Locate all the events for the rustc.exe process (probably validate there is only one present) and parse them into useful data structures
- Do basic sanity checking
    - Validate we have 1 P-Start & 1 P-End at beginning and end of timeline
    - Validate every Pmc has a next CSwitch event at the same timestamp
- It may be convenient at this point to rewrite into a stream of (pmc data, CSwitch data)
- Count up deltas of PMC data on each CPU -> this is the total for the counter
- Return results

TODO: We also need to track metrics for processes spawned by the rustc process such as a linker.
*/

#[derive(Debug, Eq, PartialEq)]
struct EventHeader {
    name: String,
    columns: Vec<String>,
}

const PROCESS_START: &str = "P-Start";
const PROCESS_END: &str = "P-End";
const PMC: &str = "Pmc";
const CSWITCH: &str = "CSwitch";
const TIMESTAMP: &str = "TimeStamp";
const PROCESS_NAME_PID: &str = "Process Name ( PID)";
const THREAD_ID: &str = "ThreadID";
const INSTRUCTION_RETIRED: &str = "InstructionRetired";
const TOTAL_CYCLES: &str = "TotalCycles";
const NEW_PROCESS_NAME_PID: &str = "New Process Name ( PID)";
const OLD_PROCESS_NAME_PID: &str = "Old Process Name ( PID)";
const CPU: &str = "CPU";

fn parse_header(r: &mut dyn BufRead) -> anyhow::Result<Vec<EventHeader>> {
    let mut line = String::new();
    r.read_line(&mut line)?;

    anyhow::ensure!(line.trim() == "BeginHeader", "BeginHeader line not found at beginning of file");

    let mut headers = Vec::with_capacity(6);

    while { line.clear(); r.read_line(&mut line)? != 0 } {
        let l = line.trim();
        if l == "EndHeader" {
            break;
        }

        let mut contents = l.split(',');
        let name = contents.next().expect("event header was missing name").trim();

        if name == PROCESS_START || name == PROCESS_END || name == PMC || name == CSWITCH {
            headers.push(EventHeader {
                name: name.to_string(),
                columns: contents.map(|s| s.trim().to_string()).collect(),
            });
        }
    }

    anyhow::ensure!(line.trim() == "EndHeader", "EndHeader line not found");

    Ok(headers)
}

fn validate_os_header_line(r: &mut dyn BufRead) -> anyhow::Result<()> {
    let mut line = String::new();
    r.read_line(&mut line)?;

    anyhow::ensure!(line.starts_with("OS Version"), "OS version line not found");

    let components: Vec<_> = line.split(',').collect();

    let get_value = |key: &str| {
        components
            .iter()
            .find(|c| c.trim_start().starts_with(key))
            .expect("key not found")
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
    };

    let events_lost: u32 = get_value("Events Lost:").parse()?;
    anyhow::ensure!(events_lost == 0, "events were lost");

    let buffers_lost: u32 = get_value("Buffers lost:").parse()?;
    anyhow::ensure!(buffers_lost == 0, "buffers were lost");

    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct ProcessStart {
    process_name: String,
    timestamp: u64,
}

#[derive(Debug, Eq, PartialEq)]
struct ProcessEnd {
    process_name: String,
    timestamp: u64,
}

#[derive(Debug, Eq, PartialEq)]
struct Pmc {
    timestamp: u64,
    thread_id: u64,
    instructions_retired: u64,
    total_cycles: u64,
}

#[derive(Debug, Eq, PartialEq)]
struct CSwitch {
    timestamp: u64,
    old_process: String,
    new_process: String,
    cpu: u8,
}

#[derive(Debug, Eq, PartialEq)]
enum Event {
    ProcessStart(ProcessStart),
    ProcessEnd(ProcessEnd),
    Pmc(Pmc),
    CSwitch(CSwitch),
}

impl Event {
    fn timestamp(&self) -> u64 {
        match self {
            Event::ProcessStart(info) => info.timestamp,
            Event::ProcessEnd(info) => info.timestamp,
            Event::Pmc(info) => info.timestamp,
            Event::CSwitch(info) => info.timestamp,
        }
    }
}

fn parse_events(r: &mut dyn BufRead, headers: Vec<EventHeader>) -> anyhow::Result<Vec<Event>> {
    let find_header = |name: &str| {
        headers.iter().find(|h| h.name == name).expect("could not find header")
    };

    let find_column = |header: &EventHeader, name: &str| {
        header.columns.iter().position(|c| c == name).expect(&format!("failed to find column {}", name)) + 1
    };

    let (pstart_timestamp, pstart_process_name) = {
        let pstart = find_header(PROCESS_START);
        (find_column(pstart, TIMESTAMP), find_column(pstart, PROCESS_NAME_PID))
    };

    let (pend_timestamp, pend_process_name) = {
        let pend = find_header(PROCESS_END);
        (find_column(pend, TIMESTAMP), find_column(pend, PROCESS_NAME_PID))
    };

    let (pmc_timestamp, pmc_thread_id, pmc_instructions_retired, pmc_total_cycles) = {
        let pmc = find_header(PMC);
        (find_column(pmc, TIMESTAMP), find_column(pmc, THREAD_ID), find_column(pmc, INSTRUCTION_RETIRED), find_column(pmc, TOTAL_CYCLES))
    };

    let (cswitch_timestamp, cswitch_old_process, cswitch_new_process, cswitch_cpu) = {
        let cswitch = find_header(CSWITCH);
        (find_column(cswitch, TIMESTAMP), find_column(cswitch, OLD_PROCESS_NAME_PID), find_column(cswitch, NEW_PROCESS_NAME_PID), find_column(cswitch, CPU))
    };

    let mut events = Vec::new();
    let mut buffer = Vec::new();
    let mut rustc_process = None;

    while { buffer.clear(); r.read_until(b'\n', &mut buffer)? != 0 } {
        // Some lines (notably "FileVersion" ones) include non utf-8 data. We don't care about that
        // data so replacing it with REPLACEMENT_CHARACTER is fine.
        let line = String::from_utf8_lossy(&buffer[..]);
        let columns: Vec<_> = line.trim().split(',').collect();

        let event = match columns[0].trim() {
            PROCESS_START => {
                let process_name = columns[pstart_process_name].trim();

                if !process_name.contains("rustc.exe") {
                    continue;
                }

                assert_eq!(None, rustc_process.replace(process_name.to_string()), "multiple rustc processes found");

                Event::ProcessStart(ProcessStart {
                    process_name: process_name.to_string(),
                    timestamp: columns[pstart_timestamp].trim().parse()?,
                })
            }
            PROCESS_END => {
                let process_name = columns[pend_process_name].trim();

                if !process_name.contains("rustc.exe") {
                    continue;
                }

                assert_eq!(rustc_process.as_ref().unwrap(), process_name, "different rustc process ended");

                Event::ProcessEnd(ProcessEnd {
                    process_name: process_name.to_string(),
                    timestamp: columns[pend_timestamp].trim().parse()?,
                })
            }
            PMC => {
                Event::Pmc(Pmc {
                    timestamp: columns[pmc_timestamp].trim().parse()?,
                    thread_id: columns[pmc_thread_id].trim().parse()?,
                    instructions_retired: columns[pmc_instructions_retired].trim().parse()?,
                    total_cycles: columns[pmc_total_cycles].trim().parse()?,
                })
            }
            CSWITCH => {
                let timestamp = columns[cswitch_timestamp].trim().parse()?;
                let old_process = columns[cswitch_old_process].trim();
                let new_process = columns[cswitch_new_process].trim();

                if !old_process.contains("rustc.exe") && !new_process.contains("rustc.exe") {
                    // In this case, the previous Pmc event at this same timestamp isn't relevant.
                    // There might not be a previous event if the CSwitch event occurs before the
                    // Pmc events start recording.
                    if let Some(previous_event) = events.pop() {
                        assert!(matches!(previous_event, Event::Pmc(_)));
                        assert_eq!(timestamp, previous_event.timestamp());
                    }

                    continue;
                }

                Event::CSwitch(CSwitch {
                    timestamp,
                    old_process: old_process.to_string(),
                    new_process: new_process.to_string(),
                    cpu: columns[cswitch_cpu].trim().parse()?,
                })
            }
            _ => continue
        };

        if let Event::ProcessStart(_) = event {
            // Don't record any events (like PMCs) before the P-Start event
            events.clear();
        } else if let Event::ProcessEnd(_) = event {
            // stop processing events after the P-End event
            events.push(event);
            break;
        }

        // Otherwise, record the event
        events.push(event);
    }

    Ok(events)
}

#[derive(Debug, PartialEq)]
pub struct Counters {
    pub instructions_retired: u64,
    pub total_cycles: u64,
    pub cpu_clock: f64,
}

impl std::ops::Add for Counters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            instructions_retired: self.instructions_retired + rhs.instructions_retired,
            total_cycles: self.total_cycles + rhs.total_cycles,
            cpu_clock: self.cpu_clock + rhs.cpu_clock,
        }
    }
}

impl std::ops::Sub for Counters {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(rhs.instructions_retired < self.instructions_retired);
        assert!(rhs.total_cycles < self.total_cycles);

        Self {
            instructions_retired: self.instructions_retired - rhs.instructions_retired,
            total_cycles: self.total_cycles - rhs.total_cycles,
            cpu_clock: self.cpu_clock - rhs.cpu_clock,
        }
    }
}

impl Default for Counters {
    fn default() -> Self {
        Self {
            instructions_retired: 0,
            total_cycles: 0,
            // FIXME(wesleywiser): We should be properly calculating this value by taking the total time
            // each rustc thread runs per core and adding them togther. This placeholder value is here
            // so that we can still render the detailed query statistics page (although the "Time (%)"
            // column will show the wrong value).
            cpu_clock: 1.0,
        }
    }
}

impl From<&Pmc> for Counters {
    fn from(pmc: &Pmc) -> Self {
        Self {
            instructions_retired: pmc.instructions_retired,
            total_cycles: pmc.total_cycles,
            // FIXME(wesleywiser): see comment in `<Counters as Default>::default()`.
            cpu_clock: 0.0,
        }
    }
}

fn process_events(events: &[Event]) -> anyhow::Result<Counters> {
    anyhow::ensure!(events.len() % 2 == 0, "events must have an even count");

    let pmc_and_cswitch_events =
        events[1..][..(events.len() - 2)].chunks(2).map(|s| {
            match s {
                [Event::Pmc(pmc), Event::CSwitch(cswitch)] => Ok((pmc, cswitch)),
                other => anyhow::bail!("unexpected events: {:?}", other),
            }
        });

    // We need to keep track of when the rustc process is running on a given CPU or not.
    // The basic algorithm here is to note the counters when rustc is moved onto the CPU and
    // then when it is moved off, add the delta to the running total.
    let mut total = Counters::default();
    let mut cpus = HashMap::new();

    for result in pmc_and_cswitch_events {
        let (pmc, cswitch) = result?;
        anyhow::ensure!(pmc.timestamp == cswitch.timestamp, "event timestamps did not match");

        // Handle if the rustc process is moving on the cpu or off the CPU.
        // Check if the process is moving off in case a different thread from
        // the rustc process is what's moving on to the CPU.

        if cswitch.old_process.contains("rustc.exe") {
            if let Some(last_counters) = cpus.remove(&cswitch.cpu) {
                // record the delta between the starting and ending counters in the overall total
                total = total + (Counters::from(pmc) - last_counters);
            } else {
                anyhow::bail!("no existing record when rustc moved off CPU")
            }
        }

        if cswitch.new_process.contains("rustc.exe") {
            anyhow::ensure!(cpus.insert(cswitch.cpu, Counters::from(pmc)).is_none(), "existing record when rustc moved onto CPU");
        }
    }

    Ok(total)
}

/// Given the path to the ETW results file, process it and calculate the
/// hardware performance counter totals for the rustc process.
pub fn parse_etw_file(path: &str) -> anyhow::Result<Counters> {
    log::trace!("path = {}", path);
    let mut file = std::io::BufReader::new(std::fs::File::open(path).unwrap());

    let headers = parse_header(&mut file).unwrap();

    validate_os_header_line(&mut file).unwrap();

    let events = parse_events(&mut file, headers).unwrap();

    Ok(process_events(&events).unwrap())
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use super::{EventHeader, Event, ProcessStart, ProcessEnd, Pmc, CSwitch, Counters};

    #[test]
    fn parse_header() -> anyhow::Result<()> {
        let mut headers = BufReader::new("BeginHeader
FirstReliableEventTimeStamp, TimeStamp
FirstReliableCSwitchEventTimeStamp, TimeStamp
                P-Start,  TimeStamp,     Process Name ( PID),  ParentPID
                P-End,  TimeStamp,     Process Name ( PID),  ParentPID
                T-Start,  TimeStamp,     Process Name ( PID),   ThreadID,          StackBase,         StackLimit,         UsrStkBase,          UsrStkLmt,            TebBase, SubProcessTag, BasePri, PagePri, IoPri, Flags,            Image!Function
                T-End,  TimeStamp,     Process Name ( PID),   ThreadID,          StackBase,         StackLimit,         UsrStkBase,          UsrStkLmt,            TebBase, SubProcessTag, BasePri, PagePri, IoPri, Flags,            Image!Function
                CSwitch,  TimeStamp, New Process Name ( PID),    New TID, Old Process Name ( PID),    Old TID, CPU, IdealProc
            MarkEvent,  TimeStamp,     Type, Level,     TranId,    AppId,     OpId, Text
            Interrupt,  TimeStamp, ElapsedTime, CPU, Vector, MessageIndex,     Status,        ServiceAddr,            Image!Function
                    DPC,  TimeStamp, ElapsedTime, CPU,        ServiceAddr,            Image!Function
                DPCTmr,  TimeStamp, ElapsedTime, CPU,        ServiceAddr,            Image!Function
        SysCallEnter,  TimeStamp,     Process Name ( PID),   ThreadID,        ServiceAddr,            Image!Function
            SysCallExit,  TimeStamp,     Process Name ( PID),   ThreadID,     Status
    SampledProfileNmi,  TimeStamp,     Process Name ( PID),   ThreadID,           PrgrmCtr, CPU, ThreadStartImage!Function,            Image!Function, Interrupts Masked
        SampledProfile,  TimeStamp,     Process Name ( PID),   ThreadID,           PrgrmCtr, CPU, ThreadStartImage!Function,            Image!Function, Count, SampledProfile type
ProfileFreq-SetInterval,  TimeStamp,     Process Name ( PID),   ThreadID, Source, New Interval, Old Interval
    ProfileFreq-DCStart,  TimeStamp,     Process Name ( PID),   ThreadID, Source, New Interval, Old Interval
    ProfileFreq-DCEnd,  TimeStamp,     Process Name ( PID),   ThreadID, Source, New Interval, Old Interval
                    Pmc,  TimeStamp,   ThreadID, InstructionRetired, TotalCycles
EndHeader".as_bytes());

        let expected = vec![
            EventHeader {
                name: "P-Start".into(),
                columns: vec!["TimeStamp".into(), "Process Name ( PID)".into(), "ParentPID".into() ],
            },
            EventHeader {
                name: "P-End".into(),
                columns: vec!["TimeStamp".into(), "Process Name ( PID)".into(), "ParentPID".into()]
            },
            EventHeader {
                name: "CSwitch".into(),
                columns: vec!["TimeStamp".into(), "New Process Name ( PID)".into(), "New TID".into(), "Old Process Name ( PID)".into(), "Old TID".into(), "CPU".into(), "IdealProc".into()],
            },
            EventHeader {
                name: "Pmc".to_string(),
                columns: vec!["TimeStamp".into(), "ThreadID".into(), "InstructionRetired".into(), "TotalCycles".into()],
            }
        ];

        assert_eq!(expected, super::parse_header(&mut headers)?);

        Ok(())
    }

    #[test]
    fn validate_events() -> anyhow::Result<()> {
        let mut line = BufReader::new("OS Version: 10.0.19043, Trace Size: 20736KB, Events Lost: 0, Buffers lost: 0, Trace Start: 132675686690347142, Trace Length: 2 sec, PointerSize: 8, Trace Name: pmc_counters_merged.etl".as_bytes());

        super::validate_os_header_line(&mut line)
    }

    #[test]
    fn parse_events() -> anyhow::Result<()> {
        let headers = vec![
            EventHeader {
                name: "P-Start".into(),
                columns: vec!["TimeStamp".into(), "Process Name ( PID)".into(), "ParentPID".into() ],
            },
            EventHeader {
                name: "P-End".into(),
                columns: vec!["TimeStamp".into(), "Process Name ( PID)".into(), "ParentPID".into()]
            },
            EventHeader {
                name: "CSwitch".into(),
                columns: vec!["TimeStamp".into(), "New Process Name ( PID)".into(), "New TID".into(), "Old Process Name ( PID)".into(), "Old TID".into(), "CPU".into(), "IdealProc".into()],
            },
            EventHeader {
                name: "Pmc".to_string(),
                columns: vec!["TimeStamp".into(), "ThreadID".into(), "InstructionRetired".into(), "TotalCycles".into()],
            }
        ];

        let mut events = BufReader::new("OS Version: 10.0.19043, Trace Size: 20736KB, Events Lost: 0, Buffers lost: 0, Trace Start: 132675686690347142, Trace Length: 2 sec, PointerSize: 8, Trace Name: pmc_counters_merged.etl
FirstReliableEventTimeStamp, 0
FirstReliableCSwitchEventTimeStamp, 6016
   UnknownEvent/Classic,          0,     tracelog.exe (8108),      24632,   0, {68fdd900-4a3e-11d1-84f4-0000f80464e3}, 0x50,  0x00,  0x0002, 48
             GroupMasks,          0,   0, 0x00000000
                    Pmc,     256444,          0, 43430750, 47757881
                CSwitch,     256444,             Idle (   0),          0,    0,   -1,           0,        0,        csrss.exe ( 608),       1044,   16,   -1,         Waiting,    WrUserRequest,  NonSwap,     11,   7,   7,   93715456,    0,    0,   Important,   Important
                    Pmc,     256448,      22992, 82586058, 89184079
                CSwitch,     256448,   powershell.exe (13872),      22992,    Idle (   0),          0,    0,   -1,         Running,        Executive,  NonSwap,     11,   3,   3,          0,    0,    1,   Important,   Important
                P-Start,     104743,        rustc.exe (10612),        480,          1, 0x0000938192a10300, 0x000000049f5be000, 0x00000000, S-1-12-1-2346571520-1185420729-3708355771-3596251678, \"rustc.exe\" --crate-name regex src\\lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=3e524b9e4d4e3569 -C extra-filename=-3e524b9e4d4e3569 --out-dir .tmpeAXOco\\target\\debug\\deps -L dependency=deps --extern aho_corasick=.tmp, <none>, <none>
                    Pmc,     104811,          0, 1808061, 2972786
                CSwitch,     104811,             Idle (   0),          0,    rustc-fake.exe ( 480),      26116,    9,   -1,         Waiting,        Executive,  NonSwap,    326,   4,   1,   91450368,    0,    0,   Important,   Important
                Pmc,     106082,      15340, 3184489, 3416818
                CSwitch,     106082,        rustc.exe (10612),      15340,    Idle (   0),          0,    0,   -1,         Running,        Executive,  NonSwap,   2216,   0,   0,          0,    0,    1,   Important,   Important
                    Pmc,     107179,      15340, 4205942, 3779655
                CSwitch,     107179,        Idle (   0),      15340,    rustc.exe (10612),          0,    0,   -1,         Running,        Executive,  NonSwap,    990,   1,   1,          0,    0,    0,   Important,   Important
                  P-End,    1359642,        rustc.exe (10612),        480,          1, 0x0000938192a10300, 0x00000000, 0x000000049f5be000, 0x00000000, S-1-12-1-2346571520-1185420729-3708355771-3596251678, \"rustc.exe\" --crate-name regex src\\lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=3e524b9e4d4e3569 -C extra-filename=-3e524b9e4d4e3569 --out-dir .tmpeAXOco\\target\\debug\\deps -L dependency=deps --extern aho_corasick=.tmp, <none>, <none>".as_bytes());

        let expected = vec![
            Event::ProcessStart(ProcessStart {
                timestamp: 104743,
                process_name: "rustc.exe (10612)".into(),
            }),
            Event::Pmc(Pmc {
                timestamp: 106082,
                thread_id: 15340,
                instructions_retired: 3184489,
                total_cycles: 3416818,
            }),
            Event::CSwitch(CSwitch {
                timestamp: 106082,
                new_process: "rustc.exe (10612)".into(),
                old_process: "Idle (   0)".into(),
                cpu: 0,
            }),
            Event::Pmc(Pmc {
                timestamp: 107179,
                thread_id: 15340,
                instructions_retired: 4205942,
                total_cycles: 3779655,
            }),
            Event::CSwitch(CSwitch {
                timestamp: 107179,
                new_process: "Idle (   0)".into(),
                old_process: "rustc.exe (10612)".into(),
                cpu: 0,
            }),
            Event::ProcessEnd(ProcessEnd {
                timestamp: 1359642,
                process_name: "rustc.exe (10612)".into(),
            })
        ];

        assert_eq!(expected, super::parse_events(&mut events, headers)?);

        Ok(())
    }

    #[test]
    fn process_events() -> anyhow::Result<()> {
        let events = [
            Event::ProcessStart(ProcessStart {
                timestamp: 104743,
                process_name: "rustc.exe (10612)".into(),
            }),
            Event::Pmc(Pmc {
                timestamp: 106082,
                thread_id: 15340,
                instructions_retired: 3184489,
                total_cycles: 3416818,
            }),
            Event::CSwitch(CSwitch {
                timestamp: 106082,
                new_process: "rustc.exe (10612)".into(),
                old_process: "Idle (   0)".into(),
                cpu: 0,
            }),
            Event::Pmc(Pmc {
                timestamp: 106085,
                thread_id: 99999,
                instructions_retired: 1000000,
                total_cycles: 20000,
            }),
            Event::CSwitch(CSwitch {
                timestamp: 106085,
                old_process: "foobar.exe (1234)".into(),
                new_process: "rustc.exe (10612)".into(),
                cpu: 3,
            }),
            Event::Pmc(Pmc {
                timestamp: 107179,
                thread_id: 15340,
                instructions_retired: 4205942,
                total_cycles: 3779655,
            }),
            Event::CSwitch(CSwitch {
                timestamp: 107179,
                new_process: "Idle (   0)".into(),
                old_process: "rustc.exe (10612)".into(),
                cpu: 0,
            }),
            Event::Pmc(Pmc {
                timestamp: 1259540,
                thread_id: 99999,
                instructions_retired: 1540000,
                total_cycles: 23400,
            }),
            Event::CSwitch(CSwitch {
                timestamp: 1259540,
                new_process: "Idle (   0)".into(),
                old_process: "rustc.exe (10612)".into(),
                cpu: 3
            }),
            Event::ProcessEnd(ProcessEnd {
                timestamp: 1359642,
                process_name: "rustc.exe (10612)".into(),
            })
        ];

        let expected = Counters {
            instructions_retired: 1561453,
            total_cycles: 366237,
            cpu_clock: 1.0,
        };

        assert_eq!(expected, super::process_events(&events)?);

        Ok(())
    }
}