extern crate alloc;

/// Common Stdout implementation
///
/// This struct provides a Write implementation that calls the platform-specific putc function.
/// Each platform must provide a `putc(ch: u8)` function.
pub struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Call platform-specific putc function
        unsafe extern "Rust" {
            fn putc(ch: u8);
        }

        for byte in s.bytes() {
            unsafe { putc(byte) };
        }
        Ok(())
    }
}

/// Get a Stdout handle
pub fn stdout() -> Stdout {
    Stdout
}

/// Common Stdin implementation
///
/// This struct provides Read-like functionality using platform-specific getc/try_getc functions.
/// Each platform must provide:
/// - `getc() -> u8` - Blocking character read
/// - `try_getc() -> Option<u8>` - Non-blocking character read
pub struct Stdin;

pub enum Error {
    WTF,
}

impl Stdin {
    /// Read a single character (blocking)
    ///
    /// This function blocks until a character is available from stdin.
    ///
    /// # Returns
    /// * The received character byte
    pub fn read(&self) -> u8 {
        unsafe extern "Rust" {
            fn getc() -> u8;
        }
        unsafe { getc() }
    }

    /// Try to read a single character (non-blocking)
    ///
    /// This function returns immediately with None if no character is available.
    ///
    /// # Returns
    /// * `Some(ch)` - Character byte if available
    /// * `None` - No character available
    pub fn try_getc(&self) -> Option<u8> {
        unsafe extern "Rust" {
            fn try_getc() -> Option<u8>;
        }
        unsafe { try_getc() }
    }

    /// Read a line into a string (blocking)
    ///
    /// This function reads characters until a newline ('\n') is encountered.
    /// The newline is not included in the returned string.
    ///
    /// # Arguments
    /// * `buffer` - String to append the line to
    ///
    /// # Returns
    /// * Number of bytes read (excluding newline)
    pub fn read_line(&self, buffer: &mut alloc::string::String) -> Result<usize, Error> {
        let mut count = 0;
        loop {
            let ch = self.read();
            count += 1;

            // Handle different line endings
            if ch == b'\n' {
                break;
            } else if ch == b'\r' {
                // Check for \r\n
                if let Some(next) = self.try_getc() {
                    if next != b'\n' {
                        // Just \r, treat as newline but we consumed an extra char
                        // Push it back conceptually (just break)
                        break;
                    }
                }
                break;
            }

            buffer.push(ch as char);
        }

        Ok(count)
    }
}

/// Get a Stdin handle
pub fn stdin() -> Stdin {
    Stdin
}
