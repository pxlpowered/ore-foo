// Copyright 2017 ore Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/*
 Crate:         ore
 File:          /lib.rs
 Module:        ::
 Visibility:    public
 */

//! A library interacting with the [Sponge Ore] web API.
//!
//! Documentation on the web API can be API can be found [here].
//!
//! [Sponge Ore]: https://ore.spongepowered.org
//! [here]: https://docs.spongepowered.org/stable/en/ore/api.html

#![doc(html_root_url = "https://docs.rs/ore/0.1.0")]
#![deny(missing_docs)]
#![deny(warnings)]
#![deny(missing_debug_implementations)]

extern crate serde;
extern crate serde_json;

#[cfg(feature = "hyper_tls_backend")]
pub mod request;
