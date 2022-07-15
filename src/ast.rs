use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) enum AstNode {
    Add(Box<AstNode>, Box<AstNode>, f32),
    Sub(Box<AstNode>, Box<AstNode>, f32),
    Mul(Box<AstNode>, Box<AstNode>, f32),
    Div(Box<AstNode>, Box<AstNode>, f32),
    Num(f32),
}

impl AstNode {
    pub(crate) fn get_value(&self) -> f32 {
        match self {
            AstNode::Add(_, _, f)
            | AstNode::Sub(_, _, f)
            | AstNode::Mul(_, _, f)
            | AstNode::Div(_, _, f)
            | AstNode::Num(f) => *f,
        }
    }

    fn get_left(&self) -> Option<AstNode> {
        match self {
            AstNode::Add(f, _, _)
            | AstNode::Sub(f, _, _)
            | AstNode::Mul(f, _, _)
            | AstNode::Div(f, _, _) => Some(*f.clone()),
            _ => None,
        }
    }

    fn get_right(&self) -> Option<AstNode> {
        match self {
            AstNode::Add(_, f, _)
            | AstNode::Sub(_, f, _)
            | AstNode::Mul(_, f, _)
            | AstNode::Div(_, f, _) => Some(*f.clone()),
            _ => None,
        }
    }
}

impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn r(root: &AstNode, is_include: bool) -> String {
            match root {
                AstNode::Add(_, _, _) if is_include => format!(
                    "({}+{})",
                    r(&root.get_left().unwrap(), true),
                    r(&root.get_right().unwrap(), true)
                ),
                AstNode::Add(_, _, _) => format!(
                    "{}+{}",
                    r(&root.get_left().unwrap(), true),
                    r(&root.get_right().unwrap(), true)
                ),
                AstNode::Sub(_, _, _) if is_include => format!(
                    "({}-{})",
                    r(&root.get_left().unwrap(), true),
                    r(&root.get_right().unwrap(), true)
                ),
                AstNode::Sub(_, _, _) => format!(
                    "{}-{}",
                    r(&root.get_left().unwrap(), true),
                    r(&root.get_right().unwrap(), true)
                ),
                AstNode::Mul(_, _, _) => format!(
                    "{}*{}",
                    r(&root.get_left().unwrap(), true),
                    r(&root.get_right().unwrap(), true)
                ),
                AstNode::Div(_, _, _) => format!(
                    "{}/{}",
                    r(&root.get_left().unwrap(), true),
                    r(&root.get_right().unwrap(), true)
                ),
                AstNode::Num(v) => v.to_string(),
            }
        }
        write!(f, "{}={}", self.get_value(), r(self, false))
    }
}

