use std::{
    path::Path,
    str::FromStr,
    sync::{Arc, LazyLock},
};

use crate::load::{partition_in_place, SiteCtxt};
use chrono::{DateTime, NaiveDate, Utc};
use database::{BenchmarkRequest, BenchmarkRequestStatus, BenchmarkRequestType};
use hashbrown::HashSet;
use parking_lot::RwLock;
use regex::Regex;
use tokio::time::{self, Duration};

/// Store the latest master commits or do nothing if all of them are
/// already in the database
async fn create_benchmark_request_master_commits(
    ctxt: &Arc<SiteCtxt>,
    conn: &dyn database::pool::Connection,
) -> anyhow::Result<()> {
    let master_commits = &ctxt.get_master_commits().commits;
    // TODO; delete at some point in the future
    let cutoff: chrono::DateTime<Utc> = chrono::DateTime::from_str("2025-06-01T00:00:00.000Z")?;

    for master_commit in master_commits {
        // We don't want to add masses of obsolete data
        if master_commit.time >= cutoff {
            let pr = master_commit.pr.unwrap_or(0);
            let benchmark = BenchmarkRequest::create_master(
                &master_commit.sha,
                &master_commit.parent_sha,
                pr,
                master_commit.time,
                BenchmarkRequestStatus::ArtifactsReady,
                "",
                "",
            );
            conn.insert_benchmark_request(&benchmark).await;
        }
    }
    Ok(())
}

/// Parses strings in the following formats extracting the Date & release tag
/// `static.rust-lang.org/dist/2016-05-24/channel-rust-1.9.0.toml`
/// `static.rust-lang.org/dist/2016-05-31/channel-rust-nightly.toml`
/// `static.rust-lang.org/dist/2016-06-01/channel-rust-beta.toml`
/// `static.rust-lang.org/dist/2025-06-26/channel-rust-1.89-beta.toml`
/// `static.rust-lang.org/dist/2025-06-26/channel-rust-1.89.0-beta.toml`
/// `static.rust-lang.org/dist/2025-06-26/channel-rust-1.89.0-beta.2.toml`
fn parse_release_string(url: &str) -> Option<(String, DateTime<Utc>)> {
    static VERSION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+\.\d+\.\d+)").unwrap());

    // Grab ".../YYYY-MM-DD/FILE.toml" components with Path helpers.
    let file = Path::new(url).file_name().and_then(|n| n.to_str())?;

    let date_str = Path::new(url)
        .parent()
        .and_then(Path::file_name)
        .and_then(|n| n.to_str())?;

    // No other beta releases are recognized as toolchains.
    //
    // We also have names like this:
    //
    // * channel-rust-1.75-beta.toml
    // * channel-rust-1.75.0-beta.toml
    // * channel-rust-1.75.0-beta.1.toml
    //
    // Which should get ignored for now, they're not consumable via rustup yet.
    if file.contains("beta") && file != "channel-rust-beta.toml" {
        return None;
    }

    // Parse the YYYY-MM-DD segment and stamp it with *current* UTC time.
    if let Ok(naive) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let published = naive
            .and_hms_opt(0, 0, 0)
            .expect("valid HMS")
            .and_local_timezone(Utc)
            .single()
            .unwrap();

        // Special-case the rolling beta channel.
        if file == "channel-rust-beta.toml" {
            return Some((format!("beta-{date_str}"), published));
        }

        // Otherwise pull out a semver like "1.70.0" and return it.
        if let Some(cap) = VERSION_RE.captures(file).and_then(|m| m.get(1)) {
            return Some((cap.as_str().to_owned(), published));
        }
    }

    None
}

/// Store the latest release commits or do nothing if all of them are
/// already in the database
async fn create_benchmark_request_releases(
    conn: &dyn database::pool::Connection,
) -> anyhow::Result<()> {
    let releases: String = reqwest::get("https://static.rust-lang.org/manifests.txt")
        .await?
        .text()
        .await?;
    // TODO; delete at some point in the future
    let cutoff: chrono::DateTime<Utc> = chrono::DateTime::from_str("2025-06-01T00:00:00.000Z")?;

    for release_string in releases.lines().rev().take(20) {
        if let Some((name, date_time)) = parse_release_string(release_string) {
            if date_time >= cutoff {
                let release_request = BenchmarkRequest::create_release(
                    &name,
                    date_time,
                    BenchmarkRequestStatus::ArtifactsReady,
                    "",
                    "",
                );
                conn.insert_benchmark_request(&release_request).await;
            }
        }
    }
    Ok(())
}

