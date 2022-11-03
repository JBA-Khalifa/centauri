// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Contains subxt generated types for the parachain

#![allow(missing_docs)]

#[cfg(feature = "build-metadata-from-ws")]
include!(concat!(env!("OUT_DIR"), "/parachain.rs"));

#[cfg(not(feature = "build-metadata-from-ws"))]
pub use subxt_generated::parachain::*;