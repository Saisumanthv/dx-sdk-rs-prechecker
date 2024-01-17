// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           19
// Async Callback (empty):               1
// Total number of exported functions:  21

#![no_std]
#![feature(alloc_error_handler, lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    dharitri_price_aggregator_sc
    (
        addOracles
        removeOracles
        submit
        submitBatch
        latestRoundData
        latestPriceFeed
        latestPriceFeedOptional
        setSubmissionCount
        getOracles
        setPairDecimals
        getPairDecimals
        submission_count
        pause
        unpause
        isPaused
        stake
        unstake
        voteSlashMember
        slashMember
    )
}

dharitri_sc_wasm_adapter::empty_callback! {}