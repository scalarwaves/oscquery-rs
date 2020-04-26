use crate::value::*;

#[derive(Debug)]
pub enum ParamGet {
    Int(ValueGet<i32>),
    Float(ValueGet<f32>),
    String(ValueGet<String>),
    Time(ValueGet<(u32, u32)>),
    Long(ValueGet<i64>),
    Double(ValueGet<f64>),
    Char(ValueGet<char>),
    Midi(ValueGet<(u8, u8, u8, u8)>),
    Bool(ValueGet<bool>),
    //TODO Blob(ValueGet<Box<[u8]>>), //does clip mode make and range make sense?
    //TODO Array(Box<[Self]>),
    //TODO Nil,
    //TODO Inf,
}

#[derive(Debug)]
pub enum ParamSet {
    Int(ValueSet<i32>),
    Float(ValueSet<f32>),
    String(ValueSet<String>),
    Time(ValueSet<(u32, u32)>),
    Long(ValueSet<i64>),
    Double(ValueSet<f64>),
    Char(ValueSet<char>),
    Midi(ValueSet<(u8, u8, u8, u8)>),
    Bool(ValueSet<bool>),
    //TODO Blob(ValueSet<Box<[u8]>>), //does clip mode make and range make sense?
    //TODO Array(Box<[Self]>),
}

#[derive(Debug)]
pub enum ParamGetSet {
    Int(ValueGetSet<i32>),
    Float(ValueGetSet<f32>),
    String(ValueGetSet<String>),
    Time(ValueGetSet<(u32, u32)>),
    Long(ValueGetSet<i64>),
    Double(ValueGetSet<f64>),
    Char(ValueGetSet<char>),
    Midi(ValueGetSet<(u8, u8, u8, u8)>),
    Bool(ValueGetSet<bool>),
    //TODO Blob(ValueGetSet<Box<[u8]>>), //does clip mode make and range make sense?
    //TODO Array(Box<[Self]>),
}
