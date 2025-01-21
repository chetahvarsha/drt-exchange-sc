// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            4
// Async Callback (empty):               1
// Total number of exported functions:   6

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    energy_factory_mock
    (
        init => init
        setUserEnergy => set_user_energy
        getEnergyAmountForUser => get_energy_amount_for_user
        getEnergyEntryForUser => get_energy_entry_for_user
        setUserEnergyAfterLockedTokenTransfer => set_user_energy_after_locked_token_transfer
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
