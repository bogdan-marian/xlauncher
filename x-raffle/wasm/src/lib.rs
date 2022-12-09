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
        getRoundUsers
        getRoundUsersStats
        getRoundWinNumbers
        getTicketClaimed
        getTicketOwner
        getTicketPrice
        getTicketPrizeRanking
        getTicketToken
        getTreasuryAddress
        getUserRounds
        injectPrize
        openGenesisRound
        setPrizeSettings
        setSettings
        setTicketPrice
        setTicketToken
        setTreasuryAddress
    )
}

elrond_wasm_node::wasm_empty_callback! {}
