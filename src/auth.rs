pub enum Auth {
    None,
    //PersonalToken,
}

impl Default for Auth {
    fn default() -> Self {
        Self::None
    }
}
