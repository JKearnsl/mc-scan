use std::time::{SystemTime, UNIX_EPOCH};

pub use scanner::export::to_csv;

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

    let (y, mo, d, h, mi, s) = {
        // UTC civil date from Unix seconds via Hinnant's days-to-civil algorithm,
        // avoiding a timezone/date dependency just for the filename.
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
    };
    format!("mc-scan-{y:04}{mo:02}{d:02}-{h:02}{mi:02}{s:02}Z.csv")
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn filename_formats_utc_timestamp() {
        let t = UNIX_EPOCH + Duration::from_secs(1_609_459_200);
        assert_eq!(default_filename(t), "mc-scan-20210101-000000Z.csv");
        assert_eq!(default_filename(UNIX_EPOCH), "mc-scan-19700101-000000Z.csv");
    }
}
