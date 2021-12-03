# Pallet implementation for Decentralised Identifiers

## Tags

### Tag 0.0.8 Features
* Signing a DID from the Issuer's Private Key
* The Issuer can assign other trusted Signatures as an array
* This comes in the form of a Signature Array
* ```rust
  #[derive(Clone, Decode, Encode, Eq, PartialEq, Debug)] 
  pub struct DIDSignature  {
      pub public_key: Vec<u8>,
      pub proof: Signature,
      pub active: bool,
      pub created_time_stamp: u64,
      pub updated_timestamp: u64,
  }
  ```
