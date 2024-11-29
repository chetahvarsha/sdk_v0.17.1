use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/basic-features.wasm",
		Box::new(|context| Box::new(basic_features::contract_obj(context))),
	);
	contract_map
}

#[test]
fn block_info_rs() {
	numbat_wasm_debug::denali_rs("denali/block_info.scen.json", &contract_map());
}

#[test]
fn boxed_bytes_zeros_rs() {
	numbat_wasm_debug::denali_rs("denali/boxed_bytes_zeros.scen.json", &contract_map());
}

#[test]
fn count_ones_rs() {
	numbat_wasm_debug::denali_rs("denali/count_ones.scen.json", &contract_map());
}

#[test]
fn crypto_keccak256_rs() {
	numbat_wasm_debug::denali_rs("denali/crypto_keccak256.scen.json", &contract_map());
}

#[test]
fn crypto_sha256_rs() {
	numbat_wasm_debug::denali_rs("denali/crypto_sha256.scen.json", &contract_map());
}

#[test]
fn echo_array_u8_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_array_u8.scen.json", &contract_map());
}

#[test]
fn echo_async_result_empty_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_async_result_empty.scen.json", &contract_map());
}

#[test]
fn echo_big_int_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_big_int.scen.json", &contract_map());
}

#[test]
fn echo_big_uint_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_big_uint.scen.json", &contract_map());
}

#[test]
fn echo_boxed_bytes_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_boxed_bytes.scen.json", &contract_map());
}

#[test]
fn echo_i32_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_i32.scen.json", &contract_map());
}

#[test]
fn echo_i64_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_i64.scen.json", &contract_map());
}

#[test]
fn echo_nothing_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_nothing.scen.json", &contract_map());
}

#[test]
fn echo_ser_ex_1_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_ser_ex_1.scen.json", &contract_map());
}

#[test]
fn echo_slice_u8_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_slice_u8.scen.json", &contract_map());
}

#[test]
fn echo_str_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_str.scen.json", &contract_map());
}

#[test]
fn echo_str_box_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_str_box.scen.json", &contract_map());
}

#[test]
fn echo_string_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_string.scen.json", &contract_map());
}

#[test]
fn echo_u64_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_u64.scen.json", &contract_map());
}

#[test]
fn echo_usize_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_usize.scen.json", &contract_map());
}

#[test]
fn echo_varags_tuples_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_varags_tuples.scen.json", &contract_map());
}

#[test]
fn echo_varargs_u32_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_varargs_u32.scen.json", &contract_map());
}

#[test]
fn echo_vec_u8_rs() {
	numbat_wasm_debug::denali_rs("denali/echo_vec_u8.scen.json", &contract_map());
}

#[test]
fn events_rs() {
	numbat_wasm_debug::denali_rs("denali/events.scen.json", &contract_map());
}

#[test]
fn events_legacy_rs() {
	numbat_wasm_debug::denali_rs("denali/events_legacy.scen.json", &contract_map());
}

#[test]
fn get_caller_rs() {
	numbat_wasm_debug::denali_rs("denali/get_caller.scen.json", &contract_map());
}

#[test]
fn get_cumulated_validator_rewards_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/get_cumulated_validator_rewards.scen.json",
		&contract_map(),
	);
}

// TODO: uncomment after implemented the full DCDT format in denali-rs
// #[test]
// fn get_dcdt_local_roles_rs() {
// 	numbat_wasm_debug::denali_rs(
// 		"denali/get_dcdt_local_roles.scen.json",
// 		&contract_map(),
// 	);
// }

#[test]
fn panic_rs() {
	numbat_wasm_debug::denali_rs("denali/panic.scen.json", &contract_map());
}

#[test]
fn return_codes_rs() {
	numbat_wasm_debug::denali_rs("denali/return_codes.scen.json", &contract_map());
}

// TODO: Fix by implementing is_smart_contract mock
/*
#[test]
fn sc_properties_rs() {
	numbat_wasm_debug::denali_rs("denali/sc_properties.scen.json", &contract_map());
}
*/

#[test]
fn sc_result_rs() {
	numbat_wasm_debug::denali_rs("denali/sc_result.scen.json", &contract_map());
}

#[test]
fn storage_addr_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_addr.scen.json", &contract_map());
}

#[test]
fn storage_big_int_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_big_int.scen.json", &contract_map());
}

#[test]
fn storage_big_uint_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_big_uint.scen.json", &contract_map());
}

#[test]
fn storage_bool_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_bool.scen.json", &contract_map());
}

#[test]
fn storage_clear_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_clear.scen.json", &contract_map());
}

#[test]
fn storage_i64_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_i64.scen.json", &contract_map());
}

#[test]
fn storage_i64_bad_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_i64_bad.scen.json", &contract_map());
}

#[test]
fn storage_map1_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_map1.scen.json", &contract_map());
}

#[test]
fn storage_map2_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_map2.scen.json", &contract_map());
}

#[test]
fn storage_map3_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_map3.scen.json", &contract_map());
}

#[test]
fn storage_mapper_linked_list_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/storage_mapper_linked_list.scen.json",
		&contract_map(),
	);
}

#[test]
fn storage_mapper_map_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_mapper_map.scen.json", &contract_map());
}

#[test]
fn storage_mapper_set_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_mapper_set.scen.json", &contract_map());
}

#[test]
fn storage_mapper_map_storage_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/storage_mapper_map_storage.scen.json",
		&contract_map(),
	);
}

#[test]
fn storage_mapper_single_value_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/storage_mapper_single_value.scen.json",
		&contract_map(),
	);
}

#[test]
fn storage_mapper_vec_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_mapper_vec.scen.json", &contract_map());
}

#[test]
fn storage_opt_addr_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_opt_addr.scen.json", &contract_map());
}

#[test]
fn storage_reserved_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_reserved.scen.json", &contract_map());
}

#[test]
fn storage_u64_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_u64.scen.json", &contract_map());
}

#[test]
fn storage_u64_bad_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_u64_bad.scen.json", &contract_map());
}

#[test]
fn storage_usize_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_usize.scen.json", &contract_map());
}

#[test]
fn storage_usize_bad_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_usize_bad.scen.json", &contract_map());
}

#[test]
fn storage_vec_u8_rs() {
	numbat_wasm_debug::denali_rs("denali/storage_vec_u8.scen.json", &contract_map());
}
