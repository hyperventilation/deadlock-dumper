use std::fmt::{self, Write};

use super::{ButtonMap, CodeWriter, Formatter};

impl CodeWriter for ButtonMap {
    fn write_rs(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#![allow(non_upper_case_globals, unused)]\n")?;

        fmt.block("pub mod cs2_dumper", false, |fmt| {
            writeln!(fmt, "// Module: client.dll")?;

            fmt.block("pub mod buttons", false, |fmt| {
                for (name, value) in self {
                    let mut name = name.clone();

                    if name == "use" {
                        name = format!("r#{}", name);
                    }

                    writeln!(fmt, "pub const {}: usize = {:#X};", name, value)?;
                }

                Ok(())
            })
        })
    }
}