/// Sorts try and master requests that are in the `ArtifactsReady` status.
/// Doesn't consider in-progress requests or release artifacts.
fn sort_benchmark_requests(done: &HashSet<String>, request_queue: &mut [BenchmarkRequest]) {
    let mut done: HashSet<String> = done.iter().cloned().collect();

    // Ensure all the items are ready to be sorted, if they are not this is
    // undefined behaviour
    assert!(request_queue.iter().all(|bmr| {
        bmr.status == BenchmarkRequestStatus::ArtifactsReady
            && matches!(
                bmr.commit_type,
                BenchmarkRequestType::Master { .. } | BenchmarkRequestType::Try { .. }
            )
    }));

    let mut finished = 0;
    while finished < request_queue.len() {
        // The next level is those elements in the unordered queue which
        // are ready to be benchmarked (i.e., those with parent in done or no
        // parent).
        let level_len = partition_in_place(request_queue[finished..].iter_mut(), |bmr| {
            bmr.parent_sha().is_none_or(|parent| done.contains(parent))
        });

        // No commit is ready for benchmarking. This can happen e.g. when a try parent commit
        // was forcefully removed from the master branch of rust-lang/rust. In this case, just
        // let the commits be benchmarked in the current order that we have, these benchmark runs
        // just won't have a parent result available.
        if level_len == 0 {
            if cfg!(test) {
                panic!("No commit is ready for benchmarking");
            } else {
                log::warn!("No commit is ready for benchmarking");
                return;
            }
        }

        // Everything in level has the same topological order, then we sort based on heuristics
        let level = &mut request_queue[finished..][..level_len];
        level.sort_unstable_by_key(|bmr| {
            (
                // Pr number takes priority
                *bmr.pr().unwrap_or(&0),
                // Order master commits before try commits
                match bmr.commit_type {
                    BenchmarkRequestType::Try { .. } => 1,
                    BenchmarkRequestType::Master { .. } => 0,
                    BenchmarkRequestType::Release { .. } => unreachable!(),
                },
                bmr.created_at,
            )
        });
        for c in level {
            done.insert(c.tag().to_string());
        }
        finished += level_len;
    }
}

