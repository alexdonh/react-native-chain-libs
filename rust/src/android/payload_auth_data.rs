use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::handle_exception_result;
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{PayloadAuthData, StakeDelegationAuthData};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_payloadAuthDataForNoPayload(
  env: JNIEnv, _: JObject
) -> jobject {
  handle_exception_result(|| PayloadAuthData::for_no_payload().rptr().jptr(&env)).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_payloadAuthDataForStakeDelegation(
  env: JNIEnv, _: JObject, stake_delegation_auth_data: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let stake_delegation_auth_data = stake_delegation_auth_data.rptr(&env)?;
    stake_delegation_auth_data
      .typed_ref::<StakeDelegationAuthData>()
      .and_then(|stake_delegation_auth_data| PayloadAuthData::for_stake_delegation(stake_delegation_auth_data).rptr().jptr(&env))
  })
  .jresult(&env)
}
