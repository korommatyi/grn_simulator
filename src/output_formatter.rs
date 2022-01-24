use std::io::Write;

use crate::system::System;

pub struct CSVFormatter {
    pub(crate) output: std::fs::File,
}

impl CSVFormatter {
    pub fn start(&mut self, system: &System) {
        let mut header = "time,last_reaction_id,".to_string();
        header.push_str(&system.idx_to_name.join(","));
        header.push_str("/n");
        self.output.write(header.as_bytes());
    }

    pub fn write_current_state(&mut self, system: &System) {
        let state = system
            .state
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let mut line = system.time_of_last_reaction.to_string();
        line.push_str(",");
        line.push_str(&system.last_reaction.to_string());
        line.push_str(",");
        line.push_str(&state);
        line.push_str("/n");
        self.output.write(line.as_bytes());
    }

    pub fn finish(&mut self, system: &System) {
        // Nothing to do
    }
}
