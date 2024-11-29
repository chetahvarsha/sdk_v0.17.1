use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/erc20.wasm",
		Box::new(|context| Box::new(erc20::contract_obj(context))),
	);
	contract_map
}

#[test]
fn allowance_callercaller_rs() {
	numbat_wasm_debug::denali_rs("denali/allowance_CallerCaller.scen.json", &contract_map());
}

#[test]
fn allowance_callerother_rs() {
	numbat_wasm_debug::denali_rs("denali/allowance_CallerOther.scen.json", &contract_map());
}

#[test]
fn allowance_othercaller_rs() {
	numbat_wasm_debug::denali_rs("denali/allowance_OtherCaller.scen.json", &contract_map());
}

#[test]
fn allowance_othereqother_rs() {
	numbat_wasm_debug::denali_rs("denali/allowance_OtherEqOther.scen.json", &contract_map());
}

#[test]
fn allowance_otherneqother_rs() {
	numbat_wasm_debug::denali_rs("denali/allowance_OtherNEqOther.scen.json", &contract_map());
}

#[test]
fn approve_caller_positive_rs() {
	numbat_wasm_debug::denali_rs("denali/approve_Caller-Positive.scen.json", &contract_map());
}

#[test]
fn approve_caller_zero_rs() {
	numbat_wasm_debug::denali_rs("denali/approve_Caller-Zero.scen.json", &contract_map());
}

#[test]
fn approve_other_positive_rs() {
	numbat_wasm_debug::denali_rs("denali/approve_Other-Positive.scen.json", &contract_map());
}

#[test]
fn approve_other_zero_rs() {
	numbat_wasm_debug::denali_rs("denali/approve_Other-Zero.scen.json", &contract_map());
}

#[test]
fn approve_switchcaller_rs() {
	numbat_wasm_debug::denali_rs("denali/approve_SwitchCaller.scen.json", &contract_map());
}

#[test]
fn balanceof_caller_rs() {
	numbat_wasm_debug::denali_rs("denali/balanceOf_Caller.scen.json", &contract_map());
}

#[test]
fn balanceof_noncaller_rs() {
	numbat_wasm_debug::denali_rs("denali/balanceOf_NonCaller.scen.json", &contract_map());
}

#[test]
fn not_payable_rs() {
	numbat_wasm_debug::denali_rs("denali/not_payable.scen.json", &contract_map());
}

#[test]
fn totalsupply_positive_rs() {
	numbat_wasm_debug::denali_rs("denali/totalSupply_Positive.scen.json", &contract_map());
}

#[test]
fn totalsupply_zero_rs() {
	numbat_wasm_debug::denali_rs("denali/totalSupply_Zero.scen.json", &contract_map());
}

#[test]
fn transferfrom_alldistinct_balanceeqallowance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_AllDistinct-BalanceEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_balanceneqallowance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_AllDistinct-BalanceNEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_entireallowancemorethanbalance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_AllDistinct-EntireAllowanceMoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_entirebalanceeqallowance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_AllDistinct-EntireBalanceEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_entirebalancemorethanallowance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_AllDistinct-EntireBalanceMoreThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_morethanallowancelessthanbalance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_AllDistinct-MoreThanAllowanceLessThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_morethanbalancelessthanallowance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_AllDistinct-MoreThanBalanceLessThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_nooverflow_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_AllDistinct-NoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_stillnooverflow_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_AllDistinct-StillNoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_allequal_allowancerelevant_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_AllEqual-AllowanceRelevant.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_allequal_entirebalance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_AllEqual-EntireBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqfrom_allowancerelevant_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_CallerEqFrom-AllowanceRelevant.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqfrom_entirebalance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_CallerEqFrom-EntireBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqfrom_morethanbalance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_CallerEqFrom-MoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqto_balanceneqallowance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_CallerEqTo-BalanceNEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqto_morethanallowancelessthanbalance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_CallerEqTo-MoreThanAllowanceLessThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqto_morethanbalancelessthanallowance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_CallerEqTo-MoreThanBalanceLessThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_exploratory_multipletransferssucceed_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_Exploratory-MultipleTransfersSucceed.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_exploratory_multipletransfersthrow_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_Exploratory-MultipleTransfersThrow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_balanceeqallowance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_FromEqTo-BalanceEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_balanceneqallowance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_FromEqTo-BalanceNEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_entireallowancemorethanbalance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_FromEqTo-EntireAllowanceMoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_entirebalanceeqallowance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_FromEqTo-EntireBalanceEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_entirebalancemorethanallowance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_FromEqTo-EntireBalanceMoreThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_morethanallowancelessthanbalance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_FromEqTo-MoreThanAllowanceLessThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_morethanbalancelessthanallowance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_FromEqTo-MoreThanBalanceLessThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_nooverflow_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transferFrom_FromEqTo-NoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_allowanceirrelevant_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transfer_Caller-AllowanceIrrelevant.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_entirebalance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transfer_Caller-EntireBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_morethanbalance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transfer_Caller-MoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_nooverflow_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transfer_Caller-NoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_positive_rs() {
	numbat_wasm_debug::denali_rs("denali/transfer_Caller-Positive.scen.json", &contract_map());
}

#[test]
fn transfer_caller_stillnooverflow_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transfer_Caller-StillNoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_zero_rs() {
	numbat_wasm_debug::denali_rs("denali/transfer_Caller-Zero.scen.json", &contract_map());
}

#[test]
fn transfer_other_allowanceirrelevant_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transfer_Other-AllowanceIrrelevant.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_entirebalance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transfer_Other-EntireBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_morethanbalance_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transfer_Other-MoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_nooverflow_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transfer_Other-NoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_positive_rs() {
	numbat_wasm_debug::denali_rs("denali/transfer_Other-Positive.scen.json", &contract_map());
}

#[test]
fn transfer_other_stillnooverflow_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/transfer_Other-StillNoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_zero_rs() {
	numbat_wasm_debug::denali_rs("denali/transfer_Other-Zero.scen.json", &contract_map());
}
