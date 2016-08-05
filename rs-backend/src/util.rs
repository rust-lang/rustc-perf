use load::TestRun;
use chrono::NaiveDateTime;

/// Returns where the passed date is or should go in the sorted data slice.
fn get_insert_location(data: &[TestRun], date: &NaiveDateTime) -> ::std::result::Result<usize, usize> {
    data.binary_search_by(|probe| probe.date.cmp(date))
}

/// Return the start index for an iterator from the passed date to the index
/// returned by the companion function, `end_idx`.
pub fn start_idx(data: &[TestRun], date: &NaiveDateTime) -> usize {
    match get_insert_location(data, date) {
        Ok(idx) => idx,
        Err(idx) => if idx != 0 {
            idx - 1
        } else {
            0
        }
    }
}

/// Returns the end index for an iterator from the `start_idx()` to this date.
pub fn end_idx(data: &[TestRun], date: &NaiveDateTime) -> usize {
    match get_insert_location(data, date) {
        Ok(idx) => idx,
        Err(idx) => if idx < data.len() {
            idx
        } else {
            data.len() - 1
        }
    }
}
