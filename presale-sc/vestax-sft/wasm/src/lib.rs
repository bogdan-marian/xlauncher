// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           17
// Async Callback (empty):               1
// Total number of exported functions:  19

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    xlauncher_presale
    (
        fundContract
        getTokenBalance
        getSftBalance
        buy
        buySft
        collect
        setTokenInfo
        fundWithSft
        getCollectionIdentifier
        getNonce
        getTokenId
        getPrice
        getMinAmount
        getMaxAmount
        getMaxBalance
        getClientList
        getClientBoughtValue
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
