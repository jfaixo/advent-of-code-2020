
pub type Expression = Box<dyn Evaluable>;

pub trait Evaluable {
    fn evaluate(&self) -> i64;
    fn print(&self, depth: usize);
}

pub trait Operation {
    fn set_left(&mut self, value: Expression);
    fn set_right(&mut self, value: Expression);
}

pub struct Number {
    pub value : i64,
}

impl Evaluable for Number {
    fn evaluate(&self) -> i64 {
        self.value
    }

    fn print(&self, depth: usize) {
        for _i in 0..depth { eprint!(" ") }
        eprintln!("{}", self.value);
    }
}

pub struct Addition{
    pub left: Expression,
    pub right: Expression,
}

impl Evaluable for Addition {
    fn evaluate(&self) -> i64 {
        self.left.evaluate() + self.right.evaluate()
    }

    fn print(&self, depth: usize) {
        for _i in 0..depth { eprint!(" ") }
        eprintln!("+");
        self.left.print(depth + 2);
        self.right.print(depth + 2);
    }
}

impl Operation for Addition {
    fn set_left(&mut self, value: Expression) {
        self.left = value;
    }

    fn set_right(&mut self, value: Expression) {
        self.right = value;
    }
}

pub struct Multiplication {
    pub left: Expression,
    pub right: Expression,
}

impl Evaluable for Multiplication {
    fn evaluate(&self) -> i64 {
        self.left.evaluate() * self.right.evaluate()
    }

    fn print(&self, depth: usize) {
        for _i in 0..depth { eprint!(" ") }
        eprintln!("*");
        self.left.print(depth + 2);
        self.right.print(depth + 2);
    }
}

impl Operation for Multiplication {
    fn set_left(&mut self, value: Expression) {
        self.left = value;
    }

    fn set_right(&mut self, value: Expression) {
        self.right = value;
    }
}