pub trait ExtractIf<T> {
    fn extract_if_stable<F>(&mut self, predicate: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool;
}

/// Vec method `extract_if` is unstable, this very simple implementation
/// can be deleted once it is stable
impl<T> ExtractIf<T> for Vec<T> {
    fn extract_if_stable<F>(&mut self, mut predicate: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool,
    {
        let mut extracted = Vec::new();
        let mut i = 0;

        while i < self.len() {
            if predicate(&self[i]) {
                extracted.push(self.remove(i));
            } else {
                i += 1;
            }
        }
        extracted
    }
}

/// Assumes that master/release artifacts have been put into the DB.
pub async fn build_queue(
    conn: &mut dyn database::pool::Connection,
    completed_set: &HashSet<String>,
) -> anyhow::Result<Vec<BenchmarkRequest>> {
    let mut pending = conn
        .get_benchmark_requests_by_status(&[
            BenchmarkRequestStatus::InProgress,
            BenchmarkRequestStatus::ArtifactsReady,
        ])
        .await?;

    // The queue starts with in progress
    let mut queue: Vec<BenchmarkRequest> = pending
        .extract_if_stable(|request| matches!(request.status, BenchmarkRequestStatus::InProgress));

    // We sort the in-progress ones based on the started date
    queue.sort_unstable_by(|a, b| a.created_at.cmp(&b.created_at));

    // Add release artifacts ordered by the release tag (1.87.0 before 1.88.0) and `created_at`.
    let mut release_artifacts: Vec<BenchmarkRequest> = pending.extract_if_stable(|request| {
        matches!(request.commit_type, BenchmarkRequestType::Release { .. })
    });

    release_artifacts.sort_unstable_by(|a, b| {
        a.tag()
            .cmp(b.tag())
            .then_with(|| a.created_at.cmp(&b.created_at))
    });

    queue.append(&mut release_artifacts);
    sort_benchmark_requests(completed_set, &mut pending);
    queue.append(&mut pending);
    Ok(queue)
}

/// Enqueue the job into the job_queue
async fn enqueue_next_job(conn: &mut dyn database::pool::Connection) -> anyhow::Result<()> {
    // We draw back all completed requests
    let completed: HashSet<String> = conn
        .get_benchmark_requests_by_status(&[BenchmarkRequestStatus::Completed])
        .await?
        .into_iter()
        .map(|request| request.tag().to_string())
        .collect();

    let queue = build_queue(conn, &completed).await?;

    if let Some(request) = queue.into_iter().next() {
        if request.status != BenchmarkRequestStatus::InProgress {
            // TODO: actually enqueue the jobs
            conn.update_benchmark_request_status(&request, BenchmarkRequestStatus::InProgress)
                .await?;
        }
    }

    Ok(())
}

/// For queueing jobs, add the jobs you want to queue to this function
async fn cron_enqueue_jobs(site_ctxt: &Arc<SiteCtxt>) -> anyhow::Result<()> {
    let mut conn = site_ctxt.conn().await;
    // Put the master commits into the `benchmark_requests` queue
    create_benchmark_request_master_commits(site_ctxt, &*conn).await?;
    // Put the releases into the `benchmark_requests` queue
    create_benchmark_request_releases(&*conn).await?;
    enqueue_next_job(&mut *conn).await?;
    Ok(())
}

/// Entry point for the cron
pub async fn cron_main(site_ctxt: Arc<RwLock<Option<Arc<SiteCtxt>>>>, seconds: u64) {
    let mut interval = time::interval(Duration::from_secs(seconds));
    let ctxt = site_ctxt.clone();

    loop {
        interval.tick().await;

        if let Some(ctxt_clone) = {
            let guard = ctxt.read();
            guard.as_ref().cloned()
        } {
            match cron_enqueue_jobs(&ctxt_clone).await {
                Ok(_) => log::info!("Cron job executed at: {:?}", std::time::SystemTime::now()),
                Err(e) => log::error!("Cron job failed to execute {}", e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Duration, NaiveDate, TimeZone, Utc};
    use database::tests::run_postgres_test;

    /// Helper: unwrap the Option, panic otherwise.
    fn tag(url: &str) -> String {
        parse_release_string(url)
            .expect("Some") // Option<_>
            .0 // take the tag
    }

    /// Helper: unwrap the DateTime and keep only the YYYY-MM-DD part
    fn day(url: &str) -> NaiveDate {
        parse_release_string(url).expect("Some").1.date_naive()
    }

    fn days_ago(day_str: &str) -> chrono::DateTime<Utc> {
        // Walk backwards until the first non-digit, then slice
        let days = day_str
            .strip_prefix("days")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let timestamp = Utc::now() - Duration::days(days);
        // zero out the seconds
        Utc.with_ymd_and_hms(
            timestamp.year(),
            timestamp.month(),
            timestamp.day(),
            0,
            0,
            0,
        )
        .unwrap()
    }

    fn create_master(sha: &str, parent: &str, pr: u32, age_days: &str) -> BenchmarkRequest {
        BenchmarkRequest::create_master(
            sha,
            parent,
            pr,
            days_ago(age_days),
            BenchmarkRequestStatus::ArtifactsReady,
            "",
            "",
        )
    }

    fn create_try(sha: &str, parent: &str, pr: u32, age_days: &str) -> BenchmarkRequest {
        BenchmarkRequest::create_try(
            sha,
            Some(parent),
            pr,
            days_ago(age_days),
            BenchmarkRequestStatus::ArtifactsReady,
            "",
            "",
        )
    }

    fn create_release(tag: &str, age_days: &str) -> BenchmarkRequest {
        BenchmarkRequest::create_release(
            tag,
            days_ago(age_days),
            BenchmarkRequestStatus::ArtifactsReady,
            "",
            "",
        )
    }

    async fn db_insert_requests(
        conn: &dyn database::pool::Connection,
        requests: &[BenchmarkRequest],
    ) {
        for request in requests {
            conn.insert_benchmark_request(&request).await;
        }
    }

    /// Get an `InProgress` item out of the `benchmark_requests` table. In
    /// practice this is the job that has been enqueued.
    async fn get_in_progress(conn: &dyn database::pool::Connection) -> Option<BenchmarkRequest> {
        conn.get_benchmark_requests_by_status(&[BenchmarkRequestStatus::InProgress])
            .await
            .unwrap()
            .first()
            .cloned()
    }

    fn queue_order_matches(queue: &[BenchmarkRequest], expected: &[&str]) {
        let queue_shas: Vec<&str> = queue.iter().map(|req| req.tag()).collect();
        assert_eq!(queue_shas, expected)
    }

    trait BenchmarkRequestExt {
        fn with_status(self, status: BenchmarkRequestStatus) -> Self;
    }

    impl BenchmarkRequestExt for BenchmarkRequest {
        fn with_status(mut self, status: BenchmarkRequestStatus) -> Self {
            self.status = status;
            self
        }
    }

    /// Nothing to do, empty table
    #[tokio::test]
    async fn enqueue_next_job_no_jobs() {
        run_postgres_test(|ctx| async {
            let mut db = ctx.db_client().connection().await;

            enqueue_next_job(&mut *db).await?;

            let in_progress = get_in_progress(&*db).await;

            assert!(in_progress.is_none());
            Ok(ctx)
        })
        .await;
    }

    /// Parent completed -> child is picked
    #[tokio::test]
    async fn get_next_benchmark_request_completed_parent() {
        run_postgres_test(|ctx| async {
            let mut db = ctx.db_client().connection().await;
            let parent =
                create_master("a", "x", 1, "days5").with_status(BenchmarkRequestStatus::Completed);
            let child = create_master("b", "a", 1, "days5");

            db_insert_requests(&*db, &[parent, child]).await;

            enqueue_next_job(&mut *db).await?;

            let in_progress = get_in_progress(&*db).await;

            assert_eq!(in_progress.unwrap().tag(), "b");
            Ok(ctx)
        })
        .await;
    }

    /// Release (no parent) is always eligible
    #[tokio::test]
    async fn get_next_benchmark_request_no_parent_release() {
        run_postgres_test(|ctx| async {
            let mut db = ctx.db_client().connection().await;
            let release = create_release("v1.2.3", "days2");

            db_insert_requests(&*db, &[release]).await;

            enqueue_next_job(&mut *db).await?;

            let in_progress = get_in_progress(&*db).await;

            assert_eq!(in_progress.unwrap().tag(), "v1.2.3");
            Ok(ctx)
        })
        .await;
    }

    /// Parent exists but is older -> parent gets picked
    #[tokio::test]
    async fn get_next_benchmark_request_oldest_first() {
        run_postgres_test(|ctx| async {
            let mut db = ctx.db_client().connection().await;
            let c1 = create_master("x", "x", 1, "days521")
                .with_status(BenchmarkRequestStatus::Completed);
            let c2 = create_master("y", "y", 2, "days521")
                .with_status(BenchmarkRequestStatus::Completed);

            let m1 = create_master("old", "x", 3, "days45");
            let m2 = create_master("new", "y", 4, "days1");

            db_insert_requests(&*db, &[c1, c2, m1, m2]).await;
            enqueue_next_job(&mut *db).await?;

            let in_progress = get_in_progress(&*db).await;

            assert_eq!(in_progress.unwrap().tag(), "old");
            Ok(ctx)
        })
        .await;
    }

    /// Parent SHA missing entirely -> child is ready
    #[cfg(unix)] // test will not panic on windows and would be skipped entirely
    #[tokio::test]
    #[should_panic(expected = "No commit is ready for benchmarking")]
    async fn get_next_benchmark_request_missing_parent() {
        run_postgres_test(|ctx| async {
            let mut db = ctx.db_client().connection().await;
            let orphan = create_master("orphan", "gone", 42, "days1");

            db_insert_requests(&*db, &[orphan]).await;
            enqueue_next_job(&mut *db).await?;

            let in_progress = get_in_progress(&*db).await;
            assert_eq!(in_progress.unwrap().tag(), "orphan");

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn get_next_benchmark_request_large_mixture() {
        run_postgres_test(|ctx| async {
            let mut db = ctx.db_client().connection().await;
            // Fresh parents that will unblock some children
            let parent_master = create_master("parent_m", "x", 911, "days5")
                .with_status(BenchmarkRequestStatus::Completed);
            let parent_try = create_try("parent_t", "x", 888, "days4")
                .with_status(BenchmarkRequestStatus::Completed);
            let parent_master_two = create_master("gp", "x", 922, "days5")
                .with_status(BenchmarkRequestStatus::Completed);
            let parent_master_three = create_master("blocked_p", "x", 932, "days5")
                .with_status(BenchmarkRequestStatus::Completed);

            // Two releases, the older one should win overall
            let rel_old = create_release("v0.8.0", "days40"); // 40days old
            let rel_new = create_release("v1.0.0", "days10");

            // Ready masters (parents completed)
            let master_low_pr = create_master("m_low", "parent_m", 1, "days12");
            let master_high_pr = create_master("m_high", "parent_m", 7, "days8");

            let blocked_parent = create_master("blocked_p", "gp", 0, "days3");
            let master_blocked = create_master("blocked_c", "blocked_p", 0, "days1");

            // A try commit that is ready
            let try_ready = create_try("t_ready", "parent_t", 42, "days2");

            let requests = vec![
                parent_master,
                parent_master_two,
                parent_master_three,
                parent_try,
                master_high_pr,
                master_low_pr,
                master_blocked,
                blocked_parent,
                try_ready,
                rel_old,
                rel_new,
            ];

            db_insert_requests(&*db, &requests).await;
            enqueue_next_job(&mut *db).await?;

            // The oldest release ("v0.8.0") outranks everything else
            let in_progress = get_in_progress(&*db).await;
            assert_eq!(in_progress.unwrap().tag(), "v0.8.0");
            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn get_next_benchmark_request_large_mixture_no_release() {
        run_postgres_test(|ctx| async {
            let mut db = ctx.db_client().connection().await;
            // Fresh parents that will unblock some children
            let parent_master = create_master("parent_m", "x", 8, "days5")
                .with_status(BenchmarkRequestStatus::Completed);
            let parent_try = create_try("parent_t", "x", 9, "days4")
                .with_status(BenchmarkRequestStatus::Completed);
            let parent_master_two = create_master("gp", "x", 10, "days5")
                .with_status(BenchmarkRequestStatus::Completed);
            let parent_master_three = create_master("blocked_p", "x", 11, "days5")
                .with_status(BenchmarkRequestStatus::Completed);

            // Ready masters (parents completed)
            let m1 = create_master("m_low", "parent_m", 3, "days12");
            let m2 = create_master("m_high", "parent_m", 7, "days8");

            let m3 = create_master("B", "gp", 1, "days3");
            let m4 = create_master("C", "blocked_p", 2, "days1");

            // A try commit that is ready
            let t1 = create_try("t_ready", "parent_t", 42, "days2");

            let requests = vec![
                parent_master,
                parent_master_two,
                parent_master_three,
                parent_try,
                m2,
                m1,
                m4,
                m3,
                t1,
            ];

            db_insert_requests(&*db, &requests).await;
            enqueue_next_job(&mut *db).await?;

            // The oldest release ("v0.8.0") outranks everything else
            let in_progress = get_in_progress(&*db).await;
            assert_eq!(in_progress.unwrap().tag(), "B");
            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn queue_ordering() {
        run_postgres_test(|ctx| async {
            /* Key:
             * +---------------------+
             * | m - master          |
             * | t - try             |
             * | r - release         |
             * | C - Completed       |
             * | R - Artifacts Ready |
             * | IP - In Progress    |
             * +---------------------+
             *
             * This is the graph we have:
             *              2: A release
             *             +------------+
             *             | r "v1.2.3" |
             *             +------------+
             *                                  1: Currently `in_progress`
             *                                     +---------------+
             *                                +--->| t "t1" IP pr1 |
             *                                |    +---------------+
             *             +-----------+      |
             *             | m "rrr" C | -----+-->
             *             +-----------+      |
             *                                |    +---------------+
             *                                +--->| t "yee" R pr1 | 3: a try with a low pr
             *                                     +---------------+
             *             +-----------+
             *             | m "aaa" C |
             *             +-----------+
             *                   |
             *                   V
             *           +----------------+
             *           | m "mmm" R pr88 | 6: a master commit
             *           +----------------+
             *
             *             +-----------+
             *             | m "345" C |
             *             +-----------+
             *                   |
             *                   V
             *           +----------------+
             *           | m "123" R pr11 | 4: a master commit, high pr number
             *           +----------------+
             *
             *
             *             +-----------+
             *             | m "bar" C |
             *             +-----------+
             *                   |
             *                   V
             *           +----------------+
             *           | m "foo" R pr77 | 5: a master commit
             *           +----------------+
             *                   |
             *                   V
             *           +---------------+
             *           | t "baz" R pr4 | 7: a try with a low pr, blocked by parent
             *           +---------------+
             *
             *  The master commits should take priority, then "yee" followed
             *  by "baz"
             **/

            let mut db = ctx.db_client().connection().await;
            let requests = vec![
                create_master("foo", "bar", 77, "days2"),
                create_master("123", "345", 11, "days2"),
                create_try("baz", "foo", 4, "days1"),
                create_release("v.1.2.3", "days2"),
                create_try("yee", "rrr", 1, "days2"), // lower PR number takes priority
                create_try("t1", "rrr", 1, "days1").with_status(BenchmarkRequestStatus::InProgress),
                create_master("mmm", "aaa", 88, "days2"),
            ];

            db_insert_requests(&*db, &requests).await;

            let completed: HashSet<String> = HashSet::from([
                "bar".to_string(),
                "345".to_string(),
                "rrr".to_string(),
                "aaa".to_string(),
            ]);

            let sorted: Vec<BenchmarkRequest> = build_queue(&mut *db, &completed).await.unwrap();

            queue_order_matches(
                &sorted,
                &["t1", "v.1.2.3", "yee", "123", "foo", "mmm", "baz"],
            );
            Ok(ctx)
        })
        .await;
    }

    #[test]
    fn parses_stable_versions() {
        assert_eq!(
            tag("static.rust-lang.org/dist/2016-05-24/channel-rust-1.9.0.toml"),
            "1.9.0"
        );
        assert_eq!(
            day("static.rust-lang.org/dist/2016-05-24/channel-rust-1.9.0.toml"),
            NaiveDate::from_ymd_opt(2016, 5, 24).unwrap()
        );

        assert_eq!(
            tag("static.rust-lang.org/dist/2025-06-26/channel-rust-1.88.0.toml"),
            "1.88.0"
        );
        assert_eq!(
            day("static.rust-lang.org/dist/2025-06-26/channel-rust-1.88.0.toml"),
            NaiveDate::from_ymd_opt(2025, 6, 26).unwrap()
        );
    }

    #[test]
    fn parses_plain_beta_channel() {
        let want = "beta-2016-06-01";
        let url = "static.rust-lang.org/dist/2016-06-01/channel-rust-beta.toml";

        assert_eq!(tag(url), want);
        assert_eq!(day(url), NaiveDate::from_ymd_opt(2016, 6, 1).unwrap());
    }

    #[test]
    fn skips_unconsumable_channels() {
        // nightly never returns Anything
        assert!(parse_release_string(
            "static.rust-lang.org/dist/2016-05-31/channel-rust-nightly.toml"
        )
        .is_none());

        // versioned-beta artefacts are skipped too
        for should_ignore in [
            "static.rust-lang.org/dist/2025-06-26/channel-rust-1.89-beta.toml",
            "static.rust-lang.org/dist/2025-06-26/channel-rust-1.89.0-beta.toml",
            "static.rust-lang.org/dist/2025-06-26/channel-rust-1.89.0-beta.2.toml",
        ] {
            assert!(
                parse_release_string(should_ignore).is_none(),
                "{should_ignore} should be ignored"
            );
        }
    }
}
