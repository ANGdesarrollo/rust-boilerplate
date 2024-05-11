pub trait CreateUseCaseTrait {
    async fn handle(payload: String) -> String;
}