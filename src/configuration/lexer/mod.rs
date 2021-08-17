mod char_iter;
mod simple;
mod word;

use super::Position;

#[derive(Debug)]
pub enum Word {
    Token(String),
    Url(String),
    Colon,
    DefinitionSign,
    Coma,

    DirectoryCompositionBegin,
    DirectoryCompositionEnd,
    DirectoryConcatenateBegin,
    DirectoryConcatenateEnd,

    PipeDirectory2Directory,
    PipeDirectory2File,
    PipeFile2Directory,
    PipeFile2File,

    // for format
    WhiteSpace(String),
}

// mettre numéro de ligne et de caractère
pub fn lexer(_src: &str) -> impl Iterator<Item = Word> {
    (0..1).map(|_| Word::DirectoryCompositionBegin)
}
