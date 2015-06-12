#![deny(missing_docs)]

//! Bootstrapped meta rules for mathematical notation.

extern crate range;
extern crate piston_meta;

use std::path::PathBuf;
use std::rc::Rc;
use std::cell::Cell;
use range::Range;
use piston_meta::{ MetaData, Rule };

/// Returns rules for parsing mathematical notation.
pub fn rules() -> Vec<(Rc<String>, Rule)> {
    use piston_meta::*;

    let separators: Rc<String> = Rc::new("()[]{},;:/*+-".into());

    let member_bracket = Rule::Optional(Box::new(Optional {
        debug_id: 100,
        rule: Rule::Sequence(Sequence {
            debug_id: 200,
            args: vec![
                Rule::Token(Token {
                    debug_id: 300,
                    text: Rc::new("[".into()),
                    inverted: false,
                    property: None,
                }),
                Rule::Select(Select {
                    debug_id: 400,
                    args: vec![
                        Rule::Token(Token {
                            debug_id: 500,
                            text: Rc::new(":".into()),
                            inverted: false,
                            property: None,
                        }),
                        Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                            debug_id: 600,
                            any_characters: separators.clone(),
                            optional: false,
                            property: None,
                        }),
                    ]
                }),
                Rule::Token(Token {
                    debug_id: 700,
                    text: Rc::new("]".into()),
                    inverted: false,
                    property: None,
                }),
            ],
        })
    }));

    let brackets_rule = Rule::SeparatedBy(Box::new(SeparatedBy {
        debug_id: 900,
        optional: true,
        allow_trail: false,
        rule: member_bracket,
        by: Rule::Whitespace(Whitespace {
            debug_id: 1000,
            optional: false,
        })
    }));

    let path_rule = Rule::Sequence(Sequence {
        debug_id: 1200,
        args: vec![
            Rule::Optional(Box::new(Optional {
                debug_id: 1300,
                rule: Rule::Token(Token {
                    debug_id: 1400,
                    text: Rc::new("::".into()),
                    inverted: false,
                    property: Some(Rc::new("root".into())),
                }),
            })),
            Rule::SeparatedBy(Box::new(SeparatedBy {
                debug_id: 1500,
                optional: false,
                allow_trail: true,
                rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                    debug_id: 1600,
                    any_characters: separators.clone(),
                    optional: false,
                    property: Some(Rc::new("name".into())),
                }),
                by: Rule::Token(Token {
                    debug_id: 1700,
                    text: Rc::new("::".into()),
                    inverted: false,
                    property: None,
                }),
            })),
        ]
    });

    let arg_rule = Rule::Sequence(Sequence {
        debug_id: 1900,
        args: vec![
            Rule::Node(Node {
                name: Rc::new("brackets".into()),
                property: Some(Rc::new("brackets".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                name: Rc::new("path".into()),
                property: Some(Rc::new("path".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                name: Rc::new("brackets".into()),
                property: Some(Rc::new("brackets".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 2000,
                rule: Rule::Node(Node {
                    name: Rc::new("repeated_arguments".into()),
                    property: Some(Rc::new("repeated_arguments".into())),
                    debug_id: 0,
                    index: Cell::new(None),
                })
            }))
        ]
    });

    let arguments = Rule::Sequence(Sequence {
        debug_id: 2200,
        args: vec![
            Rule::Token(Token {
                debug_id: 2300,
                text: Rc::new("(".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 2400,
                optional: true,
            }),
            Rule::SeparatedBy(Box::new(SeparatedBy {
                debug_id: 2500,
                optional: true,
                allow_trail: true,
                rule: Rule::Select(Select {
                    debug_id: 2600,
                    args: vec![
                        Rule::Number(Number {
                            debug_id: 2700,
                            allow_underscore: true,
                            property: None,
                        }),
                        Rule::Text(Text {
                            debug_id: 2800,
                            allow_empty: true,
                            property: None,
                        }),
                        Rule::Node(Node {
                            name: Rc::new("arguments".into()),
                            property: Some(Rc::new("arguments".into())),
                            debug_id: 0,
                            index: Cell::new(None),
                        }),
                        Rule::Node(Node {
                            name: Rc::new("member_lambda".into()),
                            property: Some(Rc::new("member_lambda".into())),
                            debug_id: 0,
                            index: Cell::new(None),
                        }),
                        Rule::Node(Node {
                            name: Rc::new("lambda".into()),
                            property: Some(Rc::new("lambda".into())),
                            debug_id: 0,
                            index: Cell::new(None),
                        }),
                        Rule::Node(Node {
                            name: Rc::new("arg".into()),
                            property: Some(Rc::new("arg".into())),
                            debug_id: 0,
                            index: Cell::new(None),
                        }),
                    ]
                }),
                by: Rule::Sequence(Sequence {
                    debug_id: 2900,
                    args: vec![
                        Rule::Token(Token {
                            debug_id: 3000,
                            text: Rc::new(",".into()),
                            inverted: false,
                            property: None,
                        }),
                        Rule::Whitespace(Whitespace {
                            debug_id: 3100,
                            optional: false,
                        }),
                    ],
                }),
            })),
            Rule::Whitespace(Whitespace {
                debug_id: 3200,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 3300,
                text: Rc::new(")".into()),
                inverted: false,
                property: None,
            }),
        ]
    });

    let repeated_arguments = Rule::Repeat(Box::new(Repeat {
        debug_id: 3500,
        optional: false,
        rule: Rule::Node(Node {
            name: Rc::new("arguments".into()),
            property: Some(Rc::new("arguments".into())),
            debug_id: 0,
            index: Cell::new(None),
        }),
    }));

    let comment_rule = Rule::Sequence(Sequence {
        debug_id: 3700,
        args: vec![
            Rule::Whitespace(Whitespace {
                debug_id: 22,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 3800,
                text: Rc::new("//".into()),
                inverted: false,
                property: None,
            }),
            Rule::UntilAny(UntilAny {
                debug_id: 3900,
                any_characters: Rc::new("\n".into()),
                optional: true,
                property: None,
            }),
        ],
    });

    let lambda = Rule::Sequence(Sequence {
        debug_id: 4100,
        args: vec![
            Rule::Whitespace(Whitespace {
                debug_id: 4200,
                optional: true,
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 4300,
                rule: Rule::Sequence(Sequence {
                    debug_id: 4400,
                    args: vec![
                        Rule::Token(Token {
                            debug_id: 4500,
                            text: Rc::new("fn".into()),
                            inverted: false,
                            property: None,
                        }),
                        Rule::Whitespace(Whitespace {
                            debug_id: 4600,
                            optional: true,
                        }),
                    ]
                }),
            })),
            Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 4700,
                any_characters: separators.clone(),
                optional: true,
                property: Some(Rc::new("name".into())),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 4800,
                optional: true,
            }),
            Rule::Node(Node {
                name: Rc::new("brackets".into()),
                property: Some(Rc::new("brackets".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                name: Rc::new("repeated_arguments".into()),
                property: Some(Rc::new("repeated_arguments".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 4900,
                optional: false,
            }),
            Rule::Token(Token {
                debug_id: 5000,
                text: Rc::new("->".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 5100,
                optional: false,
            }),
            Rule::Node(Node {
                name: Rc::new("arg".into()),
                property: Some(Rc::new("arg".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 5200,
                optional: true,
            }),
        ]
    });

    let fn_rule = Rule::Sequence(Sequence {
        debug_id: 5400,
        args: vec![
            Rule::Optional(Box::new(Optional {
                debug_id: 5500,
                rule: Rule::Sequence(Sequence {
                    debug_id: 5600,
                    args: vec![
                        Rule::Token(Token {
                            debug_id: 5700,
                            text: Rc::new("pub".into()),
                            inverted: false,
                            property: None,
                        }),
                        Rule::Whitespace(Whitespace {
                            debug_id: 5800,
                            optional: true,
                        }),
                    ]
                }),
            })),
            Rule::Node(Node {
                name: Rc::new("lambda".into()),
                property: Some(Rc::new("lambda".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Token(Token {
                debug_id: 5900,
                text: Rc::new(";".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 6000,
                optional: true,
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 6100,
                rule: Rule::Node(Node {
                    name: Rc::new("comment".into()),
                    property: Some(Rc::new("comment".into())),
                    debug_id: 0,
                    index: Cell::new(None),
                })
            })),
        ]
    });


    let use_rule = Rule::Sequence(Sequence {
        debug_id: 6300,
        args: vec![
            Rule::Whitespace(Whitespace {
                debug_id: 6400,
                optional: true,
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 6500,
                rule: Rule::Sequence(Sequence {
                    debug_id: 6600,
                    args: vec![
                        Rule::Token(Token {
                            debug_id: 6700,
                            text: Rc::new("pub".into()),
                            inverted: false,
                            property: None,
                        }),
                        Rule::Whitespace(Whitespace {
                            debug_id: 6800,
                            optional: true,
                        }),
                    ]
                }),
            })),
            Rule::Token(Token {
                debug_id: 6900,
                text: Rc::new("use".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 7000,
                optional: false,
            }),
            Rule::Node(Node {
                name: Rc::new("path".into()),
                property: Some(Rc::new("path".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 7100,
                rule: Rule::Token(Token {
                    debug_id: 7200,
                    text: Rc::new("*".into()),
                    inverted: false,
                    property: None,
                }),
            })),
            Rule::Token(Token {
                debug_id: 7300,
                text: Rc::new(";".into()),
                inverted: false,
                property: None,
            }),
        ]
    });

    let module = Rule::Sequence(Sequence {
        debug_id: 7500,
        args: vec![
            Rule::Whitespace(Whitespace {
                debug_id: 7600,
                optional: true,
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 7700,
                rule: Rule::Sequence(Sequence {
                    debug_id: 7800,
                    args: vec![
                        Rule::Token(Token {
                            debug_id: 7900,
                            text: Rc::new("pub".into()),
                            inverted: false,
                            property: None,
                        }),
                        Rule::Whitespace(Whitespace {
                            debug_id: 8000,
                            optional: true,
                        }),
                    ]
                }),
            })),
            Rule::Token(Token {
                debug_id: 8100,
                text: Rc::new("mod".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 8200,
                optional: false,
            }),
            Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 8300,
                any_characters: separators.clone(),
                optional: true,
                property: Some(Rc::new("name".into())),
            }),
            Rule::Token(Token {
                debug_id: 8400,
                text: Rc::new(";".into()),
                inverted: false,
                property: None,
            }),
        ]
    });

    let member_lambda = Rule::Sequence(Sequence {
        debug_id: 8600,
        args: vec![
            Rule::Node(Node {
                name: Rc::new("arg".into()),
                property: Some(Rc::new("arg".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 8700,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 8800,
                text: Rc::new(":".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 8900,
                optional: false,
            }),
            Rule::Node(Node {
                name: Rc::new("arg".into()),
                property: Some(Rc::new("arg".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
        ]
    });

    let member_rule = Rule::Sequence(Sequence {
        debug_id: 9100,
        args: vec![
            Rule::Node(Node {
                name: Rc::new("member_lambda".into()),
                property: Some(Rc::new("member_lambda".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Token(Token {
                debug_id: 9200,
                text: Rc::new(";".into()),
                inverted: false,
                property: None,
            }),
        ]
    });

    let line_rule = Rule::Select(Select {
        debug_id: 9300,
        args: vec![
            Rule::Node(Node {
                name: Rc::new("comment".into()),
                property: Some(Rc::new("comment".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                name: Rc::new("use".into()),
                property: Some(Rc::new("use".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                name: Rc::new("module".into()),
                property: Some(Rc::new("module".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                name: Rc::new("member".into()),
                property: Some(Rc::new("member".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                name: Rc::new("fn".into()),
                property: Some(Rc::new("fn".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
        ]
    });

    let document_rule = Rule::Lines(Box::new(Lines {
        debug_id: 9400,
        rule: line_rule,
    }));

    let rules: Vec<(Rc<String>, _)> = vec![
        (Rc::new("comment".into()), comment_rule),
        (Rc::new("use".into()), use_rule),
        (Rc::new("module".into()), module),
        (Rc::new("fn".into()), fn_rule),
        (Rc::new("lambda".into()), lambda),
        (Rc::new("arg".into()), arg_rule),
        (Rc::new("member".into()), member_rule),
        (Rc::new("member_lambda".into()), member_lambda),
        (Rc::new("brackets".into()), brackets_rule),
        (Rc::new("arguments".into()), arguments),
        (Rc::new("path".into()), path_rule),
        (Rc::new("repeated_arguments".into()), repeated_arguments),
        (Rc::new("document".into()), document_rule),
    ];
    update_refs(&rules);
    rules
}

/// Returns rules for parsing meta rules.
pub fn meta_rules() -> Vec<(Rc<String>, Rule)> {
    use std::rc::Rc;
    use piston_meta::*;

    let opt: Rc<String> = Rc::new("optional".into());
    let inv: Rc<String> = Rc::new("inverted".into());
    let prop: Rc<String> = Rc::new("property".into());
    let any: Rc<String> = Rc::new("any_characters".into());
    let seps: Rc<String> = Rc::new("[]{}():.!?".into());

    // 1."string" [..seps!("name") ":" w? t?("text")]
    let string_rule = Rule::Sequence(Sequence {
        debug_id: 1000,
        args: vec![
            Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 1001,
                any_characters: seps.clone(),
                optional: false,
                property: Some(Rc::new("name".into()))
            }),
            Rule::Token(Token {
                debug_id: 1002,
                text: Rc::new(":".into()),
                inverted: false,
                property: None
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 1003,
                optional: true,
            }),
            Rule::Text(Text {
                debug_id: 1004,
                allow_empty: true,
                property: Some(Rc::new("text".into())),
            })
        ]
    });

    // 2."node" [$("id") "." t!("name") w! @"rule"("rule")]
    let node_rule = Rule::Sequence(Sequence {
        debug_id: 2000,
        args: vec![
            Rule::Number(Number {
                debug_id: 2001,
                allow_underscore: false,
                property: Some(Rc::new("id".into())),
            }),
            Rule::Token(Token {
                debug_id: 2002,
                text: Rc::new("::".into()),
                inverted: false,
                property: None,
            }),
            Rule::Text(Text {
                debug_id: 2003,
                allow_empty: false,
                property: Some(Rc::new("name".into())),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 2004,
                optional: false,
            }),
            Rule::Node(Node {
                debug_id: 2005,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            })
        ]
    });

    // rule 3:"set" {t!("value") ..seps!("ref")}
    let set_rule = Rule::Select(Select {
        debug_id: 3003,
        args: vec![
            Rule::Text(Text {
                debug_id: 3004,
                allow_empty: false,
                property: Some(Rc::new("value".into())),
            }),
            Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 3005,
                any_characters: seps.clone(),
                optional: false,
                property: Some(Rc::new("ref".into())),
            })
        ]
    });

    // 4."opt" {"?"(opt) "!"(!opt)}
    let opt_rule = Rule::Select(Select {
        debug_id: 4000,
        args: vec![
            Rule::Token(Token {
                debug_id: 4001,
                text: Rc::new("?".into()),
                inverted: false,
                property: Some(opt.clone())
            }),
            Rule::Token(Token {
                debug_id: 4002,
                text: Rc::new("!".into()),
                inverted: true,
                property: Some(opt.clone())
            }),
        ]
    });

    // 5."number" ["$" ?("_"("underscore")) ?(@"set"("name"))]
    let number_rule = Rule::Sequence(Sequence {
        debug_id: 5000,
        args: vec![
            Rule::Token(Token {
                debug_id: 5001,
                text: Rc::new("$".into()),
                inverted: false,
                property: None,
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 5002,
                rule: Rule::Token(Token {
                    debug_id: 5003,
                    text: Rc::new("_".into()),
                    inverted: false,
                    property: Some(Rc::new("underscore".into()))
                })
            })),
            Rule::Optional(Box::new(Optional {
                debug_id: 5004,
                rule: Rule::Node(Node {
                    debug_id: 5005,
                    name: Rc::new("set".into()),
                    property: Some(Rc::new("property".into())),
                    index: Cell::new(None),
                })
            }))
        ]
    });

    // 6."text" ["t" {"?"("allow_empty") "!"(!"allow_empty")} ?(@"set"(prop))]
    let text_rule = Rule::Sequence(Sequence {
        debug_id: 6000,
        args: vec![
            Rule::Token(Token {
                debug_id: 6001,
                text: Rc::new("t".into()),
                inverted: false,
                property: None,
            }),
            Rule::Select(Select {
                debug_id: 6002,
                args: vec![
                    Rule::Token(Token {
                        debug_id: 6003,
                        text: Rc::new("?".into()),
                        inverted: false,
                        property: Some(Rc::new("allow_empty".into())),
                    }),
                    Rule::Token(Token {
                        debug_id: 6004,
                        text: Rc::new("!".into()),
                        inverted: true,
                        property: Some(Rc::new("allow_empty".into())),
                    })
                ]
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 6005,
                rule: Rule::Node(Node {
                    debug_id: 6006,
                    name: Rc::new("set".into()),
                    property: Some(prop.clone()),
                    index: Cell::new(None),
                })
            })),
        ]
    });

    // 7."reference" [{["@" t!("name")] $("id")} ?(@"set"(prop))]
    let reference_rule = Rule::Sequence(Sequence {
        debug_id: 7000,
        args: vec![
            Rule::Select(Select {
                debug_id: 7001,
                args: vec![
                    Rule::Sequence(Sequence {
                        debug_id: 7002,
                        args: vec![
                            Rule::Token(Token {
                                debug_id: 7003,
                                text: Rc::new("@".into()),
                                inverted: false,
                                property: None,
                            }),
                            Rule::Text(Text {
                                debug_id: 7004,
                                allow_empty: false,
                                property: Some(Rc::new("name".into())),
                            }),
                        ]
                    }),
                    Rule::Number(Number {
                        debug_id: 7005,
                        allow_underscore: false,
                        property: Some(Rc::new("id".into()))
                    })
                ]
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 7006,
                rule: Rule::Node(Node {
                    debug_id: 7007,
                    name: Rc::new("set".into()),
                    property: Some(prop.clone()),
                    index: Cell::new(None)
                })
            }))
        ]
    });

    // 8."sequence" ["[" w? s!.(w!) {@"rule"("rule")} "]"]
    let sequence_rule = Rule::Sequence(Sequence {
        debug_id: 8000,
        args: vec![
            Rule::Token(Token {
                debug_id: 8001,
                text: Rc::new("[".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 8002,
                optional: true,
            }),
            Rule::SeparatedBy(Box::new(SeparatedBy {
                debug_id: 8003,
                optional: false,
                allow_trail: true,
                by: Rule::Whitespace(Whitespace {
                    debug_id: 8004,
                    optional: false,
                }),
                rule: Rule::Node(Node {
                    debug_id: 8005,
                    name: Rc::new("rule".into()),
                    property: Some(Rc::new("rule".into())),
                    index: Cell::new(None)
                })
            })),
            Rule::Token(Token {
                debug_id: 8006,
                text: Rc::new("]".into()),
                inverted: false,
                property: None,
            })
        ]
    });

    // 9."select" ["{" w? s!.(w!) {@"rule"("rule")} "}"]
    let select_rule = Rule::Sequence(Sequence {
        debug_id: 9000,
        args: vec![
            Rule::Token(Token {
                debug_id: 9001,
                text: Rc::new("{".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 9002,
                optional: true,
            }),
            Rule::SeparatedBy(Box::new(SeparatedBy {
                debug_id: 9003,
                optional: false,
                allow_trail: true,
                by: Rule::Whitespace(Whitespace {
                    debug_id: 9004,
                    optional: false,
                }),
                rule: Rule::Node(Node {
                    debug_id: 9005,
                    name: Rc::new("rule".into()),
                    property: Some(Rc::new("rule".into())),
                    index: Cell::new(None),
                })
            })),
            Rule::Token(Token {
                debug_id: 9006,
                text: Rc::new("}".into()),
                inverted: false,
                property: None,
            })
        ]
    });

    // 10."separated_by" ["s" @"opt" ?("."("allow_trail"))
    //   "(" w? @"rule"("by") w? ")" w? "{" w? @"rule"("rule") w? "}"]
    let separated_by_rule = Rule::Sequence(Sequence {
        debug_id: 10000,
        args: vec![
            Rule::Token(Token {
                debug_id: 10001,
                text: Rc::new("s".into()),
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 10002,
                name: Rc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 10003,
                rule: Rule::Token(Token {
                    debug_id: 10004,
                    text: Rc::new(".".into()),
                    inverted: false,
                    property: Some(Rc::new("allow_trail".into())),
                })
            })),
            Rule::Token(Token {
                debug_id: 10004,
                text: Rc::new("(".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 10005,
                optional: true,
            }),
            Rule::Node(Node {
                debug_id: 10006,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("by".into())),
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 10007,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 10008,
                text: Rc::new(")".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 10009,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 10010,
                text: Rc::new("{".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 10011,
                optional: true,
            }),
            Rule::Node(Node {
                debug_id: 10012,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 10013,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 10014,
                text: Rc::new("}".into()),
                inverted: false,
                property: None,
            }),
        ]
    });

    // rule 11:"token" [@"set"("text") ?(["(" w? ?("!"(inv)) @"set"(prop) w? ")"])]
    let token_rule = Rule::Sequence(Sequence {
        debug_id: 11000,
        args: vec![
            Rule::Node(Node {
                debug_id: 11001,
                name: Rc::new("set".into()),
                property: Some(Rc::new("text".into())),
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 11002,
                rule: Rule::Sequence(Sequence {
                    debug_id: 11003,
                    args: vec![
                        Rule::Token(Token {
                            debug_id: 11004,
                            text: Rc::new("(".into()),
                            inverted: false,
                            property: None,
                        }),
                        Rule::Whitespace(Whitespace {
                            debug_id: 11005,
                            optional: true,
                        }),
                        Rule::Optional(Box::new(Optional {
                            debug_id: 11006,
                            rule: Rule::Token(Token {
                                debug_id: 11007,
                                text: Rc::new("!".into()),
                                inverted: false,
                                property: Some(inv.clone()),
                            })
                        })),
                        Rule::Node(Node {
                            debug_id: 11009,
                            name: Rc::new("set".into()),
                            property: Some(prop.clone()),
                            index: Cell::new(None),
                        }),
                        Rule::Whitespace(Whitespace {
                            debug_id: 11010,
                            optional: true,
                        }),
                        Rule::Token(Token {
                            debug_id: 11011,
                            text: Rc::new(")".into()),
                            inverted: false,
                            property: None,
                        })
                    ]
                })
            })),
        ]
    });

    // 12."optional" ["?(" w? @"rule"("rule") w? ")"]
    let optional_rule = Rule::Sequence(Sequence {
        debug_id: 12001,
        args: vec![
            Rule::Token(Token {
                debug_id: 12002,
                text: Rc::new("?(".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 12003,
                optional: true,
            }),
            Rule::Node(Node {
                debug_id: 12004,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 12005,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 12006,
                text: Rc::new(")".into()),
                inverted: false,
                property: None,
            })
        ]
    });

    // 13."whitespace" ["w" @"opt"]
    let whitespace_rule = Rule::Sequence(Sequence {
        debug_id: 13000,
        args: vec![
            Rule::Token(Token {
                debug_id: 13001,
                text: Rc::new("w".into()),
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 13002,
                name: Rc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            })
        ]
    });

    // 14."until_any_or_whitespace" [".." @"set"(any) @"opt" ?(@"set"(prop))]
    let until_any_or_whitespace_rule = Rule::Sequence(Sequence {
        debug_id: 14001,
        args: vec![
            Rule::Token(Token {
                debug_id: 14002,
                text: Rc::new("..".into()),
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 14003,
                name: Rc::new("set".into()),
                property: Some(any.clone()),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 14004,
                name: Rc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 14005,
                rule: Rule::Node(Node {
                    debug_id: 14006,
                    name: Rc::new("set".into()),
                    property: Some(prop.clone()),
                    index: Cell::new(None),
                })
            }))
        ]
    });

    // 15."until_any" ["..." @"set"(any) @"opt" ?(@"set"(prop)) ]
    let until_any_rule = Rule::Sequence(Sequence {
        debug_id: 15000,
        args: vec![
            Rule::Token(Token {
                debug_id: 15001,
                text: Rc::new("...".into()),
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 15002,
                name: Rc::new("set".into()),
                property: Some(any.clone()),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 15003,
                name: Rc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 15004,
                rule: Rule::Node(Node {
                    debug_id: 15005,
                    name: Rc::new("set".into()),
                    property: Some(prop.clone()),
                    index: Cell::new(None),
                })
            }))
        ]
    });

    // 16."repeat" ["r" @"opt" "(" @"rule"("rule") ")"]
    let repeat_rule = Rule::Sequence(Sequence {
        debug_id: 16000,
        args: vec![
            Rule::Token(Token {
                debug_id: 16001,
                text: Rc::new("r".into()),
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 16002,
                name: Rc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Token(Token {
                debug_id: 16003,
                text: Rc::new("(".into()),
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 16004,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            }),
            Rule::Token(Token {
                debug_id: 16005,
                text: Rc::new(")".into()),
                inverted: false,
                property: None,
            })
        ]
    });

    // 17."lines" ["l(" w? @"rule"("rule") w? ")"]
    let lines_rule = Rule::Sequence(Sequence {
        debug_id: 17000,
        args: vec![
            Rule::Token(Token {
                debug_id: 17001,
                text: Rc::new("l(".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 17002,
                optional: true,
            }),
            Rule::Node(Node {
                debug_id: 17003,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 17004,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 17005,
                text: Rc::new(")".into()),
                inverted: false,
                property: None,
            })
        ]
    });

    // 18."rule" {5("number") 6("text") 7("reference") 8("sequence")
    //   9("select") 10("separated_by") 11("token") 12("optional")
    //   13("whitespace") 14("until_any_or_whitespace") 15("until_any")
    //   16("repeat") 17("lines")}
    let rule_rule = Rule::Select(Select {
        debug_id: 18000,
        args: vec![
            Rule::Node(Node {
                debug_id: 18009,
                name: Rc::new("whitespace".into()),
                property: Some(Rc::new("whitespace".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18009,
                name: Rc::new("until_any_or_whitespace".into()),
                property: Some(Rc::new("until_any_or_whitespace".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18010,
                name: Rc::new("until_any".into()),
                property: Some(Rc::new("until_any".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18012,
                name: Rc::new("lines".into()),
                property: Some(Rc::new("lines".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18011,
                name: Rc::new("repeat".into()),
                property: Some(Rc::new("repeat".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18001,
                name: Rc::new("number".into()),
                property: Some(Rc::new("number".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18002,
                name: Rc::new("text".into()),
                property: Some(Rc::new("text".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18003,
                name: Rc::new("reference".into()),
                property: Some(Rc::new("reference".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18004,
                name: Rc::new("sequence".into()),
                property: Some(Rc::new("sequence".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18005,
                name: Rc::new("select".into()),
                property: Some(Rc::new("select".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18006,
                name: Rc::new("separated_by".into()),
                property: Some(Rc::new("separated_by".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18007,
                name: Rc::new("token".into()),
                property: Some(Rc::new("token".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18008,
                name: Rc::new("optional".into()),
                property: Some(Rc::new("optional".into())),
                index: Cell::new(None),
            }),
        ]
    });

    // 19::"document" [l(@"string""string") l(@"node""node") w?]
    let document_rule = Rule::Sequence(Sequence {
        debug_id: 19000,
        args: vec![
            Rule::Lines(Box::new(Lines {
                debug_id: 19001,
                rule: Rule::Node(Node {
                    debug_id: 19002,
                    name: Rc::new("string".into()),
                    property: Some(Rc::new("string".into())),
                    index: Cell::new(None),
                })
            })),
            Rule::Lines(Box::new(Lines {
                debug_id: 19002,
                rule: Rule::Node(Node {
                    debug_id: 19003,
                    name: Rc::new("node".into()),
                    property: Some(Rc::new("node".into())),
                    index: Cell::new(None),
                })
            })),
            Rule::Whitespace(Whitespace {
                debug_id: 19004,
                optional: true,
            })
        ]
    });

    let rules = vec![
        (Rc::new("string".into()), string_rule),
        (Rc::new("node".into()), node_rule),
        (Rc::new("set".into()), set_rule),
        (Rc::new("opt".into()), opt_rule),
        (Rc::new("number".into()), number_rule),
        (Rc::new("text".into()), text_rule),
        (Rc::new("reference".into()), reference_rule),
        (Rc::new("sequence".into()), sequence_rule),
        (Rc::new("select".into()), select_rule),
        (Rc::new("separated_by".into()), separated_by_rule),
        (Rc::new("token".into()), token_rule),
        (Rc::new("optional".into()), optional_rule),
        (Rc::new("whitespace".into()), whitespace_rule),
        (Rc::new("until_any_or_whitespace".into()), until_any_or_whitespace_rule),
        (Rc::new("until_any".into()), until_any_rule),
        (Rc::new("repeat".into()), repeat_rule),
        (Rc::new("lines".into()), lines_rule),
        (Rc::new("rule".into()), rule_rule),
        (Rc::new("document".into()), document_rule),
    ];
    update_refs(&rules);
    rules
}

/// Prints read meta data.
pub fn print_meta_data(data: &[(Range, MetaData)]) {
    for d in data {
        match &d.1 {
            &MetaData::StartNode(ref name) => {
                println!("start `{}`", name);
            }
            &MetaData::EndNode(ref name) => {
                println!("end `{}`", name);
            }
            &MetaData::F64(ref name, val) => {
                println!("{}: {}", name, val);
            }
            &MetaData::Bool(ref name, val) => {
                println!("{}: {}", name, val);
            }
            &MetaData::String(ref name, ref val) => {
                println!("{}: {}", name, val);
            }
        }
    }
}

/// Converts meta data to rules.
pub fn convert_meta_data_to_rules(mut data: &[(Range, MetaData)])
-> Result<Vec<(Rc<String>, Rule)>, ()> {
    use piston_meta::*;

    fn update(range: Range, data: &mut &[(Range, MetaData)], offset: &mut usize) {
        let next_offset = range.next_offset();
        *data = &data[next_offset - *offset..];
        *offset = next_offset;
    }

    fn start_node(name: &str, data: &[(Range, MetaData)], offset: usize)
    -> Result<Range, ()> {
        if data.len() == 0 { return Err(()); }
        match &data[0].1 {
            &MetaData::StartNode(ref n) if &**n == name => {
                Ok(Range::new(offset, 1))
            }
            _ => Err(())
        }
    }

    fn end_node(name: &str, data: &[(Range, MetaData)], offset: usize)
    -> Result<Range, ()> {
        if data.len() == 0 { return Err(()); }
        match &data[0].1 {
            &MetaData::EndNode(ref n) if &**n == name => {
                Ok(Range::new(offset, 1))
            }
            _ => Err(())
        }
    }

    fn meta_string(name: &str, data: &[(Range, MetaData)], offset: usize)
    -> Result<(Range, Rc<String>), ()> {
        if data.len() == 0 { return Err(()); }
        match &data[0].1 {
            &MetaData::String(ref n, ref val) if &**n == name => {
                Ok((Range::new(offset, 1), val.clone()))
            }
            _ => Err(())
        }
    }

    fn meta_f64(name: &str, data: &[(Range, MetaData)], offset: usize)
    -> Result<(Range, f64), ()> {
        if data.len() == 0 { return Err(()); }
        match &data[0].1 {
            &MetaData::F64(ref n, ref val) if &**n == name => {
                Ok((Range::new(offset, 1), *val))
            }
            _ => Err(())
        }
    }

    fn meta_bool(name: &str, data: &[(Range, MetaData)], offset: usize)
    -> Result<(Range, bool), ()> {
        if data.len() == 0 { return Err(()); }
        match &data[0].1 {
            &MetaData::Bool(ref n, ref val) if &**n == name => {
                Ok((Range::new(offset, 1), *val))
            }
            _ => Err(())
        }
    }

    fn read_string(mut data: &[(Range, MetaData)], mut offset: usize)
    -> Result<(Range, (Rc<String>, Rc<String>)), ()> {
        let start_offset = offset;
        let range = try!(start_node("string", data, offset));
        update(range, &mut data, &mut offset);
        let mut name = None;
        let mut text = None;
        loop {
            if let Ok((range, val)) = meta_string("name", data, offset) {
                name = Some(val);
                update(range, &mut data, &mut offset);
            } else if let Ok((range, val)) = meta_string("text", data, offset) {
                text = Some(val);
                update(range, &mut data, &mut offset);
            } else if let Ok(range) = end_node("string", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else {
                return Err(())
            }
        }
        let name = match name {
            None => { return Err(()); }
            Some(x) => x
        };
        let text = match text {
            None => { return Err(()); }
            Some(x) => x
        };
        Ok((Range::new(start_offset, offset - start_offset), (name, text)))
    }

    fn read_sequence(mut data: &[(Range, MetaData)], mut offset: usize,
    strings: &[(Rc<String>, Rc<String>)])
    -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("sequence", data, offset));
        update(range, &mut data, &mut offset);
        let mut args: Vec<Rule> = vec![];
        loop {
            if let Ok(range) = end_node("sequence", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_rule(data, offset, strings) {
                update(range, &mut data, &mut offset);
                args.push(val);
            } else {
                println!("TEST sequence {:?}", &data[0]);
                return Err(());
            }
        }
        Ok((Range::new(start_offset, offset - start_offset), Rule::Sequence(Sequence {
            debug_id: 0,
            args: args
        })))
    }

    fn find_string(val: &str, strings: &[(Rc<String>, Rc<String>)]) -> Option<Rc<String>> {
        strings.iter().find(|&&(ref s, _)| &**s == val).map(|&(_, ref s)| s.clone())
    }

    fn read_set(property: &str, mut data: &[(Range, MetaData)], mut offset: usize,
    strings: &[(Rc<String>, Rc<String>)])
    -> Result<(Range, Rc<String>), ()> {
        let start_offset = offset;
        let range = try!(start_node(property, data, offset));
        update(range, &mut data, &mut offset);
        let mut text = None;
        loop {
            if let Ok(range) = end_node(property, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_string("ref", data, offset) {
                update(range, &mut data, &mut offset);
                text = find_string(&val, strings);
            } else if let Ok((range, val)) = meta_string("value", data, offset) {
                update(range, &mut data, &mut offset);
                text = Some(val);
            } else {
                return Err(())
            }
        }
        match text {
            None => Err(()),
            Some(text) => Ok((Range::new(start_offset, offset - start_offset), text))
        }
    }

    fn read_until_any_or_whitespace(mut data: &[(Range, MetaData)], mut offset: usize,
    strings: &[(Rc<String>, Rc<String>)])
    -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("until_any_or_whitespace", data, offset));
        update(range, &mut data, &mut offset);
        let mut any_characters = None;
        let mut optional = None;
        let mut property = None;
        loop {
            if let Ok(range) = end_node("until_any_or_whitespace", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_set("any_characters", data, offset, strings) {
                update(range, &mut data, &mut offset);
                any_characters = Some(val);
            } else if let Ok((range, val)) = meta_bool("optional", data, offset) {
                update(range, &mut data, &mut offset);
                optional = Some(val);
            } else if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else {
                println!("TEST {} until_any_or_whitespace {:?}", offset, &data[0]);
                return Err(());
            }
        }
        let optional = optional.unwrap_or(false);
        match any_characters {
            Some(any) => {
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                    debug_id: 0,
                    any_characters: any,
                    optional: optional,
                    property: property
                })))
            }
            None => Err(())
        }
    }

    fn read_token(mut data: &[(Range, MetaData)], mut offset: usize,
    strings: &[(Rc<String>, Rc<String>)])
    -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("token", data, offset));
        update(range, &mut data, &mut offset);

        let mut text = None;
        loop {
            if let Ok(range) = end_node("token", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_set("text", data, offset, strings) {
                update(range, &mut data, &mut offset);
                text = Some(val);
            } else {
                println!("TEST {} token {:?}", offset, &data[0].1);
                return Err(());
            }
        }
        match text {
            Some(text) => {
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::Token(Token {
                    debug_id: 0,
                    text: text,
                    inverted: false,
                    property: None,
                })))
            }
            None => Err(())
        }
    }

    fn read_whitespace(mut data: &[(Range, MetaData)], mut offset: usize)
    -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("whitespace", data, offset));
        update(range, &mut data, &mut offset);
        let (range, optional) = try!(meta_bool("optional", data, offset));
        update(range, &mut data, &mut offset);
        let range = try!(end_node("whitespace", data, offset));
        update(range, &mut data, &mut offset);
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Whitespace(Whitespace {
            debug_id: 0,
            optional: optional,
        })))
    }

    fn read_text(mut data: &[(Range, MetaData)], mut offset: usize,
    strings: &[(Rc<String>, Rc<String>)])
    -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("text", data, offset));
        update(range, &mut data, &mut offset);
        let mut allow_empty = None;
        let mut property = None;
        loop {
            if let Ok(range) = end_node("text", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } if let Ok((range, val)) = meta_bool("allow_empty", data, offset) {
                update(range, &mut data, &mut offset);
                allow_empty = Some(val);
            } if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else {
                println!("TEST {} text {:?}", offset, &data[0].1);
                return Err(());
            }
        }
        let allow_empty = allow_empty.unwrap_or(true);
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Text(Text {
            debug_id: 0,
            allow_empty: allow_empty,
            property: property,
        })))
    }

    fn read_number(mut data: &[(Range, MetaData)], mut offset: usize,
    strings: &[(Rc<String>, Rc<String>)])
    -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("number", data, offset));
        update(range, &mut data, &mut offset);

        let mut property = None;
        loop {
            if let Ok(range) = end_node("number", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else {
                println!("TEST {} number {:?}", offset, &data[0].1);
                return Err(());
            }
        }
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Number(Number {
            debug_id: 0,
            property: property,
            allow_underscore: false,
        })))
    }

    fn read_reference(mut data: &[(Range, MetaData)], mut offset: usize,
    strings: &[(Rc<String>, Rc<String>)])
    -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("reference", data, offset));
        update(range, &mut data, &mut offset);

        let mut name = None;
        let mut property = None;
        loop {
            if let Ok(range) = end_node("reference", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_string("name", data, offset) {
                update(range, &mut data, &mut offset);
                name = Some(val);
            } else if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else {
                println!("TEST {} reference {:?}", offset, &data[0].1);
                return Err(());
            }
        }
        match name {
            Some(name) => {
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::Node(Node {
                    debug_id: 0,
                    name: name,
                    property: property,
                    index: Cell::new(None),
                })))
            }
            None => Err(())
        }
    }

    fn read_select(mut data: &[(Range, MetaData)], mut offset: usize,
    strings: &[(Rc<String>, Rc<String>)])
    -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("select", data, offset));
        update(range, &mut data, &mut offset);
        let mut args: Vec<Rule> = vec![];
        loop {
            if let Ok(range) = end_node("select", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_rule(data, offset, strings) {
                update(range, &mut data, &mut offset);
                args.push(val);
            } else {
                println!("TEST {} select {:?}", offset, &data[0].1);
                return Err(());
            }
        }
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Select(Select {
            debug_id: 0,
            args: args
        })))
    }

    fn read_rule(mut data: &[(Range, MetaData)], mut offset: usize,
    strings: &[(Rc<String>, Rc<String>)])
    -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("rule", data, offset));
        update(range, &mut data, &mut offset);

        let mut rule = None;
        if let Ok((range, val)) = read_sequence(data, offset, strings) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_until_any_or_whitespace(data, offset, strings) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_token(data, offset, strings) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_whitespace(data, offset) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_text(data, offset, strings) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_number(data, offset, strings) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_reference(data, offset, strings) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_select(data, offset, strings) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        }

        if let Some(rule) = rule {
            let range = try!(end_node("rule", data, offset));
            update(range, &mut data, &mut offset);
            Ok((Range::new(start_offset, offset - start_offset), rule))
        } else {
            println!("TEST {} rule {:?}", offset, &data[0].1);
            Err(())
        }
    }

    fn read_node(mut data: &[(Range, MetaData)], mut offset: usize,
    strings: &[(Rc<String>, Rc<String>)])
    -> Result<(Range, (Rc<String>, Rule)), ()> {
        let start_offset = offset;
        let range = try!(start_node("node", data, offset));
        update(range, &mut data, &mut offset);
        let mut id = None;
        let mut name = None;
        let mut rule = None;
        loop {
            if let Ok(range) = end_node("node", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_f64("id", data, offset) {
                id = Some(val);
                update(range, &mut data, &mut offset);
            } else if let Ok((range, val)) = meta_string("name", data, offset) {
                name = Some(val);
                update(range, &mut data, &mut offset);
            } else if let Ok((range, val)) = read_rule(data, offset, strings) {
                rule = Some(val);
                update(range, &mut data, &mut offset);
            } else {
                println!("TEST node {:?}", &data[0]);
                return Err(())
            }
        }
        match (name, rule) {
            (Some(name), Some(rule)) => {
                Ok((Range::new(start_offset, offset - start_offset), (name, rule)))
            }
            _ => Err(())
        }
    }

    let mut strings: Vec<(Rc<String>, Rc<String>)> = vec![];
    let mut offset: usize = 0;
    loop {
        if let Ok((range, val)) = read_string(data, offset) {
            strings.push(val);
            update(range, &mut data, &mut offset);
        } else {
            break;
        }
    }
    let mut res = vec![];
    loop {
        if let Ok((range, val)) = read_node(data, offset, &strings) {
            res.push(val);
            update(range, &mut data, &mut offset);
        } else {
            return Err(());
        }
    }
    Ok(res)
}

/// Stores information about error occursing when parsing syntax.
pub enum SyntaxError {
    /// An io error occured.
    IoError(std::io::Error),
    /// A meta rule failed when parsing syntax.
    MetaError(PathBuf, String, Range, piston_meta::ParseError),
}

impl From<std::io::Error> for SyntaxError {
    fn from(error: std::io::Error) -> SyntaxError {
        SyntaxError::IoError(error)
    }
}

/// Stores information about mathematical functions.
pub struct Syntax {
    /// The source files.
    pub files: Vec<PathBuf>,
}

impl Syntax {
    /// Parses syntax.
    pub fn new(rules: &[(Rc<String>, Rule)], files: Vec<PathBuf>)
    -> Result<Syntax, SyntaxError> {
        use std::fs::File;
        use std::io::Read;
        use piston_meta::*;

        for file in &files {
            let mut file_h = try!(File::open(file));
            let mut source = String::new();
            try!(file_h.read_to_string(&mut source));

            let res = parse(&rules, &source);
            match res {
                Ok(_) => {
                    /*
                    println!("TEST tokens");
                    for token in &tokenizer.tokens[s.0..] {
                        println!("{:?}", token.0);
                    }
                    */
                }
                Err((range, err)) => {
                    return Err(SyntaxError::MetaError(
                        file.into(),
                        source,
                        range,
                        err
                    ));

                }
            }
        }
        Ok(Syntax {
            files: files,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntax() {
        let rules = rules();
        if let Err(SyntaxError::MetaError(file, source, range, err))
            = Syntax::new(&rules, vec![
                "assets/bool.txt".into(),
                "assets/nat.txt".into(),
                "assets/option.txt".into(),
                "assets/string.rs".into(),
            ]) {
            use piston_meta::*;

            let mut std_err = ParseStdErr::new(&source);
            println!("file: {:?}", file);
            // println!("source {}", source);
            std_err.error(range, err);
            assert!(false);
        }
    }

    #[test]
    fn meta_syntax() {
        let rules = meta_rules();
        if let Err(SyntaxError::MetaError(file, source, range, err))
            = Syntax::new(&rules, vec![
                "assets/self-syntax.txt".into(),
                "assets/syntax.txt".into(),
            ]) {
            use piston_meta::*;

            let mut std_err = ParseStdErr::new(&source);
            println!("file: {:?}", file);
            // println!("source {}", source);
            std_err.error(range, err);
            assert!(false);
        }
    }
}
