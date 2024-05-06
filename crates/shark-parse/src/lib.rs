
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Node {
    VariableDecl {
        identifier: String,
        var_type: String, // This will be a string for now
        mutable: bool,
    },
    BinaryExpr {
        operator: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>
    },
    UnaryExpr {
        operator: Operator,
        child: Box<Node>
    },
    BlockExpr {
        child: Box<Node>
    },
    FunctionDef {
        identifier: String,
        params: Vec<(String, String)>,
        return_type: String,
        public: bool,
        body: Box<Node>
    }
}
