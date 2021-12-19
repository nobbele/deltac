use std::borrow::Cow;

pub struct Generator {
    asm: Vec<String>,
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            asm: Vec::new()
        }
    }

    pub fn raw<'a>(&mut self, line: impl Into<Cow<'a, str>>) {
        self.asm.push(line.into().into_owned());
    }

    pub fn label<'a>(&mut self, name: impl Into<Cow<'a, str>>) {
        self.raw(format!("{}:", name.into()));
    }

    pub fn label_with_value<'a>(&mut self, name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) {
        self.raw(format!("{}: {}", name.into(), value.into()));
    }

    pub fn exit(&mut self, code: u32) {
        self.raw(format!("mov ${}, %rdi", code));
        self.raw("call exit");
    }

    pub fn full_raw(&self) -> String {
        let mut s = self.asm.join("\n");
        s.push('\n');
        s
    }
}