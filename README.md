# Vaxchain Program Library

## Token

Name - Vaxchain
Symbol - VAX
Decimals - 9

Local address (anish) - Bm42D4TxMZ7MgsQNrYcmcQCzHtg37BExDH8SyepyLERB
Devnet address - to be deployed

## Program Anatomy

### Accounts

- User
  - pubkey - PublicKey
  - createdAt - 164
  - updatedAt - 164
  - role - Enum(Manufacturer, Distributor, Doctor)
- Batch
  - pubkey - PublicKey
  - manufacturedAt - i64
  - expiresAt - i64
  - manufacturer - PublicKey
  - distributor - PublicKey
  - quantity - u8
  - tempMin - u8 (kelvins)
  - tempMax - u8 (kelvins)
  - costPerPiece - u16
  - status - Enum(Manufactured, StoredByDistributor, ReceivedByDoctor)
  - tempDefect - Boolean
- Vaccine
  - pubkey - PublicKey
  - batch - PublicKey
  - used - Boolean
  - usedBy - Option<PublicKey>
- Log
  - batch - PublicKey
  - timestamp - i64
- TempLog
  - batch - PublicKey
  - timestamp - i64
  - temp - u8 (kelvins)

### Instructions

- CreateUser
- CreateBatch (will also create vaccines)
- ShipFromManufacturer
- ReceiveByDistributor (will stake here)
- ShipFromDistributor
- ReceivedByDoctor (doctor verifies and half stake is refunded)
- UseVaccine (once all vaccines are used, the other half of stake is refunded)
- CreateTempLog (slash stake if temperature exceeds threshold)

## Clockwork part

Check for temp logs and if newest log is older than 5 minutes, slash stake
