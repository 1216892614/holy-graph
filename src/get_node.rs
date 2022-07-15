use crate::{ast, prime_match};

pub(crate) fn dfs_one(d6_results: &Vec<f32>, level: usize) -> Option<Vec<ast::AstNode>> {
    fn r(rest: Vec<ast::AstNode>, target: [f32; 3]) -> Option<ast::AstNode> {
        if rest.len() == 1
            && rest
                .iter()
                .any(|r| target.iter().any(|t| float_eq(&r.get_value(), t)))
        {
            return Some(rest[0].clone());
        } else {
            for an in 0..(rest.len() - 1) {
                for bn in (an + 1)..rest.len() {
                    let mut rest = rest.clone();
                    let (a, b) = (rest[an].clone(), rest[bn].clone());
                    let rest = {
                        rest.remove(bn);
                        rest.remove(an);
                        rest
                    };

                    return Some(
                        {
                            let mut rest = rest.clone();
                            let (a, b) = (a.clone(), b.clone());
                            rest.push(ast::AstNode::Add(
                                Box::new(a.clone()),
                                Box::new(b.clone()),
                                a.get_value() + b.get_value(),
                            ));
                            r(rest, target)
                        }
                        .unwrap_or(
                            {
                                let mut rest = rest.clone();
                                let (a, b) = (a.clone(), b.clone());
                                rest.push(ast::AstNode::Sub(
                                    Box::new(a.clone()),
                                    Box::new(b.clone()),
                                    a.get_value() - b.get_value(),
                                ));
                                r(rest, target)
                            }
                            .unwrap_or(
                                {
                                    let mut rest = rest.clone();
                                    let (a, b) = (a.clone(), b.clone());
                                    rest.push(ast::AstNode::Sub(
                                        Box::new(b.clone()),
                                        Box::new(a.clone()),
                                        b.get_value() - a.get_value(),
                                    ));
                                    r(rest, target)
                                }
                                .unwrap_or(
                                    {
                                        let mut rest = rest.clone();
                                        let (a, b) = (a.clone(), b.clone());
                                        rest.push(ast::AstNode::Mul(
                                            Box::new(a.clone()),
                                            Box::new(b.clone()),
                                            a.get_value() * b.get_value(),
                                        ));
                                        r(rest, target)
                                    }
                                    .unwrap_or(
                                        {
                                            if b.get_value() >= 0.000002 {
                                                let mut rest = rest.clone();
                                                let (a, b) = (a.clone(), b.clone());
                                                rest.push(ast::AstNode::Div(
                                                    Box::new(a.clone()),
                                                    Box::new(b.clone()),
                                                    a.get_value() / b.get_value(),
                                                ));
                                                r(rest, target)
                                            } else {
                                                None
                                            }
                                        }
                                        .unwrap_or(match {
                                            if a.get_value() >= 0.000002 {
                                                let mut rest = rest.clone();
                                                let (a, b) = (a.clone(), b.clone());
                                                rest.push(ast::AstNode::Div(
                                                    Box::new(b.clone()),
                                                    Box::new(a.clone()),
                                                    b.get_value() / a.get_value(),
                                                ));
                                                r(rest, target)
                                            } else {
                                                None
                                            }
                                        } {
                                            Some(a) => a,
                                            None => continue,
                                        }),
                                    ),
                                ),
                            ),
                        ),
                    );
                }
            }
            None
        }
    }

    match r(
        d6_results.iter().map(|i| ast::AstNode::Num(*i)).collect(),
        prime_match::get_prime_by_level(level),
    ) {
        Some(a) => Some(vec![a]),
        None => None,
    }
}

pub(crate) fn dfs_all(d6_results: &Vec<f32>, level: usize) -> Option<Vec<ast::AstNode>> {
    fn r(rest: Vec<ast::AstNode>, target: [f32; 3]) -> Option<Vec<ast::AstNode>> {
        if rest.len() == 1
            && rest
                .iter()
                .any(|r| target.iter().any(|t| float_eq(&r.get_value(), t)))
        {
            return Some(rest);
        } else {
            let mut result = Vec::new();
            for an in 0..(rest.len() - 1) {
                for bn in (an + 1)..rest.len() {
                    let mut rest = rest.clone();
                    let (a, b) = (rest[an].clone(), rest[bn].clone());
                    let rest = {
                        rest.remove(bn);
                        rest.remove(an);
                        rest
                    };

                    result.append(
                        &mut (vec![
                            {
                                let mut rest = rest.clone();
                                let (a, b) = (a.clone(), b.clone());
                                rest.push(ast::AstNode::Add(
                                    Box::new(a.clone()),
                                    Box::new(b.clone()),
                                    a.get_value() + b.get_value(),
                                ));
                                r(rest, target)
                            },
                            {
                                let mut rest = rest.clone();
                                let (a, b) = (a.clone(), b.clone());
                                rest.push(ast::AstNode::Sub(
                                    Box::new(a.clone()),
                                    Box::new(b.clone()),
                                    a.get_value() - b.get_value(),
                                ));
                                r(rest, target)
                            },
                            {
                                let mut rest = rest.clone();
                                let (a, b) = (a.clone(), b.clone());
                                rest.push(ast::AstNode::Sub(
                                    Box::new(b.clone()),
                                    Box::new(a.clone()),
                                    b.get_value() - a.get_value(),
                                ));
                                r(rest, target)
                            },
                            {
                                let mut rest = rest.clone();
                                let (a, b) = (a.clone(), b.clone());
                                rest.push(ast::AstNode::Mul(
                                    Box::new(a.clone()),
                                    Box::new(b.clone()),
                                    a.get_value() * b.get_value(),
                                ));
                                r(rest, target)
                            },
                            {
                                if b.get_value() >= 0.000002 {
                                    let mut rest = rest.clone();
                                    let (a, b) = (a.clone(), b.clone());
                                    rest.push(ast::AstNode::Div(
                                        Box::new(a.clone()),
                                        Box::new(b.clone()),
                                        a.get_value() / b.get_value(),
                                    ));
                                    r(rest, target)
                                } else {
                                    None
                                }
                            },
                            {
                                if a.get_value() >= 0.000002 {
                                    let mut rest = rest.clone();
                                    let (a, b) = (a.clone(), b.clone());
                                    rest.push(ast::AstNode::Div(
                                        Box::new(b.clone()),
                                        Box::new(a.clone()),
                                        b.get_value() / a.get_value(),
                                    ));
                                    r(rest, target)
                                } else {
                                    None
                                }
                            },
                        ])
                        .into_iter()
                        .filter(|i| i.is_some())
                        .map(|i| i.unwrap())
                        .flatten()
                        .collect(),
                    )
                }
            }
            if result.len() == 0 {
                None
            } else {
                Some(result)
            }
        }
    }

    r(
        d6_results.iter().map(|i| ast::AstNode::Num(*i)).collect(),
        prime_match::get_prime_by_level(level),
    )
}

fn float_eq(a: &f32, b: &f32) -> bool {
    a.max(*b) - a.min(*b) <= 0.000002
}


#[cfg(test)]
mod test {
    #[test]
    fn dfs_works() {
        let a = super::dfs_one(&vec![6.0, 5.0, 4.0, 4.0, 1.0], 5).unwrap();
        for b in a {
            println!("{}", b);
        }
    }
}