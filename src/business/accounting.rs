use chrono::NaiveDate;

use crate::business::account::{Account, LedgerTree};
use crate::business::period::Period;
use crate::business::transaction::Transaction;
use crate::data::functor::Functor;

use super::account::{is_desc_account, AccountLedger};

pub fn posting(
    carry_forward_ledger: LedgerTree,
    current_period_transactions: Vec<Transaction>,
) -> LedgerTree {
    fn transaction_posted(ledger: LedgerTree, t: Transaction) -> LedgerTree {
        // Post debit and credit to the ancestor ledgers
        ledger
            .fmap_ances_ledgers(t.debit_account, |x| {
                x.debit_added(t.credit_account, t.amount, t.date)
            })
            .fmap_ances_ledgers(t.credit_account, |x| {
                x.credit_added(t.debit_account, t.amount, t.date)
            })
    }

    current_period_transactions
        .into_iter()
        .fold(carry_forward_ledger, transaction_posted)
}

fn end_of_period_adjustment(
    posted_ledger: LedgerTree,
    period_ending_date: NaiveDate,
) -> LedgerTree {
    // todo

    posted_ledger
}

fn close_temp_accounts(adjusted_ledger: LedgerTree, period_ending_date: NaiveDate) -> LedgerTree {
    let process_temp_ledger = |general_e_r: (LedgerTree, LedgerTree, LedgerTree),
                               temp_ledger: AccountLedger| {
        if is_desc_account(temp_ledger.account, Account::Expense) {
            let (general_ledger, expense_isl, revenue_isl) = general_e_r;

            let debit_amount = temp_ledger.debit_amount();

            let new_general = general_ledger.fmap_ledger(temp_ledger.account, |x| {
                x.credit_added(
                    Account::IncomeSummary,
                    debit_amount,
                    period_ending_date.clone(),
                )
            });

            let new_expense_isl = expense_isl.fmap_ledger(temp_ledger.account, |x| {
                x.debit_added(
                    Account::IncomeSummary,
                    debit_amount,
                    period_ending_date.clone(),
                )
            });

            (new_general, new_expense_isl, revenue_isl)
        } else if is_desc_account(temp_ledger.account, Account::Revenue) {
            let (general_ledger, expense_isl, revenue_isl) = general_e_r;

            let credit_amount = temp_ledger.credit_amount();

            let new_general = general_ledger.fmap_ledger(temp_ledger.account, |x| {
                x.debit_added(
                    Account::IncomeSummary,
                    credit_amount,
                    period_ending_date.clone(),
                )
            });

            let new_revenue_isl = revenue_isl.fmap_ledger(temp_ledger.account, |x| {
                x.credit_added(
                    Account::IncomeSummary,
                    credit_amount,
                    period_ending_date.clone(),
                )
            });

            (new_general, expense_isl, new_revenue_isl)
        } else {
            general_e_r
        }
    };

    let init_isl_expense = LedgerTree::new(Account::Expense);
    let init_isl_revenue = LedgerTree::new(Account::Revenue);

    let (isl_added_general, expense_isl, revenue_isl) = adjusted_ledger.clone().into_iter().fold(
        (adjusted_ledger, init_isl_expense, init_isl_revenue),
        process_temp_ledger,
    );

    let expanse_amount = expense_isl.value.debit_amount();
    let revenue_amount = revenue_isl.value.credit_amount();

    if expanse_amount > revenue_amount {
        // Net loss
        isl_added_general.fmap_ances_ledgers(Account::RetainedEarnings, |x| {
            x.debit_added(
                Account::RetainedEarnings,
                expanse_amount - revenue_amount,
                period_ending_date.clone(),
            )
        })
    } else if revenue_amount > expanse_amount {
        // Net income
        isl_added_general.fmap_ances_ledgers(Account::RetainedEarnings, |x| {
            x.credit_added(
                Account::RetainedEarnings,
                revenue_amount - expanse_amount,
                period_ending_date.clone(),
            )
        })
    } else {
        isl_added_general
    }

    // adjusted_ledger
}

fn close_permanent_accounts(
    temp_closed_general_ledger: LedgerTree,
    period_ending_date: NaiveDate,
) -> LedgerTree {
    let cloes_perm_ledger = |ledger: AccountLedger| {
        if is_desc_account(ledger.account, Account::Asset)
            || is_desc_account(ledger.account, Account::Liability)
            || is_desc_account(ledger.account, Account::Equity)
        {
            let debit_amount = ledger.debit_amount();
            let credit_amount = ledger.credit_amount();

            if debit_amount > credit_amount {
                ledger.credit_added(
                    Account::EndingBalance,
                    debit_amount - credit_amount,
                    period_ending_date.clone(),
                )
            } else if credit_amount > debit_amount {
                ledger.debit_added(
                    Account::EndingBalance,
                    credit_amount - debit_amount,
                    period_ending_date.clone(),
                )
            } else {
                ledger
            }
        } else {
            ledger
        }
    };

    temp_closed_general_ledger.fmap1(&cloes_perm_ledger)
}

fn close_ledger(posted_ledger: LedgerTree, period_ending_date: NaiveDate) -> LedgerTree {
    let adjusted_general_ledger = end_of_period_adjustment(posted_ledger, period_ending_date);
    let temp_closed_general_ledger =
        close_temp_accounts(adjusted_general_ledger, period_ending_date);
    close_permanent_accounts(temp_closed_general_ledger, period_ending_date)
}
