// Copyright 2017 ore Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/*
 Crate:         ore
 File:          /request.rs
 Module:        ::request
 Visibility:    public
 */

//! Provides the interfaces for request data from an Ore repository.

use serde::Deserialize;
use serde::de;
use serde_json;
use std::error;
use std::fmt;
use std::result;

/// The possible errors that can be returned by request methods.
#[derive(Debug)]
pub enum Error {
    /// Invalid Json data.
    Json(serde_json::Error),
}

impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Json(ref why) => fmt::Display::fmt(why, f),
        }
    }
}

impl error::Error for Error {

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Json(ref why) => Some(why),
        }
    }

    fn description(&self) -> &str {
        match *self {
            Error::Json(ref why) => why.description(),
        }
    }
}

#[doc(hidden)]
impl From<serde_json::Error> for Error {

    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

/// The type returned by request methods.
pub type Result<T> = result::Result<T, Error>;

/// The base Request interface.
pub trait RawRequest
{
    /// Request raw bytes from the `uri` source.
    fn request(&self, threads: usize, uri: &str) -> Result<&[u8]>;
}

/// The Request interface for when the returned data is known to be Json bytes.
pub trait JsonRequest<'a, D>: RawRequest
    where D: Deserialize<'a>
{
    /// Request Json bytes and convert into `D` type.
    fn request(&'a self, threads: usize, uri: &str) -> Result<D> {
        serde_json::from_slice(RawRequest::request(self, threads, uri)?)
            .map_err(|why| Error::from(why))
    }
}
