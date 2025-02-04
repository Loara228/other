
#[derive(Clone)]
pub struct ModalModel {
    /// доп текст
    pub input1: String,

    /// заявок на кредит
    pub input2: String,

    /// кредитов выдано
    pub input3: String,

    /// Регистраций 1С
    pub input4: String,

    /// Адаптер
    pub input5: String,

    /// ДС
    pub input6: String,
}

impl Default for ModalModel {
    fn default() -> Self {
        Self {
            input1: String::default(),
            input2: String::from("0"),
            input3: String::from("0"),
            input4: String::from("0"),
            input5: String::from("0"),
            input6: String::from("0"),
        }
    }
}