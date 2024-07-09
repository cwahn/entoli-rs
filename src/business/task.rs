use chrono::{Duration, NaiveDate};

use super::{ocs::Ocs, transaction::Transaction};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Task {
    TerminalTask {
        name: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
        opt_dependency: Vec<Task>, // ?
        hr_req: Vec<(Ocs, i32)>,
        transactions: Vec<Transaction>,
        // ? employments
    },
    ComplexTask {
        name: String,
        sub_tasks: Vec<Task>,
        opt_dependency: Vec<Task>, // ?
        hr_req: Vec<(Ocs, i32)>,
        min_start_date: Option<NaiveDate>,
        max_end_date: Option<NaiveDate>,
        max_expense: Option<i32>,
    },
}

impl Task {
    fn name(&self) -> &str {
        match self {
            Task::TerminalTask { name, .. } => name,
            Task::ComplexTask { name, .. } => name,
        }
    }

    fn start_date(&self) -> NaiveDate {
        match self {
            Task::TerminalTask { start_date, .. } => *start_date,
            Task::ComplexTask { sub_tasks, .. } => {
                sub_tasks.iter().map(|x| x.start_date()).min().unwrap() // Complex task must have at least one sub task
            }
        }
    }

    fn end_date(&self) -> NaiveDate {
        match self {
            Task::TerminalTask { end_date, .. } => *end_date,
            Task::ComplexTask { sub_tasks, .. } => {
                sub_tasks.iter().map(|x| x.end_date()).max().unwrap() // Complex task must have at least one sub task
            }
        }
    }

    fn duration(&self) -> Duration {
        self.end_date().signed_duration_since(self.start_date())
    }

    fn sub_tasks(&self) -> Vec<&Task> {
        match self {
            Task::TerminalTask { .. } => vec![],
            Task::ComplexTask { sub_tasks, .. } => sub_tasks.iter().collect(),
        }
    }

    fn dependencies(&self) -> Vec<&Task> {
        match self {
            Task::TerminalTask { opt_dependency, .. } => opt_dependency.iter().collect(),
            Task::ComplexTask { opt_dependency, .. } => opt_dependency.iter().collect(),
        }
    }

    // fn hr_req(&self, ocs: Ocs) -> i32 {
    //     match self {
            
    //     }
    // }

    fn sub_dependency_pairs(&self) -> Vec<(&Task, &Task)> {
        self.sub_tasks()
            .iter()
            .flat_map(|x| {
                x.dependencies()
                    .iter()
                    .map(move |y| (*x, *y))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    // fn sorted(self) -> Task {
    //     // Recrusively sort the tasks based on their start date and then by their end date.
    //     // if self.sub_tasks().is_empty() {
    //     //     self
    //     // } else {
    //     //     let mut sorted_sub_tasks = self
    //     //         .sub_tasks()
    //     //         .iter()
    //     //         .map(|x| x.sorted())
    //     //         .collect::<Vec<_>>();

    //     //     sorted_sub_tasks.sort_by(|x, y| {
    //     //         x.start_date()
    //     //             .cmp(&y.start_date())
    //     //             .then(x.end_date().cmp(&y.end_date()))
    //     //     });

    //     //     Task::ComplexTask {
    //     //         name: self.name().to_string(),
    //     //         sub_tasks: sorted_sub_tasks,
    //     //         opt_dependency: self.dependencies().iter().map(|x| x.sorted()).collect(),
    //     //         hr_req: self.hr(),
    //     //         min_start_date: self.min_start_date(),
    //     //         max_end_date: self.max_end_date(),
    //     //         max_expense: self.max_expense(),
    //     //     }
    //     // }
    // }
}
