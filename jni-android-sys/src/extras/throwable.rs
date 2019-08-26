#![cfg(any(feature = "all", feature = "java-lang-Throwable"))]
use super::java;
use jni_glue::*;
use std::fmt::{self, Debug, Formatter};



impl ThrowableType for java::lang::Throwable {}

impl Debug for java::lang::Throwable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "java::lang::Throwable")?;

        #[cfg(not(any(feature = "all", feature = "java-lang-String")))] {
            writeln!(f, "    getMessage:            N/A (feature = \"java-lang-String\" not defined!)")?;
            writeln!(f, "    getLocalizedMessage:   N/A (feature = \"java-lang-String\" not defined!)")?;
            writeln!(f, "    getStackTrace:         N/A (feature = \"java-lang-String\" not defined!)")?;
        }

        #[cfg(any(feature = "all", feature = "java-lang-String"))] {
            match self.getMessage() {
                Ok(message) =>  writeln!(f, "    getMessage:            {:?}", message)?,
                Err(_) =>       writeln!(f, "    getMessage:            N/A (threw an exception!)")?,
            }

            match self.getLocalizedMessage() {
                Ok(message) =>  writeln!(f, "    getLocalizedMessage:   {:?}", message)?,
                Err(_) =>       writeln!(f, "    getLocalizedMessage:   N/A (threw an exception!)")?,
            }

            #[cfg(not(any(feature = "all", feature = "java-lang-StackTraceElement")))] {
                writeln!(f, "    getStackTrace:         N/A (feature = \"java-lang-StackTraceElement\" not defined!)")?;
            }

            #[cfg(any(feature = "all", feature = "java-lang-StackTraceElement"))] {
                match self.getStackTrace() {
                    Err(_) =>   writeln!(f, "    getStackTrace:         N/A (threw an exception!)")?,
                    Ok(None) => writeln!(f, "    getStackTrace:         N/A (returned null)")?,
                    Ok(Some(stack_trace)) => {
                        writeln!(f, "    getStackTrace:")?;
                        for frame in stack_trace.iter() {
                            match frame {
                                None => writeln!(f, "        N/A (frame was null)")?,
                                Some(frame) => {
                                    let file_line = match (frame.getFileName(), frame.getLineNumber()) {
                                        (Ok(Some(file)), Ok(line))    => format!("{}({}):", file.to_string_lossy(), line),
                                        (Ok(Some(file)), _)           => format!("{}:", file.to_string_lossy()),
                                        (_, _)                  => "N/A (getFileName threw an exception or returned null)".to_owned(),
                                    };

                                    let class_method = match (frame.getClassName(), frame.getMethodName()) {
                                        (Ok(Some(class)), Ok(Some(method))) => format!("{}.{}", class.to_string_lossy(), method.to_string_lossy()),
                                        (Ok(Some(class)), _)                => class.to_string_lossy(),
                                        (_, Ok(Some(method)))               => method.to_string_lossy(),
                                        (_, _)                              => "N/A (getClassName + getMethodName threw exceptions or returned null)".to_owned(),
                                    };

                                    writeln!(f, "        {:120}{}", file_line, class_method)?;
                                },
                            }
                        }
                    },
                }
            }

            // Consider also dumping:
            // API level 1+:
            //      getCause()
            // API level 19+:
            //      getSuppressed()
        }

        Ok(())
    }
}
