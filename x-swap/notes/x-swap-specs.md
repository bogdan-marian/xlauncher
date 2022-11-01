# Swap Workflow

- Users can trade 3 tokens - WEGLD, USDC, LKMEX (We implement all of them except LKMEX first.)
- There are 2 types of offers - Full Offer and Partial Offer.
    - One needs to accept full amount of Full Offer.
    - One can accept part of Partial Offer or accept full of Partial Offer.
- There will be a minimum percentage of accepting Partial Offer at once. For example, if minimum percetage is 20% and Partial Offer is 10 WEGLD for 1000 USDC, one needs to pay 200 USDC to accept Partial Offer.
- Fee will be taken when an Offer is accepted (several times for Partial Offer). (Or we can take fee when one creates an offer.)
    - 90% of Fee will be sent to the Treasury wallet and 10% of Fee will be sent to the Raffle Smart Contract
- If accepted amount of Offer is bigger than 0.5 EGLD, XRF tokens correspoinding to accepted amount of offer will be sent to both of creator and acceptor.

# Raffle Workflow

- Raffle are divided in to rounds and one Round is 15 days.
- Each Round, Users will buy raffle tickets with XRF token.
    - Ticket price will be settled in the beginning of each Round. (For example, 10 XRF for one ticket.)
    - Number of winners and percentage of rewards will also be settled in the beginning.
- While the Round is going, rewards from X-Swap will be accumulated in the Raffle Smart Contract.
- At the end of the Round, the Raffle SC will decide winners and it will directly distribute rewards to the winners.
    - One User can win one prize at each Round. If he wins several prizes, only the highest prize will be rewarded to him and SC will select others.


# Swap Implementation (Real code will look different ;)
- fn createOffer(creator: ManagedAddress, offerTokenPayment: TokenPayment, wantTokenPayment: TokenPayment, minAcceptAmount: BigUint)
- fn acceptOffer(acceptor: ManagedAddress, offerTokenPayment: TokenPayment, wantTokenPayment: TokenPayment)

# Raffle Implementation
- fn startRound(duration: u64, ticketPrice: BigUint, numberOfWinners: u32, prize_percentages: ManagedVec<u32>)
- fn buyTickets(#[payment])
- fn closeRound()  - This will select winners and distribute tokens