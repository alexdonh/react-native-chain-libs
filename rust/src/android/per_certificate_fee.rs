use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{PerCertificateFee, Value};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_perCertificateFeeNew(
  env: JNIEnv, _: JObject
) -> jobject {
  handle_exception_result(|| PerCertificateFee::new().rptr().jptr(&env)).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_perCertificateFeeSetPoolRegistration(
  env: JNIEnv, _: JObject, per_certificate_fee: JRPtr, val: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let per_certificate_fee = per_certificate_fee.rptr(&env)?;
    let val = val.rptr(&env)?;
    per_certificate_fee
      .typed_ref::<PerCertificateFee>()
      .zip(val.typed_ref::<Value>())
      .map(|(per_certificate_fee, val)| {
        PerCertificateFee::set_pool_registration(per_certificate_fee, val)
      })
      .map(|_| JObject::null())
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_perCertificateFeeSetStakeDelegation(
  env: JNIEnv, _: JObject, per_certificate_fee: JRPtr, val: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let per_certificate_fee = per_certificate_fee.rptr(&env)?;
    let val = val.rptr(&env)?;
    per_certificate_fee
      .typed_ref::<PerCertificateFee>()
      .zip(val.typed_ref::<Value>())
      .map(|(per_certificate_fee, val)| {
        PerCertificateFee::set_stake_delegation(per_certificate_fee, val)
      })
      .map(|_| JObject::null())
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_perCertificateFeeSetOwnerStakeDelegation(
  env: JNIEnv, _: JObject, per_certificate_fee: JRPtr, val: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let per_certificate_fee = per_certificate_fee.rptr(&env)?;
    let val = val.rptr(&env)?;
    per_certificate_fee
      .typed_ref::<PerCertificateFee>()
      .zip(val.typed_ref::<Value>())
      .map(|(per_certificate_fee, val)| {
        PerCertificateFee::set_owner_stake_delegation(per_certificate_fee, val)
      })
      .map(|_| JObject::null())
  })
  .jresult(&env)
}
