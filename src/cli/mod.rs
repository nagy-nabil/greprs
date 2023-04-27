use std::fmt::Display;

pub struct CliApp {
    pub name: String,
    pub desc: Option<String>,
    pub cmds: Vec<CliCmd>,
    pub opts: Vec<CliOpt>, // global option to the application
}

pub struct CliCmd {
    pub name: String,
    pub desc: Option<String>,
    pub opts: Vec<CliOpt>, /* local option to the cmd
                            */
}

pub struct CliOpt {
    pub name: String,
    pub short_cut: Option<char>,
    pub desc: Option<String>,
    pub value_type: Option<()>,
    pub is_required: bool,
}

impl Display for CliOpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("--{}", self.name);

        // add optional field if exist
        if let Some(v) = self.short_cut {
            s.push_str(" -");
            s.push(v);
        }

        if self.is_required == true {
            s.push_str(" -Required- ");
        } else {
            s.push_str(" -Optional- ");
        }

        if let Some(v) = &self.desc {
            s.push_str("\t\t");
            s.push_str(v.clone().as_str());
        }

        write!(f, "{}", s)
    }
}

impl Display for CliCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("{}\n", self.name);

        // add optional field if exist
        if let Some(v) = &self.desc {
            s.push_str(v.clone().as_str());
        }

        // print each option
        for opt in &self.opts {
            s.push_str(format!("\n\n{}\n", opt).as_str());
        }

        write!(f, "{}", s)
    }
}

impl Display for CliApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("{}\n", self.name);
        // add optional field if exist
        if let Some(v) = &self.desc {
            s.push_str(v.clone().as_str());
        }

        // print each cmd
        for opt in &self.opts {
            s.push_str(format!("\n\n{}\n", opt).as_str());
        }

        // print each cmd
        for cmd in &self.cmds {
            s.push_str(format!("\n\n{}\n", cmd).as_str());
        }
        write!(f, "{}", s)
    }
}
