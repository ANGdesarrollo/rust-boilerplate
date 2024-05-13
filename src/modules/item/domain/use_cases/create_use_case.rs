use crate::modules::item::domain::use_cases::create_use_case_trait::CreateUseCaseTrait;
pub struct CreateUseCase {}

impl CreateUseCaseTrait for CreateUseCase {
    async fn handle(payload: String) -> String {
        let response: String = String::from(payload);
        return response;
    }
}

