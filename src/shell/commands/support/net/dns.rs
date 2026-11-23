//! Minimal UDP DNS client for `ntc dns` (RFC 1035), no extra crate.
//!
//! Supports the record types ntc prints: `A`, `AAAA`, `MX`, `NS`, `CNAME`,
//! `TXT`, `PTR` and `SOA`. Servers come from `/etc/resolv.conf` (Unix) or the
//! adapter list (Windows), with public resolvers as a last resort.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};
use std::sync::OnceLock;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Record types accepted by `dns -t`, in help order.
pub(crate) const SUPPORTED_TYPES: &str = "A, AAAA, MX, NS, CNAME, TXT, PTR, SOA, ALL";

/// Total budget for one lookup, shared by every server that was queried.
/// The query goes out to all of them at once, so a dead resolver costs
/// nothing unless it is the only one, and the worst case is this deadline
/// rather than (servers x retries x timeout).
const QUERY_TIMEOUT: Duration = Duration::from_millis(1500);
/// How long one socket blocks before the others are polled again.
const POLL_SLICE: Duration = Duration::from_millis(50);
/// After an authoritative `rcode` answer, how long another resolver still gets
/// to answer `NOERROR` (an internal name that a public resolver calls
/// NXDOMAIN). Without this an NXDOMAIN would wait out the whole deadline.
const RCODE_GRACE: Duration = Duration::from_millis(300);
const PUBLIC_SERVERS: [&str; 2] = ["1.1.1.1:53", "8.8.8.8:53"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum RRType {
    A,
    Ns,
    Cname,
    Soa,
    Ptr,
    Mx,
    Txt,
    Aaaa,
}

impl RRType {
    pub(crate) fn code(self) -> u16 {
        match self {
            RRType::A => 1,
            RRType::Ns => 2,
            RRType::Cname => 5,
            RRType::Soa => 6,
            RRType::Ptr => 12,
            RRType::Mx => 15,
            RRType::Txt => 16,
            RRType::Aaaa => 28,
        }
    }

    pub(crate) fn from_code(code: u16) -> Option<Self> {
        match code {
            1 => Some(RRType::A),
            2 => Some(RRType::Ns),
            5 => Some(RRType::Cname),
            6 => Some(RRType::Soa),
            12 => Some(RRType::Ptr),
            15 => Some(RRType::Mx),
            16 => Some(RRType::Txt),
            28 => Some(RRType::Aaaa),
            _ => None,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            RRType::A => "A",
            RRType::Ns => "NS",
            RRType::Cname => "CNAME",
            RRType::Soa => "SOA",
            RRType::Ptr => "PTR",
            RRType::Mx => "MX",
            RRType::Txt => "TXT",
            RRType::Aaaa => "AAAA",
        }
    }

    /// `A`/`AAAA`/`ALL` are handled by the caller, so they return `None`.
    pub(crate) fn from_label(label: &str) -> Option<Self> {
        match label.trim().to_uppercase().as_str() {
            "A" => Some(RRType::A),
            "AAAA" => Some(RRType::Aaaa),
            "NS" => Some(RRType::Ns),
            "CNAME" => Some(RRType::Cname),
            "SOA" => Some(RRType::Soa),
            "PTR" => Some(RRType::Ptr),
            "MX" => Some(RRType::Mx),
            "TXT" => Some(RRType::Txt),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RData {
    Ip(IpAddr),
    Mx(u16, String),
    Name(String),
    Txt(String),
    Soa {
        mname: String,
        rname: String,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32,
    },
}

#[derive(Clone, Debug)]
pub(crate) struct Record {
    pub(crate) rtype: RRType,
    /// Parsed for completeness; the CLI output does not show it.
    #[allow(dead_code)]
    pub(crate) ttl: u32,
    pub(crate) data: RData,
}

impl Record {
    /// One-line value for the CLI, dig-style but without a trailing dot.
    pub(crate) fn render(&self) -> String {
        match &self.data {
            RData::Ip(ip) => ip.to_string(),
            RData::Mx(pref, exchange) => format!("{} {}", pref, exchange),
            RData::Name(name) => name.clone(),
            RData::Txt(text) => format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\"")),
            RData::Soa {
                mname,
                rname,
                serial,
                refresh,
                retry,
                expire,
                minimum,
            } => format!(
                "{} {} {} {} {} {} {}",
                mname, rname, serial, refresh, retry, expire, minimum
            ),
        }
    }
}

#[derive(Debug)]
pub(crate) enum DnsError {
    /// Server answered with a non-zero rcode (3 = NXDOMAIN).
    Rcode(u8),
    Timeout,
    Malformed,
    /// Socket setup or send failed, so no answer was received at all.
    Network(String),
}

pub(crate) fn rcode_name(code: u8) -> &'static str {
    match code {
        0 => "NOERROR",
        1 => "FORMERR",
        2 => "SERVFAIL",
        3 => "NXDOMAIN",
        4 => "NOTIMP",
        5 => "REFUSED",
        _ => "ERROR",
    }
}

/// `a.b.c.d` -> `d.c.b.a.in-addr.arpa`, IPv6 -> reversed nibbles `ip6.arpa`.
/// Returns `None` when the input is not an IP literal.
pub(crate) fn reverse_name(host: &str) -> Option<String> {
    match host.trim().parse::<IpAddr>() {
        Ok(IpAddr::V4(v4)) => {
            let o = v4.octets();
            Some(format!("{}.{}.{}.{}.in-addr.arpa", o[3], o[2], o[1], o[0]))
        }
        Ok(IpAddr::V6(v6)) => {
            let mut out = String::with_capacity(74);
            for byte in v6.octets().iter().rev() {
                out.push_str(&format!("{:x}.{:x}.", byte & 0x0f, byte >> 4));
            }
            out.push_str("ip6.arpa");
            Some(out)
        }
        Err(_) => None,
    }
}

/// Query `name` for `rtype` against every known resolver at once and take the
/// first usable answer. A resolver that does not reply can no longer hold the
/// lookup back, so one dead (or link-local IPv6) entry costs nothing.
pub(crate) fn resolve(name: &str, rtype: RRType) -> Result<Vec<Record>, DnsError> {
    let name = name.trim().trim_end_matches('.');
    if name.is_empty() {
        return Err(DnsError::Malformed);
    }
    query_servers(resolvers(), name, rtype)
}

/// Resolver list, read once per process: enumerating adapters on Windows is
/// far more expensive than the answer itself and the list rarely changes.
fn resolvers() -> &'static [SocketAddr] {
    static SERVERS: OnceLock<Vec<SocketAddr>> = OnceLock::new();
    SERVERS.get_or_init(|| {
        let mut servers = system_servers();
        for candidate in PUBLIC_SERVERS {
            if let Ok(addr) = candidate.parse::<SocketAddr>() {
                if !servers.contains(&addr) {
                    servers.push(addr);
                }
            }
        }
        servers
    })
}

