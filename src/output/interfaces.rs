use std::fmt::{self, Write};

use heck::AsSnakeCase;

use super::{slugify, CodeWriter, Formatter, InterfaceMap};

impl CodeWriter for InterfaceMap {
    fn write_rs(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#![allow(non_upper_case_globals, unused)]\n")?;

        fmt.block("pub mod cs2_dumper", false, |fmt| {
            fmt.block("pub mod interfaces", false, |fmt| {
                for (module_name, ifaces) in self {
                    writeln!(fmt, "// Module: {}", module_name)?;

                    fmt.block(
                        &format!("pub mod {}", AsSnakeCase(slugify(module_name))),
                        false,
                        |fmt| {
                            for (name, value) in ifaces {
                                writeln!(fmt, "pub const {}: usize = {:#X};", name, value)?;
                            }

                            Ok(())
                        },
                    )?;
                }

                Ok(())
            })
        })
    }
}
