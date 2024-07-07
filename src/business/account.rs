use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Account {
    AccountBase,
    Account,
    PsuedoAccount,
    IncomeSummary,
    BeginningBalance,
    EndingBalance,
    Equity,
    PaidInCapital,
    RetainedEarnings,
    Asset,
    CurrentAsset,
    Cash,
    AccountReceivable,
    Inventory,
    PrepaidExpense,
    NonCurrentAsset,
    PropertyPlantEquipment,
    IntangibleAsset,
    Goodwill,
    Patent,
    Investment,
    Liability,
    CurrentLiability,
    UnearnedRevenue,
    AccountsPayable,
    ShortTermDebt,
    AccruedExpenses,
    NonCurrentLiability,
    LongTermDebt,
    DeferredTaxLiability,
    Revenue,
    SalesRevenue,
    ServiceRevenue,
    GovernmentGrantRevenue,
    InterestRevenue,
    RentalRevenue,
    Expense,
    Cogs,
    ProductCogs,
    ServiceCogs,
    DirectLaborCogs,
    ManufacturingOverheadCogs,
    Sga,
    Salary,
    RndExpense,
    WelfareExpense,
    TravelExpense,
    SalesExpense,
    ElectricityExpense,
    RentExpense,
    PromotionExpense,
    SalesCommissionExpense,
    CommissionExpense,
    DepreciationAndAmortization,
    DepreciationExpense,
    AmortizationExpense,
    InterestExpense,
    InterestOnLoan,
    TaxExpense,
    CorporateTax,
    CustomsDuty,
    Vat,
}

pub fn sub_accounts(account: &Account) -> Vec<Account> {
    match account {
        Account::AccountBase => vec![Account::Account, Account::PsuedoAccount],
        Account::Account => vec![
            Account::Equity,
            Account::Asset,
            Account::Liability,
            Account::Revenue,
            Account::Expense,
        ],
        Account::Equity => vec![Account::PaidInCapital, Account::RetainedEarnings],
        Account::Asset => vec![Account::CurrentAsset, Account::NonCurrentAsset],
        Account::CurrentAsset => vec![
            Account::Cash,
            Account::AccountReceivable,
            Account::Inventory,
            Account::PrepaidExpense,
        ],
        Account::NonCurrentAsset => vec![
            Account::PropertyPlantEquipment,
            Account::IntangibleAsset,
            Account::Investment,
        ],
        Account::IntangibleAsset => vec![Account::Goodwill, Account::Patent],
        Account::Liability => vec![Account::CurrentLiability, Account::NonCurrentLiability],
        Account::CurrentLiability => vec![
            Account::UnearnedRevenue,
            Account::AccountsPayable,
            Account::ShortTermDebt,
            Account::AccruedExpenses,
        ],
        Account::NonCurrentLiability => vec![Account::LongTermDebt, Account::DeferredTaxLiability],
        Account::Revenue => vec![
            Account::SalesRevenue,
            Account::ServiceRevenue,
            Account::GovernmentGrantRevenue,
            Account::InterestRevenue,
            Account::RentalRevenue,
        ],
        Account::Expense => vec![
            Account::Cogs,
            Account::Sga,
            Account::DepreciationAndAmortization,
            Account::InterestExpense,
            Account::TaxExpense,
        ],
        Account::Cogs => vec![
            Account::ProductCogs,
            Account::ServiceCogs,
            Account::DirectLaborCogs,
            Account::ManufacturingOverheadCogs,
        ],
        Account::Sga => vec![
            Account::Salary,
            Account::RndExpense,
            Account::WelfareExpense,
            Account::TravelExpense,
            Account::SalesExpense,
            Account::ElectricityExpense,
            Account::RentExpense,
            Account::PromotionExpense,
            Account::SalesCommissionExpense,
            Account::CommissionExpense,
        ],
        Account::DepreciationAndAmortization => {
            vec![Account::DepreciationExpense, Account::AmortizationExpense]
        }
        Account::InterestExpense => vec![Account::InterestOnLoan],
        Account::TaxExpense => vec![Account::CorporateTax, Account::CustomsDuty, Account::Vat],
        Account::PsuedoAccount => vec![
            Account::IncomeSummary,
            Account::BeginningBalance,
            Account::EndingBalance,
        ],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_accounts() {
        let account = Account::Account;
        let sub_accounts = sub_accounts(&account);

        assert_eq!(
            sub_accounts,
            vec![
                Account::Equity,
                Account::Asset,
                Account::Liability,
                Account::Revenue,
                Account::Expense,
            ]
        );
    }
}
