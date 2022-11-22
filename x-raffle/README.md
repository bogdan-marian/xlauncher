# Notes

## raffle requirements 22/11/2022

### next round needs to be added to storage
```rust  
  #[view(getNextRoundId)]
  #[storage_mapper("previous_round_id")]
  fn previous_round_id(&self) -> SingleValueMapper<usize>;
```
 
### open round
- if `there is no current round`
    - it creates a new round
    - sets the new created round as `current round`
    - set status to `Opened`
- else
  - move the current round to `Pending`
  - create nextRound with status `Opened`

### finish_round
- check that current round status is `Pending`
- check that next round status is `Opened`
- select the winners
- set current round status to `Claimable`

### inject_prize
- if current round status is `Opened` inject prizes in current round
- else check that nextRound status is `Opened` and inject prises in next round.
- else fail

### buy tickets
- it generates a unique ticket number per round.
- users are all the time able to vew the ticket numbers that they have for each round
  - user 1 could have [001, 005, 007]
  - user 3 could have [002, 003]
  - user 4 could have [004, 006]
- we need to be able to report also that 007 is the winning ticket. (is not always first ticket that user 1 holds. this also should be random)

