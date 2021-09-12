mod object;

use super::Position;
use std::collections::{HashMap, HashSet};

struct Tree(Vec<Definition>);

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

#[derive(Debug, PartialEq, Clone)]
struct Object {
    position: Position,
    value: ObjectValue,
}

#[derive(Debug, PartialEq, Clone)]
enum ObjectValue {
    Aggregation(Vec<(String, Object)>),
    Composition(Vec<Object>),
    Pipe(Pipe),
    Literal(String),
    Variable(String),
}

#[derive(Debug, PartialEq, Clone)]
struct Pipe {
    input: Box<Object>,
    generator: Generator,
    output_is_dir: bool,
}

#[derive(Debug, PartialEq, Clone)]
enum Generator {
    Url(String),
    Path(String),
    Default(GeneratorDefault),
    Variable(String),
}

#[derive(Debug, PartialEq, Clone)]
struct GeneratorDefault {
    default_name: String,
    url: String,
}

impl Tree {
    /// Return the number of definition.
    fn number_of_definition(&self) -> usize {
        self.0.len()
    }

    /// Get a HastSet with URL of all external generators.
    pub fn generator_url_list<'a>(
        &'a self,
        default: &'a HashMap<String, String>,
    ) -> HashSet<&'a str> {
        let mut h = HashSet::new();

        self.0.iter().for_each(|def| {
            def.value.walk(|o| {
                match &o.value {
                    ObjectValue::Pipe(Pipe { generator, .. }) => match &generator {
                        Generator::Default(GeneratorDefault { default_name, url }) => {
                            h.insert(default.get(default_name).unwrap_or(url).as_str());
                        }
                        Generator::Url(s) => {
                            h.insert(s.as_str());
                        }
                        Generator::Variable(_) | Generator::Path(_) => {}
                    },
                    _ => {}
                };
            })
        });

        h
    }
}

#[test]
fn generator_url_list() {
    let mut default = HashMap::new();
    default.insert("bar".to_string(), "https://gen.exemple.com/bar".to_string());
    default.insert("foo".to_string(), "https://gen.exemple.com/foo".to_string());

    fn create_pipe(src: Object, gen: Generator) -> Object {
        Object {
            position: Position { line: 0, column: 0 },
            value: ObjectValue::Pipe(Pipe {
                input: Box::new(src),
                generator: gen,
                output_is_dir: true,
            }),
        }
    }

    let p = Position { line: 0, column: 0 };
    let root = create_pipe(
        create_pipe(
            create_pipe(
                Object {
                    position: p,
                    value: ObjectValue::Literal("literal".to_string()),
                },
                Generator::Default(GeneratorDefault {
                    default_name: "bar".to_string(),
                    url: "https://exemple.com/custom/bar".to_string(),
                }),
            ),
            Generator::Default(GeneratorDefault {
                default_name: "specificfoo".to_string(),
                url: "https://exemple.com/custom/foo".to_string(),
            }),
        ),
        Generator::Url("https://exemple.com/url".to_string()),
    );

    println!("root: {:#?}", root);

    let mut generators = HashSet::new();
    generators.insert("https://gen.exemple.com/bar");
    generators.insert("https://exemple.com/custom/foo");
    generators.insert("https://exemple.com/url");

    assert_eq!(
        generators,
        Tree(vec![Definition {
            position: p,
            key: DefinitionKey::SystemRelease,
            value: root,
        }])
        .generator_url_list(&default)
    );
}
