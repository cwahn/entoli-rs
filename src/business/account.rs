use chrono::{offset, NaiveDate};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

pub fn sub_accounts(account: Account) -> Vec<Account> {
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

pub fn descendant_accounts(account: &Account) -> Vec<Account> {
    let mut accounts = vec![];
    let mut stack = vec![account.clone()];

    while let Some(account) = stack.pop() {
        accounts.push(account.clone());
        stack.extend(sub_accounts(account));
    }

    accounts
}

pub fn is_desc_account(account: Account, ref_account: Account) -> bool {
    let mut stack = vec![ref_account];

    while let Some(acc) = stack.pop() {
        if account == acc {
            return true;
        }

        stack.extend(sub_accounts(acc));
    }

    false
}

pub fn ancestor_accounts(account: Account) -> Vec<Account> {
    let mut path = vec![];
    let root = Account::AccountBase; // Assuming AccountBase is the root

    // Should be implemented with sub_accounts
    fn _find_ancestors(account: Account, target: Account, path: &mut Vec<Account>) -> bool {
        if account == target {
            return true;
        } else {
            for sub_account in sub_accounts(account) {
                if _find_ancestors(sub_account, target, path) {
                    path.push(account.clone());
                    return true;
                }
            }

            false
        }
    }

    _find_ancestors(root, account, &mut path);
    path.reverse(); // Reverse to get ancestors in correct order
    path
}

pub fn is_ancestor_account(account: Account, ref_account: Account) -> bool {
    let root = Account::AccountBase; // Assuming AccountBase is the root

    fn _is_ancestor_account(account: Account, ref_account: Account) -> bool {
        if account == ref_account {
            return true;
        } else {
            for sub_account in sub_accounts(account) {
                if _is_ancestor_account(sub_account, ref_account) {
                    return true;
                }
            }

            false
        }
    }

    _is_ancestor_account(root, ref_account)
}

pub struct AccountEntry {
    offset_accout: Account,
    amount: i64,
    date: NaiveDate,
}

pub struct AccountLedger {
    account: Account,
    debits: Vec<AccountEntry>,
    credits: Vec<AccountEntry>,
}

impl AccountLedger {
    pub fn new(account: Account) -> AccountLedger {
        AccountLedger {
            account,
            debits: vec![],
            credits: vec![],
        }
    }

    pub fn add_debit(&mut self, offset_accout: Account, amount: i64, date: NaiveDate) {
        self.debits.push(AccountEntry {
            offset_accout,
            amount,
            date,
        });
    }

    pub fn add_credit(&mut self, offset_accout: Account, amount: i64, date: NaiveDate) {
        self.credits.push(AccountEntry {
            offset_accout,
            amount,
            date,
        });
    }

    pub fn debit_amount(&self) -> i64 {
        self.debits.iter().map(|entry| entry.amount).sum()
    }

    pub fn credit_amount(&self) -> i64 {
        self.credits.iter().map(|entry| entry.amount).sum()
    }

    pub fn balance(&self) -> i64 {
        if is_desc_account(self.account, Account::Asset)
            || is_desc_account(self.account, Account::Expense)
        {
            self.debit_amount() - self.credit_amount()
        } else if is_desc_account(self.account, Account::Liability)
            || is_desc_account(self.account, Account::Equity)
            || is_desc_account(self.account, Account::Revenue)
        {
            self.credit_amount() - self.debit_amount()
        } else {
            panic!("Balance not supported for account: {:?}", self.account);
        }
    }

    pub fn is_balanced(&self) -> bool {
        self.balance() == 0
    }

    pub fn carry_forward(
        &mut self,
        ending_date: NaiveDate,
        beginning_date: NaiveDate,
    ) -> AccountLedger {
        let mut ledger = AccountLedger::new(self.account.clone());

        let debit_amount = self.debit_amount();
        let credit_amount = self.credit_amount();

        if debit_amount > credit_amount {
            self.add_credit(
                Account::EndingBalance,
                debit_amount - credit_amount,
                ending_date,
            );
            ledger.add_debit(
                Account::BeginningBalance,
                debit_amount - credit_amount,
                beginning_date,
            );

            ledger
        } else if credit_amount > debit_amount {
            self.add_debit(
                Account::EndingBalance,
                credit_amount - debit_amount,
                ending_date,
            );
            ledger.add_credit(
                Account::BeginningBalance,
                credit_amount - debit_amount,
                beginning_date,
            );

            ledger
        } else {
            ledger
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_accounts() {
        let account = Account::Account;
        let sub_accounts = sub_accounts(account);

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

    #[test]
    fn test_descendant_accounts() {
        let account = Account::TaxExpense;
        assert_eq!(
            descendant_accounts(&account),
            vec![
                Account::TaxExpense,
                Account::Vat,
                Account::CustomsDuty,
                Account::CorporateTax,
            ]
        );
    }

    #[test]
    fn test_is_desc_account() {
        assert_ne!(
            is_desc_account(Account::TaxExpense, Account::CorporateTax),
            true
        );
        assert_eq!(
            is_desc_account(Account::CorporateTax, Account::TaxExpense),
            true
        );

        assert_eq!(is_desc_account(Account::Cash, Account::Asset), true);
    }

    #[test]
    fn test_ancestor_accounts() {
        let account = Account::TaxExpense;
        assert_eq!(
            ancestor_accounts(account),
            vec![Account::AccountBase, Account::Account, Account::Expense]
        );
    }

    #[test]
    fn test_is_ancestor_account() {
        let account = Account::TaxExpense;
        let ref_account = Account::Account;
        assert_eq!(is_ancestor_account(account, ref_account), true);
    }

    #[test]
    fn test_account_ledger() {
        let mut ledger = AccountLedger::new(Account::Cash);
        ledger.add_debit(
            Account::BeginningBalance,
            100,
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );
        ledger.add_credit(
            Account::EndingBalance,
            100,
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        assert_eq!(ledger.debit_amount(), 100);
        assert_eq!(ledger.credit_amount(), 100);
        assert_eq!(ledger.balance(), 0);
        assert_eq!(ledger.is_balanced(), true);
    }

    #[test]
    fn test_account_ledger_carry_forward() {
        let mut ledger = AccountLedger::new(Account::Cash);
        ledger.add_credit(
            Account::Inventory,
            100,
            NaiveDate::from_ymd_opt(2020, 1, 15).unwrap(),
        );

        let next_ledger = ledger.carry_forward(
            NaiveDate::from_ymd_opt(2020, 1, 31).unwrap(),
            NaiveDate::from_ymd_opt(2020, 2, 1).unwrap(),
        );

        assert_eq!(ledger.debit_amount(), 100);
        assert_eq!(ledger.credit_amount(), 100);
        assert_eq!(next_ledger.debit_amount(), 0);
        assert_eq!(next_ledger.credit_amount(), 100);
    }
}
