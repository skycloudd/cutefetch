use colored::*;
use sysinfo::{System, SystemExt};

const SEPARATOR: &str = " ";

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    let top_line = InfoLine::Top {
        username: whoami::username(),
        hostname: whoami::hostname(),
    };
    let second_line = InfoLine::Separator(top_line.len());
    let os_line = InfoLine::Normal {
        label: "os".to_string(),
        value: format!("{} {}", whoami::distro(), whoami::arch()),
    };
    let kernel_line = InfoLine::Normal {
        label: "kernel".to_string(),
        value: sys.kernel_version().unwrap_or("unknown".to_string()),
    };
    let uptime_line = InfoLine::Normal {
        label: "uptime".to_string(),
        value: format!("{}h {:02}m", sys.uptime() / 3600, sys.uptime() / 60 % 60),
    };

    let info_lines = vec![
        &top_line,
        &second_line,
        &os_line,
        &kernel_line,
        &uptime_line,
    ];

    let logo = vec![
        "      /#\\".bright_blue(),
        "     /###\\".bright_blue(),
        "    /p^###\\".bright_blue(),
        "   /##P^q##\\".bright_blue(),
        "  /##(   )##\\".bright_blue(),
        " /###P   q#,^\\".bright_blue(),
        "/P^         ^q\\".bright_blue(),
    ];

    let longest_info_line = info_lines
        .iter()
        .clone()
        .map(|info_line| info_line.len())
        .max()
        .unwrap();

    for i in 0..info_lines.len().max(logo.len()) {
        let info_line = info_lines.get(i);
        let empty = "".normal();
        let logo_line = logo.get(i).unwrap_or(&empty);

        if let Some(info_line) = info_line {
            let padding = longest_info_line - info_line.len();
            println!(
                "{}{}{}{}",
                info_line,
                " ".repeat(padding),
                SEPARATOR,
                logo_line
            );
        } else {
            println!(
                "{}{}{}",
                " ".repeat(longest_info_line),
                SEPARATOR,
                logo_line
            );
        }
    }
}

enum InfoLine {
    Normal { label: String, value: String },
    Top { username: String, hostname: String },
    Separator(usize),
}

impl InfoLine {
    fn len(&self) -> usize {
        match self {
            InfoLine::Normal { label, value } => format!("{}: {}", label, value).len(),
            InfoLine::Top { username, hostname } => format!("{}@{}", username, hostname).len(),
            InfoLine::Separator(len) => *len,
        }
    }
}

impl std::fmt::Display for InfoLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfoLine::Normal { label, value } => write!(f, "{}: {}", label.green(), value),
            InfoLine::Top { username, hostname } => {
                write!(f, "{}@{}", username.green(), hostname.green())
            }
            InfoLine::Separator(len) => write!(f, "{}", "-".repeat(*len)),
        }
    }
}
