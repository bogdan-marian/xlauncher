# x-swap notes

# basic swap use case
- Seller 1: I want to sell 1k XLH for 1 EGLD
- Seller 2: I want to sell 1k ESTAR for 1 EGLD
- Buyer 1: Accepts the offer from Seller 1
- Bayer 2: Accepts the offer from Seller 2

We have 2 wallets
- raffle contract wallet: 10% from transaction fees. 
- XLH community: 90% from transaction fees. 

# basic swap if bigger than specified amount
- if swap value > 0.5 egld
- when buyer accepts offer the seller and buyer accepts 1 XRF token (X raffle token)
- The contract generates XRF tokens only on integer division steps of value 0.5 Egld. 
  - ex: 1.49 < SwapValue < 1.5 ==> Contract generates 4 XRF tokens. 2 for buyer and 2 for seller. 

# how the raffle works
- We keep count of the raffle prize pull value
- we hve programmable number of winners each with specific win percentage from prize pull. 
  - example for 5 winners
    - winner 1: 45%
    - winner 2: 25%
    - winner 3: 15%
    - winner 4: 10%
    - winner 5: 5%
- each 15 days (number of days would be custom) we select winners and start a new cycle. 
  - raffle contract sends directly the prise to the winners. 
