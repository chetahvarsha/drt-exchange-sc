// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            2
// Async Callback (empty):               1
// Total number of exported functions:   4

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    pair_mock
    (
        init => init
        addInitialLiquidity => add_initial_liquidity
        getTokensForGivenPositionWithSafePrice => get_tokens_for_given_position_with_safe_price
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
