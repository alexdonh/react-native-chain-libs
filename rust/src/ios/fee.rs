use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::{Fee, PerCertificateFee, Transaction, Value};

#[no_mangle]
pub unsafe extern "C" fn fee_linear_fee(
  constant: RPtr, coefficient: RPtr, certificate: RPtr, per_certificate_fee: RPtr,
  result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    constant
      .typed_ref::<Value>()
      .zip(coefficient.typed_ref::<Value>())
      .zip(certificate.typed_ref::<Value>())
      .zip(per_certificate_fee.typed_ref::<PerCertificateFee>())
      .map(|(((constant, coefficient), certificate), per_certificate_fee)| {
        Fee::linear_fee(constant, coefficient, certificate, per_certificate_fee)
      })
  })
  .map(|fee| fee.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fee_calculate(
  fee: RPtr, tx: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fee.typed_ref::<Fee>().zip(tx.typed_ref::<Transaction>()).map(|(fee, tx)| fee.calculate(tx))
  })
  .map(|value| value.rptr())
  .response(result, error)
}
