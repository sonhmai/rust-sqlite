use std::fmt::Debug;
use crate::model::data_record::DataRecord;


pub trait Exec: Debug {
    // TODO use Iterator?
    fn execute(&mut self) -> Vec<DataRecord>;
}
