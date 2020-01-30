use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::handle_exception_result;
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{Certificate, Payload};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_payloadNoPayload(
  env: JNIEnv, _: JObject
) -> jobject {
  handle_exception_result(|| Payload::no_payload().rptr().jptr(&env)).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_payloadCertificate(
  env: JNIEnv, _: JObject, certificate: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let certificate = certificate.rptr(&env)?;
    certificate
      .typed_ref::<Certificate>()
      .and_then(|certificate| Payload::certificate(certificate).rptr().jptr(&env))
  })
  .jresult(&env)
}