/// Can this address be queried at all? A link-local IPv6 resolver needs a
/// scope id that `SocketAddr` does not carry here, so the packet would always
/// time out; unspecified and multicast addresses are equally useless.
fn is_usable_server(addr: &SocketAddr) -> bool {
    match addr.ip() {
        IpAddr::V4(v4) => !v4.is_unspecified() && !v4.is_multicast(),
        IpAddr::V6(v6) => {
            !v6.is_unspecified()
                && !v6.is_multicast()
                // fe80::/10 link-local
                && !(v6.segments()[0] & 0xffc0 == 0xfe80)
        }
    }
}

#[cfg(not(windows))]
fn system_servers() -> Vec<SocketAddr> {
    let mut servers = Vec::new();
    if let Ok(text) = std::fs::read_to_string("/etc/resolv.conf") {
        for line in text.lines() {
            let line = line.trim();
            let Some(rest) = line.strip_prefix("nameserver") else {
                continue;
            };
            if let Ok(ip) = rest.trim().parse::<IpAddr>() {
                let addr = SocketAddr::new(ip, 53);
                if is_usable_server(&addr) && !servers.contains(&addr) {
                    servers.push(addr);
                }
            }
        }
    }
    servers
}

#[cfg(windows)]
fn system_servers() -> Vec<SocketAddr> {
    use windows_sys::Win32::Foundation::{ERROR_BUFFER_OVERFLOW, ERROR_SUCCESS};
    use windows_sys::Win32::NetworkManagement::IpHelper::{
        GetAdaptersAddresses, GAA_FLAG_SKIP_ANYCAST, GAA_FLAG_SKIP_MULTICAST,
        GAA_FLAG_SKIP_UNICAST, IP_ADAPTER_ADDRESSES_LH,
    };
    use windows_sys::Win32::Networking::WinSock::{AF_INET, AF_INET6, AF_UNSPEC};

    let mut servers: Vec<SocketAddr> = Vec::new();
    let mut size: u32 = 16 * 1024;
    let mut buf: Vec<u8> = Vec::new();
    loop {
        buf.resize(size as usize, 0);
        let rc = unsafe {
            GetAdaptersAddresses(
                AF_UNSPEC as u32,
                GAA_FLAG_SKIP_ANYCAST | GAA_FLAG_SKIP_MULTICAST | GAA_FLAG_SKIP_UNICAST,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH,
                &mut size,
            )
        };
        if rc == ERROR_BUFFER_OVERFLOW {
            continue;
        }
        if rc != ERROR_SUCCESS {
            return servers;
        }
        break;
    }

    let mut adapter = buf.as_ptr() as *const IP_ADAPTER_ADDRESSES_LH;
    while !adapter.is_null() {
        // SAFETY: the buffer was filled by GetAdaptersAddresses and every
        // pointer we follow is owned by that buffer, which outlives the walk.
        let node = unsafe { &*adapter };
        let mut dns = node.FirstDnsServerAddress;
        while !dns.is_null() {
            let entry = unsafe { &*dns };
            let sa = entry.Address.lpSockaddr;
            let len = entry.Address.iSockaddrLength.max(0) as usize;
            if !sa.is_null() && len >= 8 {
                // Plain byte view: sockaddr_in and sockaddr_in6 share the
                // family/port layout, so no union field access is needed.
                let raw = unsafe { std::slice::from_raw_parts(sa as *const u8, len) };
                let family = u16::from_ne_bytes([raw[0], raw[1]]);
                let port = match u16::from_be_bytes([raw[2], raw[3]]) {
                    0 => 53,
                    p => p,
                };
                let addr = if family == AF_INET as u16 && len >= 8 {
                    Some(SocketAddr::new(
                        IpAddr::V4(Ipv4Addr::new(raw[4], raw[5], raw[6], raw[7])),
                        port,
                    ))
                } else if family == AF_INET6 as u16 && len >= 24 {
                    let mut octets = [0u8; 16];
                    octets.copy_from_slice(&raw[8..24]);
                    Some(SocketAddr::new(IpAddr::V6(Ipv6Addr::from(octets)), port))
                } else {
                    None
                };
                if let Some(addr) = addr {
                    if is_usable_server(&addr) && !servers.contains(&addr) {
                        servers.push(addr);
                    }
                }
            }
            dns = entry.Next;
        }
        adapter = node.Next;
    }
    servers
}

