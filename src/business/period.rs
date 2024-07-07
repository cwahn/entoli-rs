use chrono::prelude::*;
use chrono::Duration;
use chrono::NaiveDate;

#[derive(Debug)]
pub enum Period {
    Day,
    Week,
    Month,
    Quarter,
    Half,
    Year,
}

pub fn accounting_periods(
    beginning: NaiveDate,
    ending: NaiveDate,
    period: Period,
) -> Vec<(NaiveDate, NaiveDate)> {
    match period {
        Period::Day => {
            let mut current = beginning;
            let mut result = vec![];

            while current <= ending {
                result.push((current, current));
                current = current.succ_opt().unwrap();
            }

            result
        }
        Period::Week => {
            let mut current = adjust_to_start_of_week(beginning);
            let mut result = vec![];

            while current <= ending {
                let next = current + Duration::weeks(1);
                result.push((current, next.pred_opt().unwrap()));
                current = next;
            }

            result
        }
        Period::Month => {
            let mut current = adjust_to_start_of_month(beginning);
            let mut result = vec![];

            while current <= ending {
                let next = add_months(current, 1);
                result.push((current, next.pred_opt().unwrap()));
                current = next;
            }

            result
        }
        Period::Quarter => {
            let mut current = adjust_to_start_of_quarter(beginning);
            let mut result = vec![];

            while current <= ending {
                let next = add_months(current, 3);
                result.push((current, next.pred_opt().unwrap()));
                current = next;
            }

            result
        }
        Period::Half => {
            let mut current = adjust_to_start_of_half_year(beginning);
            let mut result = vec![];

            while current <= ending {
                let next = add_months(current, 6);
                result.push((current, next.pred_opt().unwrap()));
                current = next;
            }

            result
        }
        Period::Year => {
            let mut current = adjust_to_start_of_year(beginning);
            let mut result = vec![];

            while current <= ending {
                let next = current.with_year(current.year() + 1).unwrap();
                result.push((current, next.pred_opt().unwrap()));
                current = next;
            }

            result
        }
    }
}

fn adjust_to_start_of_week(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday().num_days_from_monday();
    date - Duration::days(weekday as i64)
}

fn adjust_to_start_of_month(date: NaiveDate) -> NaiveDate {
    date.with_day(1).unwrap()
}

fn adjust_to_start_of_quarter(date: NaiveDate) -> NaiveDate {
    let month = date.month();
    let start_month = match month {
        1..=3 => 1,
        4..=6 => 4,
        7..=9 => 7,
        10..=12 => 10,
        _ => unreachable!(),
    };
    date.with_month(start_month).unwrap().with_day(1).unwrap()
}

fn adjust_to_start_of_half_year(date: NaiveDate) -> NaiveDate {
    let month = date.month();
    let start_month = if month <= 6 { 1 } else { 7 };
    date.with_month(start_month).unwrap().with_day(1).unwrap()
}

fn adjust_to_start_of_year(date: NaiveDate) -> NaiveDate {
    date.with_month(1).unwrap().with_day(1).unwrap()
}

fn add_months(date: NaiveDate, months: u32) -> NaiveDate {
    let new_month = (date.month0() + months) % 12 + 1;
    let mut new_year = date.year() + (date.month0() + months) as i32 / 12;
    let mut new_date = NaiveDate::from_ymd_opt(new_year, new_month, 1).unwrap();

    while new_date < date {
        new_year += 1;
        new_date = NaiveDate::from_ymd_opt(new_year, new_month, 1).unwrap();
    }

    new_date
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accounting_periods_day() {
        let beginning = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let ending = NaiveDate::from_ymd_opt(2020, 1, 3).unwrap();
        let periods = accounting_periods(beginning, ending, Period::Day);

        assert_eq!(
            periods,
            vec![
                (
                    NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()
                ),
                (
                    NaiveDate::from_ymd_opt(2020, 1, 2).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 1, 2).unwrap()
                ),
                (
                    NaiveDate::from_ymd_opt(2020, 1, 3).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 1, 3).unwrap()
                ),
            ]
        );
    }

    #[test]
    fn test_accounting_periods_week() {
        let beginning = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let ending = NaiveDate::from_ymd_opt(2020, 1, 21).unwrap();
        let periods = accounting_periods(beginning, ending, Period::Week);

        assert_eq!(
            periods,
            vec![
                (
                    NaiveDate::from_ymd_opt(2019, 12, 30).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 1, 5).unwrap()
                ),
                (
                    NaiveDate::from_ymd_opt(2020, 1, 6).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 1, 12).unwrap()
                ),
                (
                    NaiveDate::from_ymd_opt(2020, 1, 13).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 1, 19).unwrap()
                ),
                (
                    NaiveDate::from_ymd_opt(2020, 1, 20).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 1, 26).unwrap()
                ),
            ]
        );
    }

    #[test]
    fn test_accounting_periods_month() {
        let beginning = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let ending = NaiveDate::from_ymd_opt(2020, 3, 21).unwrap();
        let periods = accounting_periods(beginning, ending, Period::Month);

        assert_eq!(
            periods,
            vec![
                (
                    NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 1, 31).unwrap()
                ),
                (
                    NaiveDate::from_ymd_opt(2020, 2, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 2, 29).unwrap()
                ),
                (
                    NaiveDate::from_ymd_opt(2020, 3, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 3, 31).unwrap()
                ),
            ]
        );
    }

    #[test]
    fn test_accounting_periods_quarter() {
        let beginning = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let ending = NaiveDate::from_ymd_opt(2020, 9, 21).unwrap();
        let periods = accounting_periods(beginning, ending, Period::Quarter);

        assert_eq!(
            periods,
            vec![
                (
                    NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 3, 31).unwrap()
                ),
                (
                    NaiveDate::from_ymd_opt(2020, 4, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 6, 30).unwrap()
                ),
                (
                    NaiveDate::from_ymd_opt(2020, 7, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 9, 30).unwrap()
                ),
            ]
        );
    }

    #[test]
    fn test_accounting_periods_half() {
        let beginning = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let ending = NaiveDate::from_ymd_opt(2021, 9, 21).unwrap();
        let periods = accounting_periods(beginning, ending, Period::Half);

        assert_eq!(
            periods,
            vec![
                (
                    NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 6, 30).unwrap()
                ),
                (
                    NaiveDate::from_ymd_opt(2020, 7, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 12, 31).unwrap()
                ),
                (
                    NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2021, 6, 30).unwrap()
                ),
                (
                    NaiveDate::from_ymd_opt(2021, 7, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2021, 12, 31).unwrap()
                ),
            ]
        );
    }

    #[test]
    fn test_accounting_periods_year() {
        let beginning = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let ending = NaiveDate::from_ymd_opt(2022, 9, 21).unwrap();
        let periods = accounting_periods(beginning, ending, Period::Year);

        assert_eq!(
            periods,
            vec![
                (
                    NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2020, 12, 31).unwrap()
                ),
                (
                    NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2021, 12, 31).unwrap()
                ),
                (
                    NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2022, 12, 31).unwrap()
                ),
            ]
        );
    }
}
