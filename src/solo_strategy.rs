
pub mod strategy {
    use crate::actions::action::Action;

    pub struct SoloStrategy {
    }
    impl SoloStrategy {
        pub fn choose_action(&self) -> Option<impl Action> {
            None
        }
    }
}
