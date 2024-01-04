use prettytable::{Cell, Row, Table};
use serde::{Deserialize, Serialize};

use crate::core::CliArgs;

#[derive(Debug, Clone)]
pub struct ReportHeader {
    pub node: String,
    pub args: CliArgs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    pub success: bool,
    pub error: Option<String>,
    pub duration: u64,
    pub result: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Report {
    pub header: ReportHeader,
    pub data: Vec<ReportData>,
}

impl Report {
    pub fn new(header: ReportHeader) -> Self {
        Self {
            header,
            data: Vec::new(),
        }
    }

    pub fn add_data(&mut self, data: ReportData) {
        self.data.push(data);
    }

    pub fn display(&self) {
        let mut table = Table::new();

        // Add headers
        table.add_row(Row::new(vec![
            Cell::new("Node"),
            Cell::new("Address"),
            Cell::new("From"),
            Cell::new("To"),
            Cell::new("Method"),
            Cell::new("Timeout"),
            Cell::new("Params"),
            Cell::new("Success"),
            Cell::new("Error"),
            Cell::new("Duration"),
            Cell::new("Result"),
        ]));

        // Add rows
        for data in &self.data {
            table.add_row(Row::new(vec![
                Cell::new(&self.header.node),
                Cell::new(&self.header.args.address),
                Cell::new(&self.header.args.from.to_string()),
                Cell::new(&self.header.args.to.to_string()),
                Cell::new(&self.header.args.method),
                Cell::new(&self.header.args.timeout.to_string()),
                Cell::new(&self.header.args.params),
                Cell::new(&data.success.to_string()),
                Cell::new(&data.error.clone().unwrap_or("".to_string())),
                Cell::new(&data.duration.to_string()),
                Cell::new(&data.result.clone().unwrap_or("".to_string())),
            ]));
        }

        // Print the table
        table.printstd();
    }
}
