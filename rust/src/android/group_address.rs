use super::ptr_j::*;
use super::result::ToJniResult;
use super::string::ToJniString;
use super::string::ToString;
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::GroupAddress;
use std::convert::TryFrom;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_groupAddressGetAccountKey(
  env: JNIEnv, _: JObject, group_address: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let group_address = group_address.rptr(&env)?;
    group_address
      .typed_ref::<GroupAddress>()
      .map(|group_address| group_address.get_account_key())
      .and_then(|public_key| public_key.rptr().jptr(&env))
  })
  .jresult(&env)
}
