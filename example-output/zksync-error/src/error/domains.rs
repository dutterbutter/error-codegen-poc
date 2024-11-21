
//!
//! AUTOGENERATED BASED ON A SET OF JSON FILES, DO NOT EDIT MANUALLY
//!

use crate::error::ICustomError;
use crate::error::IUnifiedError;
use crate::kind::Kind;
use strum_macros::EnumDiscriminants;
use strum_macros::FromRepr;
use crate::error::definitions::Sequencer;
use crate::error::definitions::SequencerCode;
use crate::error::definitions::Zksolc;
use crate::error::definitions::ZksolcCode;

#[repr(i32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ZksyncError {
   Core(Core), 
   Compiler(Compiler), 
}

impl ZksyncError {
    pub fn get_kind(&self) -> crate::kind::Kind {
        match self {
         ZksyncError::Core(Core::Sequencer(_)) => { Kind::Core(CoreCode::Sequencer) },
         ZksyncError::Compiler(Compiler::Zksolc(_)) => { Kind::Compiler(CompilerCode::Zksolc) },
      }
   }
   
    pub fn get_code(&self) -> i32 {
        match self {
         ZksyncError::Core(Core::Sequencer(error)) => { Into::<SequencerCode>::into(error) as i32 },
         ZksyncError::Compiler(Compiler::Zksolc(error)) => { Into::<ZksolcCode>::into(error) as i32 },
      }
   }
}

impl std::fmt::Display for ZksyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}
impl IUnifiedError<ZksyncError> for ZksyncError {}
impl std::error::Error for ZksyncError {}


#[repr(i32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(CoreCode))]
#[strum_discriminants(derive(serde::Serialize, serde::Deserialize, FromRepr))]
#[strum_discriminants(vis(pub))]
pub enum Core {
   Sequencer(Sequencer),
}


#[repr(i32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(CompilerCode))]
#[strum_discriminants(derive(serde::Serialize, serde::Deserialize, FromRepr))]
#[strum_discriminants(vis(pub))]
pub enum Compiler {
   Zksolc(Zksolc),
}


impl ICustomError<ZksyncError, ZksyncError> for Sequencer {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::Core(Core::Sequencer(self.clone()))
    }
}


impl ICustomError<ZksyncError, ZksyncError> for Zksolc {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::Compiler(Compiler::Zksolc(self.clone()))
    }
}

