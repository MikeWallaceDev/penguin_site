pub mod app;
pub mod assembly;
pub mod contact;
pub mod home;
pub mod i18n_provider;
pub mod training;
pub mod tune_ups;

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple_logic() {
        assert_eq!(1 + 1, 2);
    }
}
