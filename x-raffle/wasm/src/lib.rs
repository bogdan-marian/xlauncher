////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    x_raffle
    (
        buyTickets
        claimPrize
        finishAndStartNewRound
        getCurrentRoundId
        getCurrentRoundStatus
        getNumberOfWinners
        getPrizePercentages
        getRound
        getRoundEndTimestamp
        getRoundLeftTokens
        getRoundNumberOfWinners
        getRoundPrizePercentages
        getRoundPrizeTokens
        getRoundStartTimestamp
        getRoundStatus
        getRoundTicketPrice
        getRoundUserRanking
        getRoundUserTickets
        getRoundWinners
        getTicketPrice
        getTicketToken
        injectPrize
        openGenesisRound
        setPrizeSettings
        setSettings
        setTicketPrice
        setTicketToken
    )
}

elrond_wasm_node::wasm_empty_callback! {}
