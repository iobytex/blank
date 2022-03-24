#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::unnecessary_wraps)]

use frame_support::{
    weights::{ClassifyDispatch, DispatchClass, Pays, Weight},
    dispatch::{DispatchResult, PaysFee, WeighData},
	ensure,
};

// struct tuple zero index is for Database read and write
pub struct CreateProductWeight(pub u64);

impl WeighData<(&u128,&u32,&u32,&u32)> for CreateProductWeight{
    fn weigh_data(&self, target: (&u128,&u32,&u32,&u32)) -> Weight {
        todo!()
    }
}

impl ClassifyDispatch<(&u128,&u32,&u32,&u32)> for CreateProductWeight {
    fn classify_dispatch(&self, target: (&u128,&u32,&u32,&u32)) -> DispatchClass {
        todo!()
    }
}

impl PaysFee<(&u128,&u32,&u32,&u32)> for CreateProductWeight{
    fn pays_fee(&self, _target: (&u128,&u32,&u32,&u32)) -> Pays {
        todo!()
    }
}