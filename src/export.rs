//! CSV export of scan results via a native "save as" dialog.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::scanner::types::{Edition, ServerInfo};

const HEADER: &str = "addr,edition,version,protocol,online,max_players,latency_ms,\
online_mode,secure_chat,motd,players,world,plugins,mods,gamemode,bedrock_edition,sub_motd";

/// Serialize results as RFC 4180 CSV. Nested fields (players, plugins, mods) are
/// flattened into a single `;`-separated cell.
pub fn to_csv(items: &[ServerInfo]) -> String {
    let mut out = String::with_capacity(HEADER.len() + items.len() * 96);
    out.push_str(HEADER);
    out.push('\n');
    for s in items {
        let mods = s
            .mods
            .iter()
            .map(|m| {
                if m.version.is_empty() {
                    m.id.clone()
                } else {
                    format!("{} {}", m.id, m.version)
                }
            })
            .collect::<Vec<_>>()
            .join(";");

        // Order must match HEADER exactly.
        let fields = [
            s.addr.to_string(),
            edition_name(&s.edition).to_string(),
            s.version.clone(),
            s.protocol.to_string(),
            s.online.to_string(),
            s.max_players.to_string(),
            s.latency_ms.to_string(),
            tristate(s.online_mode, "online", "cracked"),
            tristate(s.secure_chat, "true", "false"),
            s.motd.clone(),
            s.samples.join(";"),
            s.world.clone().unwrap_or_default(),
            s.plugins.join(";"),
            mods,
            s.gamemode.clone().unwrap_or_default(),
            s.bedrock_edition.clone().unwrap_or_default(),
            s.sub_motd.clone().unwrap_or_default(),
        ];
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            push_escaped(&mut out, field);
        }
        out.push('\n');
    }
    out
}

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

fn edition_name(e: &Edition) -> &'static str {
    match e {
        Edition::Java => "Java",
        Edition::Bedrock => "Bedrock",
    }
}

fn tristate(v: Option<bool>, yes: &str, no: &str) -> String {
    match v {
        Some(true) => yes.to_string(),
        Some(false) => no.to_string(),
        None => String::new(),
    }
}

fn push_escaped(out: &mut String, field: &str) {
    if field.contains(['"', ',', '\n', '\r']) {
        out.push('"');
        for c in field.chars() {
            if c == '"' {
                out.push('"'); // RFC 4180: double the quote
            }
            out.push(c);
        }
        out.push('"');
    } else {
        out.push_str(field);
    }
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
    use std::net::SocketAddr;
    use std::time::Duration;

    fn server(port: u16) -> ServerInfo {
        ServerInfo::base(SocketAddr::from(([1, 2, 3, 4], port)), Edition::Java)
    }

    #[test]
    fn header_column_count_matches_row() {
        let cols = HEADER.split(',').count();
        let csv = to_csv(&[server(25565)]);
        let row = csv.lines().nth(1).unwrap();
        // No field here needs quoting, so a plain split is accurate.
        assert_eq!(row.split(',').count(), cols);
    }

    #[test]
    fn writes_scalar_and_flattened_fields() {
        let mut s = server(25565);
        s.version = "1.20.1".into();
        s.online = 5;
        s.max_players = 20;
        s.samples = vec!["alice".into(), "bob".into()];
        s.online_mode = Some(false);
        let csv = to_csv(&[s]);
        let row = csv.lines().nth(1).unwrap();
        assert!(row.starts_with("1.2.3.4:25565,Java,1.20.1,0,5,20,0,cracked,"));
        assert!(row.contains(",alice;bob,"));
    }

    #[test]
    fn escapes_commas_quotes_and_newlines() {
        let mut s = server(25565);
        s.motd = "A, \"great\" server\nline2".into();
        let csv = to_csv(&[s]);
        assert!(csv.contains("\"A, \"\"great\"\" server\nline2\""));
    }

    #[test]
    fn empty_results_still_have_a_header() {
        let csv = to_csv(&[]);
        assert_eq!(csv, format!("{HEADER}\n"));
    }

    #[test]
    fn filename_formats_utc_timestamp() {
        // 2021-01-01 00:00:00 UTC.
        let t = UNIX_EPOCH + Duration::from_secs(1_609_459_200);
        assert_eq!(default_filename(t), "mc-scan-20210101-000000Z.csv");
        // Epoch itself.
        assert_eq!(default_filename(UNIX_EPOCH), "mc-scan-19700101-000000Z.csv");
    }
}
