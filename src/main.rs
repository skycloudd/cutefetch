use colored::*;
use once_cell::sync::Lazy;

fn main() {
    let mut machine = machine_info::Machine::new();

    let system_info = machine.system_info();

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
        value: system_info.kernel_version,
    };

    let uptime_line = InfoLine::Normal {
        label: "uptime".to_string(),
        value: {
            match uptime_lib::get() {
                Ok(uptime) => {
                    format!(
                        "{}h {:02}m",
                        uptime.as_secs() / 3600,
                        (uptime.as_secs() % 3600) / 60
                    )
                }
                Err(err) => {
                    eprintln!("{}", err);
                    std::process::exit(1);
                }
            }
        },
    };

    let cpu_line = InfoLine::Normal {
        label: "cpu".to_string(),
        value: format!("{}", system_info.processor.brand),
    };

    let gpu_lines = system_info.graphics.iter().map(|gpu| InfoLine::Normal {
        label: "gpu".to_string(),
        value: format!("{}", gpu.name),
    });

    let ram_line = InfoLine::Normal {
        label: "memory".to_string(),
        value: match machine.system_status() {
            Ok(status) => format!(
                "{:.2}MiB / {:.2}MiB",
                status.memory / 1024,
                system_info.memory / 1024 / 1024
            ),
            Err(err) => format!("{}", err),
        },
    };

    let mut info_lines = vec![
        top_line,
        second_line,
        os_line,
        kernel_line,
        uptime_line,
        cpu_line,
    ];

    info_lines.extend(gpu_lines);
    info_lines.push(ram_line);

    let logo = &ARCH_LOGO;

    let longest_first_line = logo.iter().clone().map(|line| line.len()).max().unwrap();

    for i in 0..logo.len().max(info_lines.len()) {
        let empty = "".normal();
        let logo_line = logo.get(i).unwrap_or(&empty);
        let info_line = info_lines.get(i);

        let logo_padding = longest_first_line - logo_line.len();

        println!(
            "{}{}{}",
            logo_line,
            " ".repeat(logo_padding + 1),
            info_line.unwrap_or(&InfoLine::Separator(0)),
        );
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

static ARCH_LOGO: Lazy<[ColoredString; 7]> = Lazy::new(|| {
    [
        "      /#\\".bright_blue(),
        "     /###\\".bright_blue(),
        "    /p^###\\".bright_blue(),
        "   /##P^q##\\".bright_blue(),
        "  /##(   )##\\".bright_blue(),
        " /###P   q#,^\\".bright_blue(),
        "/P^         ^q\\".bright_blue(),
    ]
});
