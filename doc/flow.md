# The flow

## Selectors
### MoveSelectors
The move selectors will supply the caller with a list of moves depending on the state of the Solution. They expect that the same data one which they base their calcuation is given to the move itself. Between the call to get the move list and the call to do the move the instance may change, the data inside may not.