// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use dusk_bytes::{Error as BytesError, Serializable};

use super::{Error, Result};

pub struct Seed([u8; 64]);

impl Seed {
    pub fn from_reader<T: AsRef<[u8]>>(buf: T) -> Result<Self> {
        if let Ok(x) = buf.as_ref().try_into() {
            Ok(Self(x))
        } else {
            Err(Error::WalletFileCorrupted)
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Serializable<64> for Seed {
    type Error = BytesError;

    fn from_bytes(buff: &[u8; Seed::SIZE]) -> core::result::Result<Self, Self::Error> {
        Ok(Self(*buff))
    }

    fn to_bytes(&self) -> [u8; Seed::SIZE] {
        self.0
    }
}