/// Send one query to every server and wait for the first usable reply.
///
/// Servers of the same family share a socket, so the same packet fans out and
/// the replies are matched by id. A non-zero rcode is remembered but does not
/// end the wait: another resolver may still answer authoritatively, e.g. a
/// public resolver saying NXDOMAIN for an internal name the LAN resolver knows.
fn query_servers(
    servers: &[SocketAddr],
    name: &str,
    rtype: RRType,
) -> Result<Vec<Record>, DnsError> {
    let id = next_id();
    let query = build_query(id, name, rtype.code());

    // One socket per address family.
    let mut sockets: Vec<(bool, UdpSocket)> = Vec::new();
    let mut targets: Vec<(usize, SocketAddr)> = Vec::new();
    let mut net_err: Option<DnsError> = None;
    for &server in servers {
        if !is_usable_server(&server) {
            continue;
        }
        let v4 = server.is_ipv4();
        let idx = match sockets.iter().position(|(family, _)| *family == v4) {
            Some(i) => i,
            None => {
                let bind: SocketAddr = if v4 {
                    "0.0.0.0:0".parse().expect("valid bind")
                } else {
                    "[::]:0".parse().expect("valid bind")
                };
                match UdpSocket::bind(bind) {
                    Ok(socket) => {
                        sockets.push((v4, socket));
                        sockets.len() - 1
                    }
                    Err(e) => {
                        net_err = Some(DnsError::Network(e.to_string()));
                        continue;
                    }
                }
            }
        };
        targets.push((idx, server));
    }

    let mut awaiting = 0usize;
    for (idx, server) in &targets {
        match sockets[*idx].1.send_to(&query, server) {
            Ok(_) => awaiting += 1,
            Err(e) => net_err = Some(DnsError::Network(e.to_string())),
        }
    }
    if awaiting == 0 {
        return Err(net_err.unwrap_or(DnsError::Timeout));
    }

    let deadline = Instant::now() + QUERY_TIMEOUT;
    let mut buf = [0u8; 4096];
    let mut rcode_err: Option<(DnsError, Instant)> = None;
    while awaiting > 0 {
        let now = Instant::now();
        if now >= deadline {
            break;
        }
        if let Some((_, at)) = &rcode_err {
            if at.elapsed() >= RCODE_GRACE {
                break;
            }
        }
        let wait = (deadline - now).min(POLL_SLICE);
        for (_, socket) in &sockets {
            if socket.set_read_timeout(Some(wait)).is_err() {
                continue;
            }
            match socket.recv_from(&mut buf) {
                Ok((len, _)) => {
                    awaiting = awaiting.saturating_sub(1);
                    match parse_response(&buf[..len], id) {
                        Ok(records) => return Ok(records),
                        Err(e @ DnsError::Rcode(_)) => {
                            if rcode_err.is_none() {
                                rcode_err = Some((e, Instant::now()));
                            }
                        }
                        Err(_) => {}
                    }
                }
                Err(e) if is_timeout(&e) => {}
                Err(e) => {
                    awaiting = awaiting.saturating_sub(1);
                    net_err = Some(DnsError::Network(e.to_string()));
                }
            }
        }
    }

    if let Some((e, _)) = rcode_err {
        return Err(e);
    }
    Err(net_err.unwrap_or(DnsError::Timeout))
}

