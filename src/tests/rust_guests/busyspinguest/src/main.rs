#![no_std]
#![no_main]

extern crate alloc;
extern crate hyperlight_guest;

use alloc::string::ToString;
use alloc::vec::Vec;
use core::arch::x86_64::{_mm_cvtsd_f64, _mm_set_sd, _mm_sqrt_sd};
use core::hint::black_box;
use hyperlight_common::flatbuffer_wrappers::function_call::FunctionCall;
use hyperlight_common::flatbuffer_wrappers::function_types::{
    ParameterType, ParameterValue, ReturnType,
};
use hyperlight_common::flatbuffer_wrappers::guest_error::ErrorCode;
use hyperlight_common::flatbuffer_wrappers::util::get_flatbuffer_result;
use hyperlight_guest::error::{HyperlightGuestError, Result};
use hyperlight_guest_bin::guest_function::definition::GuestFunctionDefinition;
use hyperlight_guest_bin::guest_function::register::register_function;

const EXEC_UNIT: usize = 100;

fn take_sqrts() -> f64 {
    let mut tmp: f64 = 0.0;
    for _ in 0..EXEC_UNIT {
        unsafe {
            // Use SSE2 sqrt instruction equivalent to C.SQRTSD
            // black_box prevents compiler optimizations
            let input = _mm_set_sd(black_box(10.0));
            let result = _mm_sqrt_sd(input, input);
            tmp = _mm_cvtsd_f64(result);
        }
        black_box(tmp);
    }
    tmp
}

fn busy_spin(function_call: FunctionCall) -> Result<Vec<u8>> {
    if let (ParameterValue::UInt(requested_cpu_time), ParameterValue::UInt(multiplier)) = (
        function_call.parameters.clone().unwrap()[0].clone(),
        function_call.parameters.clone().unwrap()[1].clone(),
    ) {
        let total_iterations = multiplier * requested_cpu_time;
        for _ in 0..total_iterations {
            let result = take_sqrts();
            black_box(result);
        }
        Ok(get_flatbuffer_result(0))
    } else {
        Err(HyperlightGuestError::new(
            ErrorCode::GuestFunctionParameterTypeMismatch,
            "Invalid parameters passed to busy_spin".to_string(),
        ))
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn hyperlight_main() {
    let busy_spin_def = GuestFunctionDefinition::new(
        "BusySpin".to_string(),
        Vec::from(&[ParameterType::UInt, ParameterType::UInt]),
        ReturnType::VecBytes,
        busy_spin as usize,
    );

    register_function(busy_spin_def);
}

#[unsafe(no_mangle)]
pub fn guest_dispatch_function(function_call: FunctionCall) -> Result<Vec<u8>> {
    let function_name = function_call.function_name.clone();
    return Err(HyperlightGuestError::new(
        ErrorCode::GuestFunctionNotFound,
        function_name,
    ));
}
