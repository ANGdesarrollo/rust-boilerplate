use crate::modules::item::domain::use_cases::list_use_case_trait::ListUseCaseTrait;

pub struct ListUseCase {}

impl ListUseCaseTrait for ListUseCase {
    async fn handle() -> Vec<String> {
        let response: Vec<String> = vec![
            String::from("Hola"),
            String::from("Mundo"),
        ];

        return response;
    }
}