fn is_timeout(err: &std::io::Error) -> bool {
    matches!(
        err.kind(),
        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
    )
}

fn next_id() -> u16 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    ((nanos ^ (std::process::id() << 8)) & 0xffff) as u16
}

fn build_query(id: u16, name: &str, qtype: u16) -> Vec<u8> {
    let mut out = Vec::with_capacity(name.len() + 18);
    out.extend_from_slice(&id.to_be_bytes());
    out.extend_from_slice(&0x0100u16.to_be_bytes()); // RD = 1
    out.extend_from_slice(&1u16.to_be_bytes()); // QDCOUNT
    out.extend_from_slice(&[0u8; 6]); // AN/NS/AR count
    for label in name.split('.').filter(|l| !l.is_empty()) {
        let bytes = label.as_bytes();
        let len = bytes.len().min(63);
        out.push(len as u8);
        out.extend_from_slice(&bytes[..len]);
    }
    out.push(0);
    out.extend_from_slice(&qtype.to_be_bytes());
    out.extend_from_slice(&1u16.to_be_bytes()); // QCLASS = IN
    out
}

fn parse_response(buf: &[u8], id: u16) -> Result<Vec<Record>, DnsError> {
    if buf.len() < 12 {
        return Err(DnsError::Malformed);
    }
    let reply_id = u16::from_be_bytes([buf[0], buf[1]]);
    if reply_id != id {
        return Err(DnsError::Malformed);
    }
    let flags = u16::from_be_bytes([buf[2], buf[3]]);
    let rcode = (flags & 0x000f) as u8;
    let qdcount = u16::from_be_bytes([buf[4], buf[5]]) as usize;
    let ancount = u16::from_be_bytes([buf[6], buf[7]]) as usize;

    let mut pos = 12;
    for _ in 0..qdcount {
        let (_, next) = read_name(buf, pos).ok_or(DnsError::Malformed)?;
        pos = next + 4;
        if pos > buf.len() {
            return Err(DnsError::Malformed);
        }
    }

    if rcode != 0 {
        return Err(DnsError::Rcode(rcode));
    }

    let mut records = Vec::new();
    for _ in 0..ancount {
        let (record, next) = parse_rr(buf, pos)?;
        if let Some(record) = record {
            records.push(record);
        }
        pos = next;
    }
    Ok(records)
}

