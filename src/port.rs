use std::net::{TcpListener, TcpStream};

use anyhow::{Context, Result, bail};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortArg {
    Auto,
    Exact(u16),
}

impl PortArg {
    pub fn resolve(self) -> Result<u16> {
        match self {
            Self::Auto => free_port(),
            Self::Exact(port) => {
                if port_in_use(port) {
                    bail!("port {port} is already in use; pass --port auto or choose another port");
                }
                Ok(port)
            }
        }
    }
}

pub fn parse_port_arg(value: &str) -> Result<PortArg, String> {
    if value.eq_ignore_ascii_case("auto") {
        return Ok(PortArg::Auto);
    }
    match value.parse::<u16>() {
        Ok(0) => Err("port 0 is invalid; pass --port auto to pick a free port".into()),
        Ok(port) => Ok(PortArg::Exact(port)),
        Err(_) => Err(format!(
            "invalid port `{value}`; expected a number 1-65535 or `auto`"
        )),
    }
}

pub fn free_port() -> Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0").context("failed to allocate a local port")?;
    Ok(listener.local_addr()?.port())
}

fn port_in_use(port: u16) -> bool {
    TcpStream::connect(("127.0.0.1", port)).is_ok()
}

#[cfg(test)]
fn bind_ephemeral(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_auto_port() {
        assert_eq!(parse_port_arg("auto").unwrap(), PortArg::Auto);
        assert_eq!(parse_port_arg("AUTO").unwrap(), PortArg::Auto);
    }

    #[test]
    fn parses_explicit_port() {
        assert_eq!(parse_port_arg("9001").unwrap(), PortArg::Exact(9001));
    }

    #[test]
    fn rejects_invalid_port() {
        let err = parse_port_arg("nope").unwrap_err();
        assert!(err.contains("invalid port `nope`"));
        assert!(
            parse_port_arg("0")
                .unwrap_err()
                .contains("port 0 is invalid")
        );
    }

    #[test]
    fn exact_occupied_port_errors() {
        let held = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = held.local_addr().unwrap().port();
        let err = PortArg::Exact(port).resolve().unwrap_err().to_string();
        assert!(
            err.contains("already in use") && err.contains("--port auto"),
            "{err}"
        );
    }

    #[test]
    fn auto_port_resolves_to_a_free_port() {
        for _ in 0..8 {
            if bind_ephemeral(PortArg::Auto.resolve().unwrap()) {
                return;
            }
        }
        panic!("auto port did not yield a bindable port");
    }
}
