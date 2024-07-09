use chrono::NaiveDate;

use crate::business::account::{Account, LedgerTree};
use crate::business::period::Period;
use crate::business::transaction::Transaction;
use crate::functor::Functor;

use super::account::{is_desc_account, AccountLedger};

pub fn posting(
    carry_forward_ledger: &mut LedgerTree,
    current_period_transactions: Vec<Transaction>,
) {
    fn post_transaction_mut(ledger: &mut LedgerTree, t: Transaction) {
        // Debit posted
        ledger.update_ances_ledgers(t.debit_account, |x| {
            x.add_debit(t.credit_account, t.amount, t.date);
        });

        // Credit posted
        ledger.update_ances_ledgers(t.credit_account, |x| {
            x.add_credit(t.debit_account, t.amount, t.date);
        });
    }

    current_period_transactions
        .into_iter()
        .for_each(|t| post_transaction_mut(carry_forward_ledger, t));
}

fn end_of_period_adjustment(posted_ledger: &mut LedgerTree, period_ending_date: NaiveDate) {
    // todo

    // return posted_ledger;
}

fn close_temp_accounts(adjusted_ledger: &mut LedgerTree, period_ending_date: NaiveDate) {
    let mut expense_isl = LedgerTree::new(Account::Expense);
    let mut revenue_isl = LedgerTree::new(Account::Revenue);

    // let process_temp_ledger = |general_e_r: (LedgerTree, LedgerTree, LedgerTree),
    //                            temp_ledger: AccountLedger| {
    //     if is_desc_account(temp_ledger.account, Account::Expense) {
    //         let (general_ledger, expense_isl, revenue_isl) = general_e_r;

    //         let debit_amount = temp_ledger.debit_amount();

    //         let new_general = general_ledger.update_ledger(temp_ledger.account, |x| {
    //             x.add_credit(
    //                 Account::IncomeSummary,
    //                 debit_amount,
    //                 period_ending_date.clone(),
    //             )
    //         });

    //         let new_expense_isl = expense_isl.update_ledger(temp_ledger.account, |x| {
    //             x.add_debit(
    //                 Account::IncomeSummary,
    //                 debit_amount,
    //                 period_ending_date.clone(),
    //             )
    //         });

    //         (new_general, new_expense_isl, revenue_isl)
    //     } else if is_desc_account(temp_ledger.account, Account::Revenue) {
    //         let (general_ledger, expense_isl, revenue_isl) = general_e_r;

    //         let credit_amount = temp_ledger.credit_amount();

    //         let new_general = general_ledger.update_ledger(temp_ledger.account, |x| {
    //             x.add_debit(
    //                 Account::IncomeSummary,
    //                 credit_amount,
    //                 period_ending_date.clone(),
    //             )
    //         });

    //         let new_revenue_isl = revenue_isl.update_ledger(temp_ledger.account, |x| {
    //             x.add_credit(
    //                 Account::IncomeSummary,
    //                 credit_amount,
    //                 period_ending_date.clone(),
    //             )
    //         });

    //         (new_general, expense_isl, new_revenue_isl)
    //     } else {
    //         general_e_r
    //     }
    // };

    let process_temp_ledger = |temp_ledger: &AccountLedger| {
        if is_desc_account(temp_ledger.account, Account::Expense) {
            let debit_amount = temp_ledger.debit_amount();

            adjusted_ledger.update_ledger(temp_ledger.account, &|x| {
                x.add_credit(
                    Account::IncomeSummary,
                    debit_amount,
                    period_ending_date.clone(),
                );
            });

            expense_isl.update_ledger(temp_ledger.account, &|x| {
                x.add_debit(
                    Account::IncomeSummary,
                    debit_amount,
                    period_ending_date.clone(),
                );
            });
        } else if is_desc_account(temp_ledger.account, Account::Revenue) {
            let credit_amount = temp_ledger.credit_amount();

            adjusted_ledger.update_ledger(temp_ledger.account, &|x| {
                x.add_debit(
                    Account::IncomeSummary,
                    credit_amount,
                    period_ending_date.clone(),
                );
            });

            revenue_isl.update_ledger(temp_ledger.account, &|x| {
                x.add_credit(
                    Account::IncomeSummary,
                    credit_amount,
                    period_ending_date.clone(),
                );
            });
        } else {
        }
    };

    // todo Maybe optimize to remove the clone
    // let (isl_added_general, expense_isl, revenue_isl) = adjusted_ledger.clone().into_iter().fold(
    //     (adjusted_ledger, init_isl_expense, init_isl_revenue),
    //     process_temp_ledger,
    // );

    // adjusted_ledger.mut_iter().for_each(process_temp_ledger);
    adjusted_ledger.update(&process_temp_ledger);

    let expanse_amount = expense_isl.value.debit_amount();
    let revenue_amount = revenue_isl.value.credit_amount();

    if expanse_amount > revenue_amount {
        // Net loss
        adjusted_ledger.update_ances_ledgers(Account::RetainedEarnings, |x| {
            x.add_debit(
                Account::RetainedEarnings,
                expanse_amount - revenue_amount,
                period_ending_date.clone(),
            );
        });
    } else if revenue_amount > expanse_amount {
        // Net income
        adjusted_ledger.update_ances_ledgers(Account::RetainedEarnings, |x| {
            x.add_credit(
                Account::RetainedEarnings,
                revenue_amount - expanse_amount,
                period_ending_date.clone(),
            );
        });
    } else {
        // adjusted_ledger
    };

    // adjusted_ledger
}

fn close_permanent_accounts(
    temp_closed_general_ledger: LedgerTree,
    period_ending_date: NaiveDate,
) -> LedgerTree {
    let cloes_perm_ledger = |ledger: &AccountLedger| {
        if is_desc_account(ledger.account, Account::Asset)
            || is_desc_account(ledger.account, Account::Liability)
            || is_desc_account(ledger.account, Account::Equity)
        {
            let debit_amount = ledger.debit_amount();
            let credit_amount = ledger.credit_amount();

            if debit_amount > credit_amount {
                ledger.add_credit(
                    Account::EndingBalance,
                    debit_amount - credit_amount,
                    period_ending_date.clone(),
                )
            } else if credit_amount > debit_amount {
                ledger.add_debit(
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

    temp_closed_general_ledger.fmap(&cloes_perm_ledger)
}

fn close_ledger(posted_ledger: LedgerTree, period_ending_date: NaiveDate) -> LedgerTree {
    let adjusted_general_ledger = end_of_period_adjustment(posted_ledger, period_ending_date);
    let temp_closed_general_ledger =
        close_temp_accounts(adjusted_general_ledger, period_ending_date);
    close_permanent_accounts(temp_closed_general_ledger, period_ending_date)
}
