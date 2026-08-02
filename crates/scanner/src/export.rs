//! CSV serialization of scan results (RFC 4180). The native "save as" dialog
//! lives in the GUI crate; this half is pure and reusable by a headless caller.

use crate::types::{Edition, ServerInfo};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Edition, ServerInfo};
    use std::net::SocketAddr;

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
}
