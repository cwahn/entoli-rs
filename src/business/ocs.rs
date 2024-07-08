#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Ocs {
    Ceo,
    Cto,
    Coo,
    Developer,
    CppDeveloper,
    PythonDeveloper,
    BackendDeveloper,
    FrontendDeveloper,
    AiDeveloper,
    Engineer,
    MechanicalEngineer,
    ElectronicalEngineer,
    Sales,
    Marketer,
    MarketingManager,
    UiDesigner,
    CustomerService,
    Admin,
    Service,
    Trainer,
}

pub fn sub_occupations(occupation: &Ocs) -> Vec<Ocs> {
    match occupation {
        Ocs::Developer => vec![
            Ocs::CppDeveloper,
            Ocs::PythonDeveloper,
            Ocs::BackendDeveloper,
            Ocs::FrontendDeveloper,
            Ocs::AiDeveloper,
        ],
        Ocs::Engineer => vec![Ocs::MechanicalEngineer, Ocs::ElectronicalEngineer],
        Ocs::Service => vec![Ocs::Trainer],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_occupations() {
        assert_eq!(
            sub_occupations(&Ocs::Developer),
            vec![
                Ocs::CppDeveloper,
                Ocs::PythonDeveloper,
                Ocs::BackendDeveloper,
                Ocs::FrontendDeveloper,
                Ocs::AiDeveloper,
            ]
        );
        assert_eq!(
            sub_occupations(&Ocs::Engineer),
            vec![Ocs::MechanicalEngineer, Ocs::ElectronicalEngineer,]
        );
        assert_eq!(sub_occupations(&Ocs::Service), vec![Ocs::Trainer,]);
    }
}
