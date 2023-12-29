use std::ops::Range;

#[derive(Clone,Copy,Debug)]
pub enum ConditionResult<'a> {
    Nothing,
    Approved,
    Rejected,
    Next(&'a str)
}

#[derive(Clone,Debug)]
pub enum ConditionResult2<'a> {
    FallThrough(&'a str, [Range<u32>; 4]),
    Approved([Range<u32>; 4]),
    Rejected([Range<u32>; 4]),
    Next(&'a str, [Range<u32>; 4], [Range<u32>; 4])
}
