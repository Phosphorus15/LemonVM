use super::*;
// #[test]
// fn LocVarFromBytes() {
//     let bytes = [
//         0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
//         0x00,
//     ];
//     let mut reader = reader::Reader::new(bytes.as_ptr());
//     let locvar = reader.read_loc_var();
//     assert_eq!(locvar.name.0[0], 'L' as u16);
//     assert_eq!(locvar.name.0[1], 'e' as u16);
//     assert_eq!(locvar.start_pc, 1);
//     assert_eq!(locvar.end_pc, 2);
// }

#[test]
fn ValidateHeader() {
    let mut reader =
        reader::Reader::new([0x4c, 0x65, 0x4d, 0x30, 0x26, 0x01, 0x04, 0x02, 0x04, 0x08].as_ptr());
    let header = reader.read_header();
    assert_eq!(header.validate(), true);
}
// #[test]
// fn closure_capFromBytes() {
//     let bytes = [0x01, 0x02];
//     let mut reader = reader::Reader::new(bytes.as_ptr());
//     let closure_cap = reader.read_closure_cap();
//     assert_eq!(closure_cap.instack, 0x01);
//     assert_eq!(closure_cap.idx, 0x02);
// }

#[test]
fn ProtoFromByteCode() {
    // layout: types_len (tag len (uuid:data)*)*
    let constant_pool = [
        0x01,
        0x03,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,
    ];
    let bytes = [
        0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00,
        0x00,0x00,0x00,0x00,
        0x00,
        0x00,
        0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
    ];
    reader::Reader::read_constant_pool(constant_pool.as_ptr(), constant_pool.len());
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let func = reader.read_func();
    assert_eq!(super::constant_and_pool::get_constant(0x03,1), constant_and_pool::Constant::Int(1));
    assert_eq!(func.instruction_table.len(), 0);
    assert_eq!(func.const_func_refs.len(), 0);
}

#[test]
fn BinaryChunkFromByteCode() {
    let bytes = [
        0x4c, 0x65, 0x4d, 0x30, 0x26, 0x01, 0x04, 0x02, 0x04, 0x08, 
        0x01, 
        0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00,
        0x00,0x00,0x00,0x00,
        0x00,
        0x00,
        0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
    ];
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let bin = reader.read_binary_chunk();
    assert_eq!(bin.header.validate(), true);
    assert_eq!(bin.up_value_size, 1);
}

#[test]
fn ReadRowType() {
    // 0x00 -> array, 0xff -> row
    // 0x00         len         (flag  data)
    // 0xFF         len         ( vmsym    flag      data     )*
    // row start    row size      key     value type  value
    let bytes = [
        0x00,
        0x02,0x00,0x00,0x00,
        0x03, 0x01,0x00,0x00,0x00,
        0x00
    ];
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let c = reader.read_constant(0x09);
    println!("{:?}",c);
    if let super::constant_and_pool::Constant::Row(r) = c {
        assert_eq!(r.is_arr,true);
        assert_eq!(r.arr[0],super::constant_and_pool::Constant::Int(1));
    }

    let bytes = [
        0xFF,
        0x02,0x00,0x00,0x00,
        0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00,
        0x03, 0x01,0x00,0x00,0x00,
        0x01, 0x00, 0x00, 0x00, 0x68, 0x00,
        0x00
    ];
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let c = reader.read_constant(0x09);
    println!("{:?}",c);
    if let super::constant_and_pool::Constant::Row(r) = c {
        assert_eq!(r.is_arr,false);
        assert_eq!(r.row[&super::VMSym(vec!(0x0068))],super::constant_and_pool::Constant::Null);
    }
}