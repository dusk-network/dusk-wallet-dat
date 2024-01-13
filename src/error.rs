// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use block_modes::{BlockModeError, InvalidKeyIvLength};

pub type Result<T> = core::result::Result<T, WalletDataFileError>;

#[derive(Debug)]
pub enum WalletDataFileError {
    WalletFileCorrupted,
    UnknownFileVersion(u8, u8),
    InvalidKeyLength(InvalidKeyIvLength),
    WrongPassword(BlockModeError),
    NoLastPosFound,
}

impl From<InvalidKeyIvLength> for WalletDataFileError {
    fn from(e: InvalidKeyIvLength) -> Self {
        Self::InvalidKeyLength(e)
    }
}

impl From<BlockModeError> for WalletDataFileError {
    fn from(e: BlockModeError) -> Self {
        Self::WrongPassword(e)
    }
}
