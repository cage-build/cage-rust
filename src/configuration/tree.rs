use super::Position;

struct Tree(Vec<Definition>);

impl Tree {
    fn len(&self) -> usize {
        self.0.len()
    }
}

struct Definition {
    position: Position,
    key: DefinitionKey,
    value: Object,
}

#[derive(Debug)]
enum DefinitionKey {
    SystemRelease,
    SystemRun,
    SystemTest,
    Variable(String),
}

#[derive(Debug)]
struct Object {
    position: Position,
    value: ObjectValue,
}

#[derive(Debug)]
enum ObjectValue {
    Aggregation(Vec<(String, Object)>),
    Composition(Vec<Object>),
    PipeDirectory(Box<Object>),
    PipeFile(Box<Object>),
    Literal,
    Variable(String),
}

struct Generator {
    position: Position,
    value: GeneratorValue,
}

#[derive(Debug)]
enum GeneratorValue {
    Url(String),
    Default(GeneratorDefault),
    Variable(String),
}

#[derive(Debug)]
struct GeneratorDefault {
    default_name: String,
    url: String,
}
