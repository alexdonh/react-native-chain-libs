use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{Fee, PerCertificateFee, Transaction, Value};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_feeLinearFee(
  env: JNIEnv, _: JObject, constant: JRPtr, coefficient: JRPtr, certificate: JRPtr,
  per_certificate_fee: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let constant = constant.rptr(&env)?;
    let coefficient = coefficient.rptr(&env)?;
    let certificate = certificate.rptr(&env)?;
    let per_certificate_fee = per_certificate_fee.rptr(&env)?;
    constant
      .typed_ref::<Value>()
      .zip(coefficient.typed_ref::<Value>())
      .zip(certificate.typed_ref::<Value>())
      .zip(per_certificate_fee.typed_ref::<PerCertificateFee>())
      .map(|(((constant, coefficient), certificate), per_certificate_fee)| {
        Fee::linear_fee(constant, coefficient, certificate, per_certificate_fee)
      })
      .and_then(|fee| fee.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_feeCalculate(
  env: JNIEnv, _: JObject, fee: JRPtr, tx: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let fee = fee.rptr(&env)?;
    let tx = tx.rptr(&env)?;
    fee
      .typed_ref::<Fee>()
      .zip(tx.typed_ref::<Transaction>())
      .map(|(fee, tx)| fee.calculate(tx))
      .and_then(|value| value.rptr().jptr(&env))
  })
  .jresult(&env)
}
