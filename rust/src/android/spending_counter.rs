use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::handle_exception_result;
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::SpendingCounter;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_spendingCounterZero(
  env: JNIEnv, _: JObject
) -> jobject {
  handle_exception_result(|| RPtr::new(SpendingCounter::zero()).jptr(&env)).jresult(&env)
}
