use chrono::NaiveDate;

use super::{ocs::Ocs, transaction::Transaction};

pub enum Task {
    TerminalTask {
        name: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
        opt_dependency: Vec<String>, // ?
        hr: Vec<(Ocs, i32)>,
        transactions: Vec<Transaction>,
        // ? employments
    },
    ComplexTask {
        name: String,
        sub_tasks: Vec<Task>,
        opt_dependency: Vec<String>, // ?
        hr: Vec<(Ocs, i32)>,
        min_start_date: Option<NaiveDate>,
        max_end_date: Option<NaiveDate>,
        max_expense: Option<i32>,
    },
}
