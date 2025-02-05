// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::process::{Command, Stdio};

use anyhow::{bail, Result};
use serde_json::{Map, Value};

pub(crate) fn workspace_root() -> Result<String> {
    let mut cmd = Command::new("cargo");
    cmd.args(["metadata", "--format-version=1"])
        .stdin(Stdio::null())
        .stderr(Stdio::inherit());

    let output = cmd.output()?;
    if !output.status.success() {
        bail!("failed to query cargo metadata");
    }
    let metadata: Map<String, Value> = serde_json::from_slice(&output.stdout)?;

    if let Some(Value::String(root)) = metadata.get("workspace_root") {
        Ok(root.clone())
    } else {
        bail!("could not location workspace root")
    }
}
