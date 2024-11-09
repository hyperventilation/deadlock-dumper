use std::fmt::{self, Write};

use heck::AsSnakeCase;

use super::{slugify, CodeWriter, Formatter, OffsetMap};

impl CodeWriter for OffsetMap {
    fn write_rs(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#![allow(non_upper_case_globals, unused)]\n")?;

        fmt.block("pub mod cs2_dumper", false, |fmt| {
            fmt.block("pub mod offsets", false, |fmt| {
                for (module_name, offsets) in self {
                    writeln!(fmt, "// Module: {}", module_name)?;

                    fmt.block(
                        &format!("pub mod {}", AsSnakeCase(slugify(module_name))),
                        false,
                        |fmt| {
                            for (name, value) in offsets {
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