/// Reads a (possibly compressed) domain name. Returns the name and the offset
/// just past the name in the original position.
fn read_name(buf: &[u8], mut pos: usize) -> Option<(String, usize)> {
    let mut labels: Vec<String> = Vec::new();
    let mut next = pos;
    let mut jumped = false;
    let mut hops = 0usize;

    loop {
        let len = *buf.get(pos)?;
        if len == 0 {
            pos += 1;
            if !jumped {
                next = pos;
            }
            break;
        }
        if len & 0xc0 == 0xc0 {
            let low = *buf.get(pos + 1)?;
            let pointer = (((len & 0x3f) as usize) << 8) | low as usize;
            if !jumped {
                next = pos + 2;
                jumped = true;
            }
            hops += 1;
            if hops > 32 {
                return None;
            }
            pos = pointer;
            continue;
        }
        if len & 0xc0 != 0 {
            return None; // extended label types are not supported
        }
        let len = len as usize;
        let end = pos + 1 + len;
        if end > buf.len() {
            return None;
        }
        labels.push(String::from_utf8_lossy(&buf[pos + 1..end]).into_owned());
        pos = end;
    }

    // An empty label list is the DNS root, printed as `.` (null MX records).
    let name = if labels.is_empty() {
        String::from(".")
    } else {
        labels.join(".")
    };
    Some((name, next))
}

