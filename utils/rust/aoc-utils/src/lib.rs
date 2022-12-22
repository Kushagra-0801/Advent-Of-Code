#[derive(Debug, Clone, Copy)]
pub struct TestCase {
    pub input: &'static str,
    pub out1: &'static str,
    pub out2: &'static str,
}

impl From<[&'static str; 3]> for TestCase {
    fn from(value: [&'static str; 3]) -> Self {
        Self::from(&value)
    }
}

impl From<&[&'static str; 3]> for TestCase {
    fn from(value: [&'static str; 3]) -> Self {
        Self {
            input: value[0],
            out1: value[1],
            out2: value[2],
        }
    }
}
