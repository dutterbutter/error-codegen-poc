
//!
//! AUTOGENERATED BASED ON A SET OF JSON FILES, DO NOT EDIT MANUALLY
//!

use crate::error::CustomErrorMessage;
use strum_macros::EnumDiscriminants;

#[repr(i32)]
#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(SequencerCode))]
#[strum_discriminants(vis(pub))]
#[non_exhaustive]
pub enum Sequencer {
   SomeCoreError { 
      path : String,
      payload : u32,
   } = 1, 
   OtherCoreError { 
      path : String,
      payload : u32,
   } = 2, 
   
} // end of Sequencer


impl CustomErrorMessage for Sequencer {
    fn get_message(&self) -> String {
        match self {
         Sequencer::SomeCoreError { 
            path,
            payload,
         }
          => { format!("[core-seq-1] Hahaha") },
         Sequencer::OtherCoreError { 
            path,
            payload,
         }
          => { format!("[core-seq-2] Tratata {path}") },
      }
   }
}

#[repr(i32)]
#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(ZksolcCode))]
#[strum_discriminants(vis(pub))]
#[non_exhaustive]
pub enum Zksolc {
   Umbrella { 
      inner : serde_json::Value,
   } = 42, 
   SolcNotFound { 
      path : String,
      payload : u32,
   } = 1, 
   FileNotFound { 
      path : String,
      file_index : u32,
   } = 2, 
   
} // end of Zksolc


impl CustomErrorMessage for Zksolc {
    fn get_message(&self) -> String {
        match self {
         Zksolc::Umbrella { 
            inner,
         }
          => { format!("[comp-zksolc-42] Any error!") },
         Zksolc::SolcNotFound { 
            path,
            payload,
         }
          => { format!("[comp-zksolc-1] I just can't find solc!") },
         Zksolc::FileNotFound { 
            path,
            file_index,
         }
          => { format!("[comp-zksolc-2] Can't find the file {path} to compile.") },
      }
   }
}
