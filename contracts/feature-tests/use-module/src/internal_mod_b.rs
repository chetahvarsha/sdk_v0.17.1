numbat_wasm::imports!();

/// Contains all events that can be emitted by the contract.
#[numbat_wasm_derive::module]
pub trait InternalModuleB {
	#[view]
	fn call_mod_b(&self) {}
}
