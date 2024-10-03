# APT Blockchain Project Binaries Documentation

## Overview

This document provides an overview of the three main binaries used in the APT Blockchain project: `aptbc_client`, `aptbc_ledger`, and `aptbc_ns`. Each of these binaries plays a crucial role in managing transactions and maintaining the blockchain.

## Binaries

### 1. `aptbc_client`
- **Description**: The client binary used to raise a request for a transaction.
- **Usage**: 
  - The `aptbc_client` sends transaction requests to the network server (`aptbc_ns`).
  - It is responsible for specifying the transaction details, which are then forwarded and processed by other components.
- **Command**:
  ```sh
  ./aptbc_client --transaction <transaction_details>



./aptbc_client --transaction '{"sender": "Alice", "receiver": "Bob", "amount": 100}'
./aptbc_ledger --port <port_number>
./aptbc_ns --port <port_number> --ledgers <ledger_addresses>
