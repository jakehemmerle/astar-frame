// Copyright 2019-2022 PureStake Inc.
// Copyright 2022      Stake Technologies
// This file is part of Utils package, originally developed by Purestake Inc.
// Utils package used in Astar Network in terms of GPLv3.
//
// Utils is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Utils is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Utils.  If not, see <http://www.gnu.org/licenses/>.

//! Encoding of XCM types for solidity

use crate::{revert, Bytes, EvmData, EvmDataReader, EvmDataWriter, EvmResult};

use frame_support::ensure;
use sp_std::vec::Vec;
use xcm::latest::{Junction, Junctions, MultiLocation, NetworkId};

// Function to convert network id to bytes
// We don't implement EVMData here as these bytes will be appended only
// to certain Junction variants
// Each NetworkId variant is represented as bytes
// The first byte represents the enum variant to be used
// The rest of the bytes (if any), represent the additional data that such enum variant requires
// In this case, only Named requires additional non-bounded data.
// In such a case, since NetworkIds will be appended at the end, we will read the buffer until the
// end to recover the name
pub(crate) fn network_id_to_bytes(network_id: NetworkId) -> Vec<u8> {
    let mut encoded: Vec<u8> = Vec::new();
    match network_id {
        NetworkId::Any => {
            encoded.push(0u8);
            encoded
        }
        NetworkId::Named(mut name) => {
            encoded.push(1u8);
            encoded.append(&mut name);
            encoded
        }
        NetworkId::Polkadot => {
            encoded.push(2u8);
            encoded
        }
        NetworkId::Kusama => {
            encoded.push(3u8);
            encoded
        }
    }
}

// Function to convert bytes to networkId
pub(crate) fn network_id_from_bytes(encoded_bytes: Vec<u8>) -> EvmResult<NetworkId> {
    ensure!(
        !encoded_bytes.is_empty(),
        revert("Junctions cannot be empty")
    );
    let mut encoded_network_id = EvmDataReader::new(&encoded_bytes);

    let network_selector = encoded_network_id.read_raw_bytes(1)?;

    match network_selector[0] {
        0 => Ok(NetworkId::Any),
        1 => Ok(NetworkId::Named(
            encoded_network_id.read_till_end()?.to_vec(),
        )),
        2 => Ok(NetworkId::Polkadot),
        3 => Ok(NetworkId::Kusama),
        _ => Err(revert("Non-valid Network Id")),
    }
}

impl EvmData for Junction {
    fn read(reader: &mut EvmDataReader) -> EvmResult<Self> {
        let junction = reader.read::<Bytes>()?;
        let junction_bytes = junction.as_bytes();

        ensure!(
            !junction_bytes.is_empty(),
            revert("Junctions cannot be empty")
        );

        // For simplicity we use an EvmReader here
        let mut encoded_junction = EvmDataReader::new(junction_bytes);

        // We take the first byte
        let enum_selector = encoded_junction.read_raw_bytes(1)?;

        // The firs byte selects the enum variant
        match enum_selector[0] {
            0 => {
                // In the case of Junction::Parachain, we need 4 additional bytes
                let mut data: [u8; 4] = Default::default();
                data.copy_from_slice(encoded_junction.read_raw_bytes(4)?);
                let para_id = u32::from_be_bytes(data);
                Ok(Junction::Parachain(para_id))
            }
            1 => {
                // In the case of Junction::AccountId32, we need 32 additional bytes plus NetworkId
                let mut account: [u8; 32] = Default::default();
                account.copy_from_slice(encoded_junction.read_raw_bytes(32)?);

                let network = encoded_junction.read_till_end()?.to_vec();
                Ok(Junction::AccountId32 {
                    network: network_id_from_bytes(network)?,
                    id: account,
                })
            }
            2 => {
                // In the case of Junction::AccountIndex64, we need 8 additional bytes plus NetworkId
                let mut index: [u8; 8] = Default::default();
                index.copy_from_slice(encoded_junction.read_raw_bytes(8)?);
                // Now we read the network
                let network = encoded_junction.read_till_end()?.to_vec();
                Ok(Junction::AccountIndex64 {
                    network: network_id_from_bytes(network)?,
                    index: u64::from_be_bytes(index),
                })
            }
            3 => {
                // In the case of Junction::AccountKey20, we need 20 additional bytes plus NetworkId
                let mut account: [u8; 20] = Default::default();
                account.copy_from_slice(encoded_junction.read_raw_bytes(20)?);

                let network = encoded_junction.read_till_end()?.to_vec();
                Ok(Junction::AccountKey20 {
                    network: network_id_from_bytes(network)?,
                    key: account,
                })
            }
            4 => Ok(Junction::PalletInstance(
                encoded_junction.read_raw_bytes(1)?[0],
            )),
            5 => {
                // In the case of Junction::GeneralIndex, we need 16 additional bytes
                let mut general_index: [u8; 16] = Default::default();
                general_index.copy_from_slice(encoded_junction.read_raw_bytes(16)?);
                Ok(Junction::GeneralIndex(u128::from_be_bytes(general_index)))
            }
            6 => Ok(Junction::GeneralKey(
                encoded_junction.read_till_end()?.to_vec(),
            )),
            7 => Ok(Junction::OnlyChild),
            _ => Err(revert("No selector for this")),
        }
    }

