#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    RegistrySetup,
    Confirmation,
    EnvSetup,
    UpdateList,
    UpdatePulling,
    Installing,
    Success,
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuSelection {
    Proceed,
    GenerateEnv,
    UpdateToken,
    CheckUpdates,
    Cancel,
}
