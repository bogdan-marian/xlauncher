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
        getLastTickerNumber
        getNumberOfWinners
        getPrizePercentages
        getRound
        getRoundEndTimestamp
        getRoundFirstTicketNumber
        getRoundLeftTokens
        getRoundNumberOfWinners
        getRoundPrizePercentages
        getRoundPrizeTokens
        getRoundStartTimestamp
        getRoundStatus
        getRoundTicketPrice
        getRoundUserTicketNumbers
        getRoundWinNumbers
        getTicketClaimed
        getTicketOwner
        getTicketPrice
        getTicketPrizeRanking
        getTicketToken
        getUser
        injectPrize
        openGenesisRound
        setPrizeSettings
        setSettings
        setTicketPrice
        setTicketToken
    )
}

elrond_wasm_node::wasm_empty_callback! {}
