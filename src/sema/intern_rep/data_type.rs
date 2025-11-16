

pub enum DataType {
    Byte,
    Bytes(u32),
    Arr(Box<DataType>)
}

