use super::*;
use crate::index::updater::BlockData;

// CAT-21 😺 - START

// to track transactions with nLockTime values between 0 and this upper limit
pub(crate) const LOCK_TIME_THRESHOLD: u32 = 1000;

// *** also ssee /src/index.ts (where the other tables are defined)
define_table! { LOCKTIME_ORDINAL_TABLE, &TxidValue, LockTimeOrdinalEntryValue }
define_table! { LOCK_TIME_TO_NUMBER, u32, u32 } // u32 for lock_time, u32 for incrementing number

// *** also see /src/entry.rs (where the other entries are defined)
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct LockTimeOrdinalEntry {
    pub transaction_id: Txid, // 1.
    pub lock_time:      u32,  // 2.
    pub number:         u32,  // 3.
    pub block_height:   u32,  // 4.
    pub block_time:     u32,  // 5.
    pub fee:            u64,  // 6.
    pub size:           u64,  // 7.
    pub weight:         u64,  // 8.
    pub value:          u64,  // 9.
    pub sat:            u64,  // 10.
}

pub(crate) type LockTimeOrdinalEntryValue = (
  TxidValue,    // 1. transaction id
  u32,          // 2. lock time
  u32,          // 3. number
  u32,          // 4. block height
  u32,          // 5. block time
  u64,          // 6. fee
  u64,          // 7. size
  u64,          // 8. weight
  u64,          // 9. value
  u64,          // 10. sat
);

impl Entry for LockTimeOrdinalEntry {
  type Value = LockTimeOrdinalEntryValue;

  fn load(
      (
          transaction_id,  // 1.
          lock_time,            // 2.
          number,               // 3.
          block_height,         // 4.
          block_time,           // 5.
          fee,                  // 6.
          size,                 // 7.
          weight,               // 8.
          value,                // 9.
          sat,                  // 10.
      ): LockTimeOrdinalEntryValue,
  ) -> Self {
      Self {
          transaction_id: Txid::load(transaction_id),
          lock_time,
          number,
          block_height,
          block_time,
          fee,
          size,
          weight,
          value,
          sat
      }
  }

  fn store(self) -> Self::Value {
      (
          self.transaction_id.store(),
          self.lock_time,
          self.number,
          self.block_height,
          self.block_time,
          self.fee,
          self.size,
          self.weight,
          self.value,
          self.sat,
      )
  }
}

pub(crate) fn get_next_lock_time_number(
  wtx: &mut WriteTransaction,
  lock_time: u32
) -> Result<u32, Error> {
  let mut lock_time_to_number = wtx.open_table(LOCK_TIME_TO_NUMBER)?;

  // Get the current lock_time_number from the table
  let current_number = lock_time_to_number
      .get(&lock_time)?
      .map(|entry| entry.value()) // Get the current value
      .unwrap_or(0); // If not present, start with 0

  // To keep next_number as 0 or increment it otherwise (counting should start 0, which is cooler)
  let next_number = if current_number == 0 { 0 } else { current_number + 1 };

  // Store the incremented number back into the table
  lock_time_to_number.insert(&lock_time, &next_number)?;

  Ok(next_number)
}

// Function to handle CAT-21 minting logic (and other future LTOs)
pub fn process_mint(
    tx: &Transaction,
    input_sat_ranges: Option<Vec<&[u8]>>,
    input_utxo_entries: &[ParsedUtxoEntry],
    block: &BlockData,
    height: u32,
    index: &Index,
) -> Result<(), Error> {

  let lock_time = tx.lock_time.to_consensus_u32();
  if lock_time > 0 && lock_time <= cat21::LOCK_TIME_THRESHOLD {

      // Get the total input value by iterating through both `tx.input` and `input_utxo_entries`
      let total_input_value = tx.input.iter().enumerate().map(|(index, _txin)| {
        // Get the corresponding ParsedUtxoEntry based on the input index
        let entry = &input_utxo_entries[index];

        // Return the total value of the UTXO entry
        entry.total_value()
      }).sum::<u64>();

      let total_output_value = tx.output.iter().map(|txout| txout.value.to_sat()).sum::<u64>();

      // Calculate the fee
      let fee = total_input_value - total_output_value;

      // Open a new write transaction
      let mut wtx = index.begin_write()?;

      // Get the next number for this lock_time
      let next_lock_time_number = cat21::get_next_lock_time_number(&mut wtx, lock_time)?;

      // According to the first-in, first-out (FIFO) rule,
      // the first sat of the first output corresponds to the first sat of the first input.
      let sat = input_sat_ranges
        .as_ref()
        .and_then(|ranges| ranges.get(0))
        .and_then(|range| range.get(0..11))
        .map(|chunk| SatRange::load(chunk.try_into().unwrap()).0)
        .unwrap_or(0); // Use 0 if no input sat range is found (should never happen)

      let first_output = &tx.output[0];
      let value = first_output.value.to_sat();

      // TODO: figure out what "size" is used by bitcoin core and/or esplora so that we have the same value for all indexers
      // see https://github.com/rust-bitcoin/rust-bitcoin/pull/2076
      // base_size vs total_size
      let size = tx.total_size().try_into().unwrap();
      let weight = tx.weight().into();
      let transaction_id = tx.compute_txid();

      if lock_time == 21 {
        println!("Meow! 😺 {} {} {}", next_lock_time_number, transaction_id, sat);
      }

      let lock_time_ordinal_entry: LockTimeOrdinalEntry = LockTimeOrdinalEntry {
          transaction_id,
          lock_time,
          number: next_lock_time_number,
          block_height: height,
          block_time: block.header.time,
          fee,
          size,
          weight,
          value,
          sat
      };

      // Open the table and insert the new entry
      {
        let mut lock_time_ordinal_table = wtx.open_table(cat21::LOCKTIME_ORDINAL_TABLE)?;
        lock_time_ordinal_table.insert(&lock_time_ordinal_entry.transaction_id.store(), lock_time_ordinal_entry.store())?;
      }

      wtx.commit()?;
  }

  Ok(())

}

// CAT-21 😺 - END
