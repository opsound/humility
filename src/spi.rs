/*
 * Copyright 2021 Oxide Computer Company
 */
use crate::hubris::*;
use anyhow::{bail, Result};

/// Looks up which Hubris task is associated with SPI (accepting a peripheral
/// hint to disambiguate).
pub fn spi_task(
    hubris: &HubrisArchive,
    peripheral: Option<u8>,
) -> Result<HubrisTask> {
    let lookup = |peripheral| {
        let spi = format!("spi{}", peripheral);
        let tasks = hubris.lookup_feature(&spi)?;

        match tasks.len() {
            0 => Ok(None),
            1 => Ok(Some(tasks[0])),
            _ => {
                bail!("more than one task has {}", spi);
            }
        }
    };

    let task = if let Some(peripheral) = peripheral {
        match lookup(peripheral)? {
            Some(task) => task,
            None => {
                bail!("SPI peripheral {} not found", peripheral);
            }
        }
    } else {
        let mut found = vec![];

        for peripheral in 0..9 {
            if let Some(task) = lookup(peripheral)? {
                found.push((peripheral, task));
            }
        }

        if found.is_empty() {
            bail!("no SPI peripherals found")
        }

        if found.len() > 1 {
            bail!(
                "SPI peripheral must be specified; valid peripherals: {}",
                found
                    .iter()
                    .map(|v| v.0.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }

        found[0].1
    };
    if task == HubrisTask::Kernel {
        bail!("SPI task cannot be the kernel");
    }
    Ok(task)
}
