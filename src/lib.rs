struct Graph<'a> {
    root: &'a dyn Node,
}

impl<'a> Graph<'a> {
    fn build(root: &'a dyn Node) -> Graph {
        Graph { root }
    }

    fn class_diagram(&self) -> Option<String> {
        let mut mermaid = String::new();
        mermaid.push_str("class_diagram\n");
        let mut nodes = vec![];
        let mut edges = vec![];
        let mut queue = vec![self.root];
        while let Some(node) = queue.pop() {
            nodes.push(node.node_name().to_string());
            if let Some(children) = node.children() {
                for child in children {
                    edges.push(format!("{} <|.. {}", node.node_name(), child.node_name()));
                    queue.push(child);
                }
            }
        }
        mermaid.push_str(&format!("    {}\n", edges.join("\n    ")));
        for node in nodes {
            mermaid.push_str(&format!("class {} {{ }}\n", node));
        }
        Some(mermaid)
    }
}

trait Node {
    fn children(&self) -> Option<Vec<&dyn Node>>;
    fn node_name(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestNode<'a> {
        children: Vec<&'a TestNode<'a>>,
        node_name: String,
    }

    impl Node for TestNode<'_> {
        fn node_name(&self) -> &str {
            self.node_name.as_str()
        }

        fn children(&self) -> Option<Vec<&dyn Node>> {
            Some(self.children.iter().map(|x| *x as &dyn Node).collect())
        }
    }

    #[test]
    fn it_works() {
        let mut parent = TestNode {
            children: vec![],
            node_name: "Parent".to_string(),
        };

        let child = TestNode {
            children: vec![],
            node_name: "Child".to_string(),
        };

        parent.children.push(&child);

        let graph = Graph::build(&parent);
        let mermaid = graph.class_diagram().unwrap();

        println!("{}", &mermaid);
        assert_eq!(
            mermaid,
            r#"class_diagram
    Parent <|.. Child
class Parent { }
class Child { }
"#
        )
    }
}
