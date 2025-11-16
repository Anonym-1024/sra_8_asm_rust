use core::num;

use crate::{parser::cst::{CstNode, CstNodeKind}, sema::ast::helpers::num_lit_to_int};


#[derive(Debug)]
pub enum DataType {
    Byte,
    Bytes(u32),
    Arr(u32, Box<DataType>)
}


impl DataType {
    pub fn from(node: &CstNode) -> Self {
        assert_eq!(node.kind, CstNodeKind::TypeDirective);
        let kind = node.child(0).kind;

        match kind {
            CstNodeKind::ByteDirective => { return Self::Byte; },
            CstNodeKind::BytesDirective => { return make_bytes_type(node.child(0)); },
            CstNodeKind::ArrDirective => { return make_arr_type(node.child(0)); },
            _ => unreachable!()
        }
    }
}


fn make_bytes_type(node: &CstNode) -> DataType{
    assert_eq!(node.kind, CstNodeKind::BytesDirective);

    let number_token = node.child(1).terminal.as_ref().unwrap();
    DataType::Bytes(num_lit_to_int(number_token) as u32)
}

fn make_arr_type(node: &CstNode) -> DataType{
    assert_eq!(node.kind, CstNodeKind::ArrDirective);

    let number_token = node.child(1).terminal.as_ref().unwrap();
    let num = num_lit_to_int(number_token) as u32;

    let type_node = node.child(2);
    let data_type = DataType::from(type_node);


    DataType::Arr(num, Box::from(data_type))
}