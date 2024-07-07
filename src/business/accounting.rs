use chrono::NaiveDate;

use crate::business::account::{Account, LedgerTree};
use crate::business::period::Period;
use crate::business::transaction::Transaction;

pub fn posting(
    carry_forward_ledger: LedgerTree,
    current_period_transactions: Vec<Transaction>,
) -> LedgerTree {
    fn post_transaction(ledger: LedgerTree, t: Transaction) -> LedgerTree {
        let debit_posted = ledger.map_ances_ledgers(t.debit_account, |x| {
            x.debit_added(
                t.credit_account,
                t.amount,
                t.date,
            )
        });

        let credit_posted = debit_posted.map_ances_ledgers(t.credit_account, |x| {
            x.credit_added(
                t.debit_account,
                t.amount,
                t.date,
            )
        });

        credit_posted
    }

    current_period_transactions
        .into_iter()
        .fold(carry_forward_ledger, post_transaction)
}


// fn close_temp_accounts(
//     posted_ledger: LedgerTree,
//     period_ending_date: NaiveDate,
// ) -> LedgerTree {
//     let init_isl_expense = LedgerTree::new(Account::Expense);
//     let init_isl_revenue = LedgerTree::new(Account::Revenue);

//     let expense_ledgers = posted_ledger.flatten().filter(|x| x.account == Account::Expense);

// }
