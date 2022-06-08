# TXOR
(Transact-or) A simple (toy) transaction processing CLI.

*The specification for this project should be available to the reader to correctly use this software.*

The program accepts a single input CSV file in the specified format.
And outputs the results of the transactions to the standard output in a CSV format as well.

## About Correctness
To ensure correctness, a number of fixture like tests have been written.
These can be viewed in the `test_data` directory.

Each test case is described by a TOML file of the following form:
```toml
name = "<test name>" # just for readability purposes
description = "<test name>" # just for readability purposes

input_csv = """<input csv header>
<input csv rows ...>
"""

[output]
transactions_in_dispute = [<disputed transaction id one>, ...]

[[output.clients]]
id = <client id>
available = <available money, in subdivisions>
held = <held money, in subdivisions>
locked = <locked state>

.
.
.
```
I have decided to go with this approach because the problem is so nicely functional in nature, we can just write predictable end-to-end tests and expect them to work.

The test cases also serve as an outline for the specification of the actual capabilities of this processor.

## About Efficiency
The solution is using an async stream on the top level and an async csv reader to accommodate that.
So that it can be used as is in a larger application.
The processing of the transactions are still happening one at a time.
The reasoning behind this choice is simple: It lets the code stay less complex, while performance should already be good enough for the future.
The blocking transaction handling operations don't use any IO, and don't take a lot of time.

## Running tests
I recommend using the just command runner utility available at: https://github.com/casey/just

To run the tests with just:
```bash 
just run-test
```

To run the tests without just:
```bash 
cargo test -- --nocapture
```

## Code structure
The cli tool is built from a few closely coupled pieces, with tight cohesion inside each part and a few shared modules.
A list of all these modules and what are they responsible for:
- CLI
  - Parses arguments
  - Sets up logging
  - Runs the processor
  - Writes results to the standard output
- Parser
  - Defines the `TransactionSource` trait, which can abstracts away the streaming of transactions
  - Implements a parser that creates said source from an async readable
- Processor
  - Implements the transaction processor
  - The processor can take a `TransactionSource` consume it
  - A processor can be turned into a processing output that tallies the client data
- Errors (Shared code)
  - Defines all the error enums the rest of the program uses
  - Everything uses error enumeration for better usability in the future
- Models (Shared code)
  - Defines a few shared models (like `Transaction`) for use everywhere else