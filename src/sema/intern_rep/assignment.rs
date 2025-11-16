

pub enum AssignmentValue {
    Number(i64),
    String(Vec<u8>),
    Assignment(Assignment)
}

pub struct Assignment {
    values: Vec<AssignmentValue>,
    repetition: u32
}