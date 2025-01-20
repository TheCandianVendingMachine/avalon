use miniserde::{ Deserialize, Serialize };

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Language {
    English
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Text {
    pub language: Language
}

impl std::fmt::Display for Language {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Language::English => "English".fmt(formatter)
        }
    }
}
