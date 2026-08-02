//! Native "save as" dialog for CSV export. The CSV serialization itself lives in
//! the `scanner` crate; this wires it to `rfd` and a timestamped default filename.

use std::time::{SystemTime, UNIX_EPOCH};

pub use scanner::export::to_csv;

/// Open a native "save as" dialog pre-filled with a timestamped name and,
/// if the user confirms, write `csv` there. Cancelling or a write error is a
/// no-op.
pub async fn save_dialog(csv: String) {
    let file = rfd::AsyncFileDialog::new()
        .set_file_name(default_filename(SystemTime::now()))
        .add_filter("CSV", &["csv"])
        .save_file()
        .await;
    if let Some(file) = file {
        let _ = std::fs::write(file.path(), csv);
    }
}

fn default_filename(now: SystemTime) -> String {
    let secs = now
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (y, mo, d, h, mi, s) = utc_ymd_hms(secs);
    format!("mc-scan-{y:04}{mo:02}{d:02}-{h:02}{mi:02}{s:02}Z.csv")
}

/// Civil date (UTC) from seconds since the Unix epoch, via Howard Hinnant's
/// days-to-civil algorithm. Avoids a timezone/date dependency for the filename.
fn utc_ymd_hms(secs: u64) -> (i64, i64, i64, i64, i64, i64) {
    let days = (secs / 86_400) as i64;
    let rem = (secs % 86_400) as i64;
    let (hh, mm, ss) = (rem / 3600, (rem % 3600) / 60, rem % 60);

    let z = days + 719_468;
    let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    (y, m, d, hh, mm, ss)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn filename_formats_utc_timestamp() {
        // 2021-01-01 00:00:00 UTC.
        let t = UNIX_EPOCH + Duration::from_secs(1_609_459_200);
        assert_eq!(default_filename(t), "mc-scan-20210101-000000Z.csv");
        // Epoch itself.
        assert_eq!(default_filename(UNIX_EPOCH), "mc-scan-19700101-000000Z.csv");
    }
}