    fn write(writer: &mut EvmDataWriter, value: Self) {
        let mut encoded: Vec<u8> = Vec::new();
        let encoded_bytes: Bytes = match value {
            Junction::Parachain(para_id) => {
                encoded.push(0u8);
                encoded.append(&mut para_id.to_be_bytes().to_vec());
                encoded.as_slice().into()
            }
            Junction::AccountId32 { network, id } => {
                encoded.push(1u8);
                encoded.append(&mut id.to_vec());
                encoded.append(&mut network_id_to_bytes(network));
                encoded.as_slice().into()
            }
            Junction::AccountIndex64 { network, index } => {
                encoded.push(2u8);
                encoded.append(&mut index.to_be_bytes().to_vec());
                encoded.append(&mut network_id_to_bytes(network));
                encoded.as_slice().into()
            }
            Junction::AccountKey20 { network, key } => {
                encoded.push(3u8);
                encoded.append(&mut key.to_vec());
                encoded.append(&mut network_id_to_bytes(network));
                encoded.as_slice().into()
            }
            Junction::PalletInstance(intance) => {
                encoded.push(4u8);
                encoded.append(&mut intance.to_be_bytes().to_vec());
                encoded.as_slice().into()
            }
            Junction::GeneralIndex(id) => {
                encoded.push(5u8);
                encoded.append(&mut id.to_be_bytes().to_vec());
                encoded.as_slice().into()
            }
            Junction::GeneralKey(mut key) => {
                encoded.push(6u8);
                encoded.append(&mut key);
                encoded.as_slice().into()
            }
            Junction::OnlyChild => {
                encoded.push(7u8);
                encoded.as_slice().into()
            }
            // TODO: The only missing item here is Junciton::Plurality. This is a complex encoded
            // type that we need to evaluate how to support
            _ => unreachable!("Junction::Plurality not supported yet"),
        };
        EvmData::write(writer, encoded_bytes);
    }

    fn has_static_size() -> bool {
        false
    }
}

impl EvmData for Junctions {
    fn read(reader: &mut EvmDataReader) -> EvmResult<Self> {
        let junctions_bytes: Vec<Junction> = reader.read()?;
        let mut junctions = Junctions::Here;
        for item in junctions_bytes {
            junctions
                .push(item)
                .map_err(|_| revert("overflow when reading junctions"))?;
        }

        Ok(junctions)
    }

    fn write(writer: &mut EvmDataWriter, value: Self) {
        let encoded: Vec<Junction> = value.iter().cloned().collect();
        EvmData::write(writer, encoded);
    }

    fn has_static_size() -> bool {
        false
    }
}

impl EvmData for MultiLocation {
    fn read(reader: &mut EvmDataReader) -> EvmResult<Self> {
        let (parents, interior) = reader.read()?;

        Ok(MultiLocation { parents, interior })
    }

    fn write(writer: &mut EvmDataWriter, value: Self) {
        EvmData::write(writer, (value.parents, value.interior));
    }

    fn has_static_size() -> bool {
        <(u8, Junctions)>::has_static_size()
    }
}
