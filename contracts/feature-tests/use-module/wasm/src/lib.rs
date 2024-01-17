// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           62
// Async Callback:                       1
// Total number of exported functions:  64

#![no_std]
#![feature(alloc_error_handler, lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    use_module
    (
        checkFeatureGuard
        checkPause
        call_contract_base_full_path_endpoint
        call_contract_base_endpoint
        call_mod_a
        call_mod_b
        call_mod_c
        only_owner_mod_endpoint
        call_derived_not_owner_only
        only_admin_mod_endpoint
        call_derived_not_admin_only
        countTo100
        mergeTokens
        mergeTokensCustomAttributes
        splitTokens
        splitTokenPartial
        claimDeveloperRewards
        dnsRegister
        issueToken
        setFeatureFlag
        depositTokensForProposal
        withdrawGovernanceTokens
        propose
        vote
        queue
        execute
        cancel
        getProposalStatus
        getProposer
        getProposalDescription
        getProposalActions
        getProposalVotes
        getTotalVotes
        getTotalDownvotes
        changeQuorum
        changeMinTokenBalanceForProposing
        changeVotingDelayInBlocks
        changeVotingPeriodInBlocks
        changeLockTimeAfterVotingEndsInBlocks
        getGovernanceTokenId
        getQuorum
        getMinFeeForPropose
        getMinTokenBalanceForProposing
        getVotingDelayInBlocks
        getVotingPeriodInBlocks
        getLockTimeAfterVotingEndsInBlocks
        pause
        unpause
        isPaused
        stake
        unstake
        voteSlashMember
        slashMember
        issueMergedToken
        addMergeableTokensToWhitelist
        removeMergeableTokensFromWhitelist
        getMergedTokenId
        getMergeableTokensWhitelist
        isAdmin
        addAdmin
        removeAdmin
        getAdmins
        callBack
    )
}
