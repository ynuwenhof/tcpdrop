use crate::tcp::TcpRow;
use clap::Parser;
use color_eyre::eyre::eyre;
use std::alloc::Layout;
use std::path::PathBuf;
use std::{alloc, fs, mem, slice};
use windows::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, NO_ERROR};
use windows::Win32::NetworkManagement::IpHelper;
use windows::Win32::NetworkManagement::IpHelper::{MIB_TCP6TABLE, MIB_TCPTABLE};

mod tcp;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, env = "TCPDROP_PRETTY")]
    pretty: bool,
    #[arg(long, env = "TCPDROP_NO_V4")]
    no_v4: bool,
    #[arg(long, env = "TCPDROP_NO_V6")]
    no_v6: bool,
    #[arg(short, long, env = "TCPDROP_OUTPUT", value_name = "FILE")]
    output: Option<PathBuf>,
}

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    color_eyre::install()?;

    let table = unsafe { tcp_table(!cli.no_v4, !cli.no_v6)? };

    let output = if cli.pretty {
        serde_json::to_string_pretty(&table)?
    } else {
        serde_json::to_string(&table)?
    };

    if let Some(dest) = cli.output {
        fs::write(dest, output.as_bytes())?;
    } else {
        println!("{output}");
    }

    Ok(())
}

unsafe fn tcp_table(v4: bool, v6: bool) -> color_eyre::Result<Vec<TcpRow>> {
    let mut table = Vec::new();

    if v4 {
        let mut v4_table = &mut MIB_TCPTABLE::default();
        let mut size = mem::size_of::<MIB_TCPTABLE>() as u32;

        let allocation = if IpHelper::GetTcpTable(Some(v4_table), &mut size, true)
            == ERROR_INSUFFICIENT_BUFFER.0
        {
            let layout = Layout::array::<u8>(size as usize)?;
            let ptr = alloc::alloc_zeroed(layout);

            v4_table = (ptr as *mut MIB_TCPTABLE)
                .as_mut()
                .ok_or_else(|| eyre!("failed to allocate v4 memory"))?;

            if IpHelper::GetTcpTable(Some(v4_table), &mut size, true) != NO_ERROR.0 {
                alloc::dealloc(ptr, layout);
                return Err(eyre!("failed to get tcp v4 table"));
            }

            Some((ptr, layout))
        } else {
            None
        };

        let raw_table =
            slice::from_raw_parts(v4_table.table.as_ptr(), v4_table.dwNumEntries as usize);

        for row in raw_table {
            table.push(TcpRow::try_from(*row)?);
        }

        if let Some((ptr, layout)) = allocation {
            alloc::dealloc(ptr, layout);
        }
    }

    if v6 {
        let mut v6_table = &mut MIB_TCP6TABLE::default();
        let mut size = mem::size_of::<MIB_TCP6TABLE>() as u32;

        let allocation =
            if IpHelper::GetTcp6Table(v6_table, &mut size, true) == ERROR_INSUFFICIENT_BUFFER.0 {
                let layout = Layout::array::<u8>(size as usize)?;
                let ptr = alloc::alloc_zeroed(layout);

                v6_table = (ptr as *mut MIB_TCP6TABLE)
                    .as_mut()
                    .ok_or_else(|| eyre!("failed to allocate v6 memory"))?;

                if IpHelper::GetTcp6Table(v6_table, &mut size, true) != NO_ERROR.0 {
                    alloc::dealloc(ptr, layout);
                    return Err(eyre!("failed to get tcp v6 table"));
                }

                Some((ptr, layout))
            } else {
                None
            };

        let raw_table =
            slice::from_raw_parts(v6_table.table.as_ptr(), v6_table.dwNumEntries as usize);

        for row in raw_table {
            table.push(TcpRow::try_from(*row)?);
        }

        if let Some((ptr, layout)) = allocation {
            alloc::dealloc(ptr, layout);
        }
    }

    Ok(table)
}
