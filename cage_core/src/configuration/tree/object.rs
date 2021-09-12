use super::{Object, ObjectValue, Pipe};

impl Object {
    /// Iter over object and sub child object.
    pub fn collect(&self) -> Vec<&Object> {
        let mut v = Vec::new();
        self.walk_inter(&mut |o| v.push(o));
        v
    }
    /// Walk over each objet into the objkect tree, and for each call the closure f.
    pub fn walk<'a, F>(&'a self, mut f: F)
    where
        F: FnMut(&'a Object),
    {
        self.walk_inter(&mut f);
    }

    fn walk_inter<'a, F>(&'a self, f: &mut F)
    where
        F: FnMut(&'a Object),
    {
        f(self);
        match &self.value {
            ObjectValue::Aggregation(list) => list.iter().for_each(move |(_, o)| o.walk_inter(f)),
            ObjectValue::Composition(list) => list.iter().for_each(move |o| o.walk_inter(f)),
            ObjectValue::Pipe(Pipe { input, .. }) => input.walk_inter(f),
            ObjectValue::Literal(_) | ObjectValue::Variable(_) => {}
        };
    }
}

#[test]
fn test_object_iter() {
    use super::{Generator, ObjectValue, Pipe, Position};

    let p = Position { line: 0, column: 0 };
    let pipe_src = Object {
        position: p,
        value: ObjectValue::Literal(String::from("source")),
    };
    let aggregation = Object {
        position: p,
        value: ObjectValue::Literal(String::from("literal string")),
    };
    let composition_intern = vec![
        Object {
            position: p,
            value: ObjectValue::Variable(String::from("variable")),
        },
        Object {
            position: p,
            value: ObjectValue::Literal(String::from("literal string")),
        },
        Object {
            position: p,
            value: ObjectValue::Pipe(Pipe {
                input: Box::new(pipe_src.clone()),
                generator: Generator::Variable(String::from("var")),
                output_is_dir: true,
            }),
        },
        Object {
            position: p,
            value: ObjectValue::Aggregation(vec![(String::from("foo/bar/"), aggregation.clone())]),
        },
    ];
    let root = Object {
        position: p,
        value: ObjectValue::Composition(composition_intern.clone()),
    };

    println!("root: {:#?}", root);

    let mut iter = root.collect().into_iter();
    assert_eq!(Some(&root), iter.next());
    assert_eq!(Some(&composition_intern[0]), iter.next());
    assert_eq!(Some(&composition_intern[1]), iter.next());
    assert_eq!(Some(&composition_intern[2]), iter.next());
    assert_eq!(Some(&pipe_src), iter.next());
    assert_eq!(Some(&composition_intern[3]), iter.next());
    assert_eq!(Some(&aggregation), iter.next());
    assert_eq!(None, iter.next());
}
