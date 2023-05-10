use cli::{CliApp, CliCmd, CliOpt};
use greprs::{run, Config};
use std::{env, process, vec};

pub mod cli;

fn main() {
    let app = CliApp {
        name: format!("greprs"),
        desc: Some(format!("grep is dead? with home made framework?")),
        cmds: vec![
            CliCmd {
                name: format!("grepcmd"),
                desc: Some(format!("this test cmd")),
                opts: vec![
                    CliOpt {
                        desc: Some(format!("option desc")),
                        is_required: false,
                        name: format!("opt"),
                        short_cut: None,
                        value_type: None,
                    },
                    CliOpt {
                        desc: Some(format!("option desc")),
                        is_required: false,
                        name: format!("opt"),
                        short_cut: None,
                        value_type: None,
                    },
                ],
            },
            CliCmd {
                name: format!("grepcmd"),
                desc: Some(format!("this test cmd")),
                opts: vec![
                    CliOpt {
                        desc: Some(format!("option desc")),
                        is_required: false,
                        name: format!("opt"),
                        short_cut: None,
                        value_type: None,
                    },
                    CliOpt {
                        desc: Some(format!("option desc")),
                        is_required: false,
                        name: format!("opt"),
                        short_cut: None,
                        value_type: None,
                    },
                ],
            },
        ],
        opts: vec![CliOpt {
            desc: Some(format!("option desc")),
            is_required: false,
            name: format!("opt"),
            short_cut: None,
            value_type: None,
        }],
    };
    eprintln!("{}", app);
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("unknow commands\n{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("error while running {}", e);
        process::exit(1);
    }
}
