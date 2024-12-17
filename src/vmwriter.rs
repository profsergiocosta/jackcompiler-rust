pub struct VMWriter {
    vm_output: String,
}

#[derive(Debug)]
pub enum Segment {
    Const,
    Arg,
    Local,
    Static,
    This,
    That,
    Pointer,
    Temp,
}

impl Segment {
    fn as_str(&self) -> &'static str {
        match self {
            Segment::Const => "constant",
            Segment::Arg => "argument",
            Segment::Local => "local",
            Segment::Static => "static",
            Segment::This => "this",
            Segment::That => "that",
            Segment::Pointer => "pointer",
            Segment::Temp => "temp",
        }
    }
}

#[derive(Debug)]
pub enum Command {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl VMWriter {
    pub fn new() -> Self {
        Self {
            vm_output: String::new(),
        }
    }

    pub fn vm_output(&self) -> &str {
        &self.vm_output
    }

    pub fn write_push(&mut self, segment: Segment, index: u16) {
        self.vm_output
            .push_str(&format!("push {} {}\n", segment.as_str(), index));
    }

    pub fn write_pop(&mut self, segment: Segment, index: usize) {
        self.vm_output
            .push_str(&format!("pop {} {}\n", segment.as_str(), index));
    }

    pub fn write_arithmetic(&mut self, command: Command) {
        self.vm_output
            .push_str(&format!("{}\n", format!("{:?}", command).to_lowercase()));
    }

    pub fn write_label(&mut self, label: &str) {
        self.vm_output.push_str(&format!("label {}\n", label));
    }

    pub fn write_goto(&mut self, label: &str) {
        self.vm_output.push_str(&format!("goto {}\n", label));
    }

    pub fn write_if(&mut self, label: &str) {
        self.vm_output.push_str(&format!("if-goto {}\n", label));
    }

    pub fn write_call(&mut self, name: &str, n_args: usize) {
        self.vm_output
            .push_str(&format!("call {} {}\n", name, n_args));
    }

    pub fn write_function(&mut self, name: &str, n_locals: usize) {
        self.vm_output
            .push_str(&format!("function {} {}\n", name, n_locals));
    }

    pub fn write_return(&mut self) {
        self.vm_output.push_str("return\n");
    }
}

