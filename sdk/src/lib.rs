// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[allow(clippy::clone_on_copy)]
#[allow(clippy::len_zero)]
#[allow(clippy::vec_init_then_push)]
mod generated_sdk;

#[cfg(feature = "clap")]
mod clap_feature;

use std::time::Duration;

pub use generated_sdk::*;
use reqwest::{header::HeaderValue, ClientBuilder};

impl Client {
    /// Create a client which adds a specified authentication token to all
    /// requests. If you have logged into the API with the oxide CLI, you can
    /// find an authentication token in your `$HOME/.config/oxide/hosts.toml`
    /// file.
    pub fn new_with_auth_token(baseurl: &str, token: &str) -> Self {
        let mut client_builder = ClientBuilder::new().connect_timeout(Duration::from_secs(15));

        let mut bearer = HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap();
        bearer.set_sensitive(true);
        client_builder = client_builder.default_headers(
            [(http::header::AUTHORIZATION, bearer)]
                .into_iter()
                .collect(),
        );

        let client = client_builder.build().unwrap();

        Client::new_with_client(baseurl, client)
    }
}
