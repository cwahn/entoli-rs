use crate::business::account::Account;
use chrono::NaiveDate;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TransactionType {
    OperatingTransaction,
    InvestingTransaction,
    FinancingTransaction,
    CustomerTransaction,
    RevenueTransaction,
    SupplierTransaction,
    EmployeeTransaction,
    OperatingExpenseTransaction,
    InterestTransaction,
    TaxTransaction,
    PropertyPlantEquipmentTransaction,
    InvestmentTransaction,
    LoanDebtTransaction,
    ShareIssuanceTransaction,
    DebtIssuanceTransaction,
    BorrowingTransaction,
    ShareRepurchaseTransaction,
    DividendTransaction,
    LeaseTransaction,
}

pub fn sub_transactions(transaction: &TransactionType) -> Vec<TransactionType> {
    match transaction {
        TransactionType::OperatingTransaction => vec![
            TransactionType::CustomerTransaction,
            TransactionType::RevenueTransaction,
            TransactionType::SupplierTransaction,
            TransactionType::EmployeeTransaction,
            TransactionType::OperatingExpenseTransaction,
            TransactionType::InterestTransaction,
            TransactionType::TaxTransaction,
        ],
        TransactionType::InvestingTransaction => vec![
            TransactionType::PropertyPlantEquipmentTransaction,
            TransactionType::InvestmentTransaction,
            TransactionType::LoanDebtTransaction,
        ],
        TransactionType::FinancingTransaction => vec![
            TransactionType::ShareIssuanceTransaction,
            TransactionType::DebtIssuanceTransaction,
            TransactionType::BorrowingTransaction,
            TransactionType::ShareRepurchaseTransaction,
            TransactionType::DividendTransaction,
            TransactionType::LeaseTransaction,
        ],
        _ => vec![],
    }
}

pub struct Transaction {
    pub trans_type: TransactionType,
    pub debit_account: Account,
    pub credit_account: Account,
    pub amount: i64,
    pub date: NaiveDate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_transactions() {
        assert_eq!(
            sub_transactions(&TransactionType::InvestingTransaction),
            vec![
                TransactionType::PropertyPlantEquipmentTransaction,
                TransactionType::InvestmentTransaction,
                TransactionType::LoanDebtTransaction,
            ]
        )
    }
}
