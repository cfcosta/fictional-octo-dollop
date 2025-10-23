# Ledger Experiment

This is a simple, toy payments engine to read a series of transactions from a CSV, handle ledger invariants and output the final state of clients account as CSV as well.

## Implementation Notes

- We have a hard limit on the number of clients (`u16::MAX`), so instead of allocating the state as we go, we pre-allocate all of it then iterate, using a Data-Driven Design approach. This avoids most of the allocation problems that could occur as we process bigger and bigger files. The whole of the ledger state (besides Transactions) takes less than 5mb of memory, so this process is very fast and very efficient.
- Transaction ids have a limit of `u32::MAX`, which means that all the transaction space can be processed in a little bit over 32 gigabytes of memory. Because of this limitation, we can not just pre-allocate all the possible transactions like we did for clients, but we still need to save them to handle disputes, resolves and chargebacks.
- We save the transactions inside a HashMap. There are some ways to improve the performance further if necessary:
    - Data can be offload to disk instead of in-memory to trade I/O performance for less memory consumption.
    - Since `u32` (4 bytes) is already the address space size for the hashed input, we could implement `std::hash::Hasher` and `std::hash::BuildHasher` to directly bypass the hashing process, making the HashMap work more like a sparse vector. This is **not** safe against adversarial input, but it's not a problem if we know the generated keys are trusted.
- We log every import error on STDERR to avoid polluting the CSV output. We continue importing the batch even in the case of errors.
- The implementation has been tested by running `cargo run` against the test files present on `tests/fixtures`. On a more complete implementation, using either [Snapshot Testing](https://insta.rs/) or generating random inputs through [proptest](https://docs.rs/proptest/latest/proptest/) and [proptest-state-machine](https://docs.rs/proptest-state-machine/latest/proptest_state_machine/) would be more reliable.
    - We do a very simplified form of property testing on the `main` function, by asserting all the invariants we expect.
- Errors are handled through the `Error` enum (using `thiserror` to avoid most of the error generating boilerplate).
- The dataset is read as a stream, applying each item directly. A more complicated operation (one that would take data simultaneously from multiple TCP connections for example) would still require atomicity on inserting, probably consuming the data from channels as they come, and processing them on a single thread. This is the default behavior for other ledger implementations (like [TigerBeetleDB](https://tigerbeetle.com/)).

## Known Issues

- We do not check if refund/chargebacks/resolve transactions point to the correct client, only that it points to a correct, existing transaction.
- We allow disputes to both withdrawals and deposits, but in most cases only deposits make sense as targets for disputes.


## AI Usage

Keeping with the spirit of the project, no AI code has been used to write this program.