fn parse_rr(buf: &[u8], pos: usize) -> Result<(Option<Record>, usize), DnsError> {
    let (_, mut cursor) = read_name(buf, pos).ok_or(DnsError::Malformed)?;
    if cursor + 10 > buf.len() {
        return Err(DnsError::Malformed);
    }
    let rtype_code = u16::from_be_bytes([buf[cursor], buf[cursor + 1]]);
    cursor += 2; // type
    cursor += 2; // class
    let ttl = u32::from_be_bytes([
        buf[cursor],
        buf[cursor + 1],
        buf[cursor + 2],
        buf[cursor + 3],
    ]);
    cursor += 4;
    let rdlen = u16::from_be_bytes([buf[cursor], buf[cursor + 1]]) as usize;
    cursor += 2;
    let end = cursor + rdlen;
    if end > buf.len() {
        return Err(DnsError::Malformed);
    }

    let rtype = RRType::from_code(rtype_code);
    let data = match rtype {
        Some(RRType::A) if rdlen == 4 => Some(RData::Ip(IpAddr::V4(Ipv4Addr::new(
            buf[cursor],
            buf[cursor + 1],
            buf[cursor + 2],
            buf[cursor + 3],
        )))),
        Some(RRType::Aaaa) if rdlen == 16 => {
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&buf[cursor..cursor + 16]);
            Some(RData::Ip(IpAddr::V6(Ipv6Addr::from(octets))))
        }
        Some(RRType::Mx) if rdlen >= 3 => {
            let pref = u16::from_be_bytes([buf[cursor], buf[cursor + 1]]);
            let (exchange, _) = read_name(buf, cursor + 2).ok_or(DnsError::Malformed)?;
            Some(RData::Mx(pref, exchange))
        }
        Some(RRType::Ns) | Some(RRType::Cname) | Some(RRType::Ptr) => {
            let (name, _) = read_name(buf, cursor).ok_or(DnsError::Malformed)?;
            Some(RData::Name(name))
        }
        Some(RRType::Txt) => {
            let mut text = String::new();
            let mut at = cursor;
            while at < end {
                let chunk_len = buf[at] as usize;
                at += 1;
                let stop = (at + chunk_len).min(end);
                text.push_str(&String::from_utf8_lossy(&buf[at..stop]));
                at = stop;
            }
            Some(RData::Txt(text))
        }
        Some(RRType::Soa) => {
            let (mname, after_mname) = read_name(buf, cursor).ok_or(DnsError::Malformed)?;
            let (rname, after_rname) = read_name(buf, after_mname).ok_or(DnsError::Malformed)?;
            let at = after_rname;
            if at + 20 > buf.len() {
                return Err(DnsError::Malformed);
            }
            let word = |i: usize| {
                u32::from_be_bytes([
                    buf[at + i * 4],
                    buf[at + i * 4 + 1],
                    buf[at + i * 4 + 2],
                    buf[at + i * 4 + 3],
                ])
            };
            Some(RData::Soa {
                mname,
                rname,
                serial: word(0),
                refresh: word(1),
                retry: word(2),
                expire: word(3),
                minimum: word(4),
            })
        }
        _ => None,
    };

    Ok((
        data.map(|data| Record {
            rtype: rtype.unwrap(),
            ttl,
            data,
        }),
        end,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds one wire RR at `offset` (no compression).
    fn push_rr(out: &mut Vec<u8>, name: &str, rtype: u16, ttl: u32, rdata: &[u8]) {
        for label in name.split('.').filter(|l| !l.is_empty()) {
            out.push(label.len() as u8);
            out.extend_from_slice(label.as_bytes());
        }
        out.push(0);
        out.extend_from_slice(&rtype.to_be_bytes());
        out.extend_from_slice(&1u16.to_be_bytes());
        out.extend_from_slice(&ttl.to_be_bytes());
        out.extend_from_slice(&(rdata.len() as u16).to_be_bytes());
        out.extend_from_slice(rdata);
    }

    fn response(qname: &str, qtype: u16, answers: &[(u16, Vec<u8>)]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&0x1234u16.to_be_bytes());
        out.extend_from_slice(&0x8180u16.to_be_bytes());
        out.extend_from_slice(&1u16.to_be_bytes());
        out.extend_from_slice(&(answers.len() as u16).to_be_bytes());
        out.extend_from_slice(&[0, 0, 0, 0]);
        for label in qname.split('.').filter(|l| !l.is_empty()) {
            out.push(label.len() as u8);
            out.extend_from_slice(label.as_bytes());
        }
        out.push(0);
        out.extend_from_slice(&qtype.to_be_bytes());
        out.extend_from_slice(&1u16.to_be_bytes());
        for (rtype, rdata) in answers {
            push_rr(&mut out, qname, *rtype, 300, rdata);
        }
        out
    }

    #[test]
    fn query_encodes_header_and_question() {
        let q = build_query(0xbeef, "example.com", 15);
        assert_eq!(&q[..2], &[0xbe, 0xef]);
        assert_eq!(q[2], 0x01); // RD set
        assert_eq!(u16::from_be_bytes([q[4], q[5]]), 1);
        // 12 header bytes + 13 qname bytes + 4 type/class bytes
        assert_eq!(q.len(), 12 + 13 + 4);
        assert_eq!(&q[12..25], b"\x07example\x03com\x00");
        assert_eq!(u16::from_be_bytes([q[25], q[26]]), 15);
        assert_eq!(u16::from_be_bytes([q[27], q[28]]), 1);
    }

    #[test]
    fn parses_a_record() {
        let buf = response("example.com", 1, &[(1, vec![93, 184, 216, 34])]);
        let recs = parse_response(&buf, 0x1234).expect("valid response");
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].rtype, RRType::A);
        assert_eq!(recs[0].render(), "93.184.216.34");
        assert_eq!(recs[0].ttl, 300);
    }

    #[test]
    fn parses_aaaa_record() {
        let mut rdata = vec![0x26, 0x06, 0x28, 0x00];
        rdata.extend_from_slice(&[0u8; 12]);
        let buf = response("example.com", 28, &[(28, rdata)]);
        let recs = parse_response(&buf, 0x1234).unwrap();
        assert_eq!(recs[0].render(), "2606:2800::");
    }

    #[test]
    fn parses_mx_record() {
        let mut rdata = 10u16.to_be_bytes().to_vec();
        rdata.extend_from_slice(b"\x04mail\x07example\x03com\x00");
        let buf = response("example.com", 15, &[(15, rdata)]);
        let recs = parse_response(&buf, 0x1234).unwrap();
        assert_eq!(recs[0].render(), "10 mail.example.com");
    }

    #[test]
    fn parses_txt_chunks_into_one_value() {
        let rdata = b"\x05hello\x06 world".to_vec();
        let buf = response("example.com", 16, &[(16, rdata)]);
        let recs = parse_response(&buf, 0x1234).unwrap();
        assert_eq!(recs[0].render(), "\"hello world\"");
    }

    #[test]
    fn parses_soa_fields_in_order() {
        let mut rdata = Vec::new();
        rdata.extend_from_slice(b"\x02ns\x07example\x03com\x00");
        rdata.extend_from_slice(b"\x0ahostmaster\x07example\x03com\x00");
        for word in [2024u32, 7200, 3600, 1209600, 3600] {
            rdata.extend_from_slice(&word.to_be_bytes());
        }
        let buf = response("example.com", 6, &[(6, rdata)]);
        let recs = parse_response(&buf, 0x1234).unwrap();
        assert_eq!(
            recs[0].render(),
            "ns.example.com hostmaster.example.com 2024 7200 3600 1209600 3600"
        );
    }

    #[test]
    fn follows_compression_pointers() {
        // name at offset 12, then a pointer back to it
        let mut buf = Vec::new();
        buf.extend_from_slice(&0x1234u16.to_be_bytes());
        buf.extend_from_slice(&0x8180u16.to_be_bytes());
        buf.extend_from_slice(&1u16.to_be_bytes());
        buf.extend_from_slice(&1u16.to_be_bytes());
        buf.extend_from_slice(&[0, 0, 0, 0]);
        buf.extend_from_slice(b"\x07example\x03com\x00");
        buf.extend_from_slice(&1u16.to_be_bytes());
        buf.extend_from_slice(&1u16.to_be_bytes());
        // answer: pointer to offset 12, type A, class IN, ttl 60, rdlen 4
        buf.extend_from_slice(&[0xc0, 12]);
        buf.extend_from_slice(&1u16.to_be_bytes());
        buf.extend_from_slice(&1u16.to_be_bytes());
        buf.extend_from_slice(&60u32.to_be_bytes());
        buf.extend_from_slice(&4u16.to_be_bytes());
        buf.extend_from_slice(&[10, 0, 0, 1]);

        let recs = parse_response(&buf, 0x1234).unwrap();
        assert_eq!(recs[0].render(), "10.0.0.1");
    }

    #[test]
    fn nxdomain_is_reported_as_rcode() {
        let mut buf = response("nope.example", 1, &[]);
        buf[3] = 0x83; // rcode 3
        match parse_response(&buf, 0x1234) {
            Err(DnsError::Rcode(3)) => {}
            other => panic!("expected NXDOMAIN, got {:?}", other),
        }
    }

    #[test]
    fn mismatched_id_is_rejected() {
        let buf = response("example.com", 1, &[]);
        assert!(matches!(
            parse_response(&buf, 0x9999),
            Err(DnsError::Malformed)
        ));
    }

    #[test]
    fn reverse_name_for_ipv4() {
        assert_eq!(
            reverse_name("8.8.8.8").as_deref(),
            Some("8.8.8.8.in-addr.arpa")
        );
        assert_eq!(
            reverse_name("192.168.1.10").as_deref(),
            Some("10.1.168.192.in-addr.arpa")
        );
    }

    #[test]
    fn reverse_name_for_ipv6() {
        let name = reverse_name("2001:db8::1").expect("valid ipv6");
        assert!(name.ends_with("ip6.arpa"));
        assert!(name.starts_with("1.0.0.0."));
    }

    #[test]
    fn reverse_name_rejects_domains() {
        assert!(reverse_name("example.com").is_none());
    }

    #[test]
    fn unusable_servers_are_skipped() {
        assert!(is_usable_server(&"1.1.1.1:53".parse().unwrap()));
        assert!(is_usable_server(&"127.0.0.1:53".parse().unwrap()));
        // Link-local IPv6 needs a scope id we cannot carry here.
        assert!(!is_usable_server(&"[fe80::1]:53".parse().unwrap()));
        assert!(!is_usable_server(&"0.0.0.0:53".parse().unwrap()));
        assert!(!is_usable_server(&"[::]:53".parse().unwrap()));
    }

    #[test]
    fn one_live_server_is_enough_even_next_to_a_dead_one() {
        // A dead TEST-NET-1 resolver first: the request must still be answered
        // by the live loopback server without waiting out a timeout per server.
        let server = UdpSocket::bind("127.0.0.1:0").expect("bind fake resolver");
        let addr = server.local_addr().expect("local addr");
        let responder = std::thread::spawn(move || {
            let mut buf = [0u8; 512];
            let (len, from) = server.recv_from(&mut buf).expect("query received");
            let mut reply = response("example.com", 1, &[(1, vec![10, 0, 0, 7])]);
            reply[0] = buf[0];
            reply[1] = buf[1];
            assert!(len >= 12);
            server.send_to(&reply, from).expect("reply sent");
        });

        let dead: SocketAddr = "192.0.2.1:53".parse().unwrap();
        let started = Instant::now();
        let records = query_servers(&[dead, addr], "example.com", RRType::A).expect("answer");
        let elapsed = started.elapsed();
        responder.join().expect("responder finished");

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].render(), "10.0.0.7");
        assert!(
            elapsed < Duration::from_millis(1000),
            "lookup waited out the dead server: {:?}",
            elapsed
        );
    }

    #[test]
    fn nxdomain_returns_quickly_despite_a_dead_server() {
        let server = UdpSocket::bind("127.0.0.1:0").expect("bind fake resolver");
        let addr = server.local_addr().expect("local addr");
        let responder = std::thread::spawn(move || {
            let mut buf = [0u8; 512];
            let (_, from) = server.recv_from(&mut buf).expect("query received");
            let mut reply = response("nope.example", 1, &[]);
            reply[0] = buf[0];
            reply[1] = buf[1];
            reply[3] = 0x83; // rcode 3 = NXDOMAIN
            server.send_to(&reply, from).expect("reply sent");
        });

        let dead: SocketAddr = "192.0.2.1:53".parse().unwrap();
        let started = Instant::now();
        let err = query_servers(&[dead, addr], "nope.example", RRType::A).unwrap_err();
        let elapsed = started.elapsed();
        responder.join().expect("responder finished");

        assert!(matches!(err, DnsError::Rcode(3)), "got {:?}", err);
        assert!(
            elapsed < Duration::from_millis(900),
            "NXDOMAIN waited out the deadline: {:?}",
            elapsed
        );
    }

    #[test]
    fn record_type_labels_roundtrip() {
        for label in ["NS", "CNAME", "SOA", "PTR", "MX", "TXT"] {
            let rtype = RRType::from_label(label).expect("known type");
            assert_eq!(rtype.label(), label);
            assert_eq!(RRType::from_code(rtype.code()), Some(rtype));
        }
        assert!(RRType::from_label("BOGUS").is_none());
    }
}
