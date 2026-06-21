//! PeerIdentityExtractor — extracts peer identity from DER-encoded leaf certificates.

use std::collections::HashMap;

use sha2::{Digest, Sha256};

use swe_edge_ingress_grpc::{
    PEER_CERT_FINGERPRINT_SHA256, PEER_CN, PEER_IDENTITY, PEER_SAN_DNS, PEER_SAN_URI,
};

const TAG_SEQUENCE: u8 = 0x30;
const TAG_SET: u8 = 0x31;
const TAG_OID: u8 = 0x06;
const TAG_UTF8: u8 = 0x0C;
const TAG_PRINTABLE: u8 = 0x13;
const TAG_IA5: u8 = 0x16;
const TAG_CONTEXT_0: u8 = 0xA0;
const TAG_CONTEXT_3: u8 = 0xA3;
const TAG_OCTET_STRING: u8 = 0x04;
const TAG_BOOLEAN: u8 = 0x01;

const SAN_DNS_TAG: u8 = 0x82;
const SAN_URI_TAG: u8 = 0x86;

const OID_COMMON_NAME: &[u8] = &[0x55, 0x04, 0x03];
const OID_SUBJECT_ALT_NAME: &[u8] = &[0x55, 0x1D, 0x11];

/// Extractor for peer identity from DER-encoded leaf certificates.
pub(crate) struct PeerIdentityExtractor;

impl PeerIdentityExtractor {
    /// Extract peer-identity key/value pairs from a DER-encoded leaf cert.
    ///
    /// Returns at minimum a SHA-256 fingerprint; CN / SAN / DN are added
    /// when the cert structure is parseable.
    pub(crate) fn extract(leaf_der: &[u8]) -> HashMap<String, String> {
        Self::extract_impl(leaf_der)
    }

    fn extract_impl(leaf_der: &[u8]) -> HashMap<String, String> {
        let mut out = HashMap::new();
        let fp = Self::hex_lower(&Sha256::digest(leaf_der));
        out.insert(PEER_CERT_FINGERPRINT_SHA256.to_string(), fp);

        let Some((_, cert_body)) = Self::read_tlv(leaf_der) else {
            return out;
        };
        if cert_body.is_empty() {
            return out;
        };
        let Some((tag, tbs_body)) = Self::read_tlv(cert_body) else {
            return out;
        };
        if tag != TAG_SEQUENCE {
            return out;
        };

        let mut rest = tbs_body;
        if let Some(b) = rest.first() {
            if *b == TAG_CONTEXT_0 {
                let Some((_, _, after)) = Self::read_tlv_with_remainder(rest) else {
                    return out;
                };
                rest = after;
            }
        }
        let Some((_, _, after)) = Self::read_tlv_with_remainder(rest) else {
            return out;
        };
        rest = after;
        let Some((_, _, after)) = Self::read_tlv_with_remainder(rest) else {
            return out;
        };
        rest = after;
        let Some((_, _, after)) = Self::read_tlv_with_remainder(rest) else {
            return out;
        };
        rest = after;
        let Some((_, _, after)) = Self::read_tlv_with_remainder(rest) else {
            return out;
        };
        rest = after;
        let Some((tag, subject_body, after)) = Self::read_tlv_with_remainder(rest) else {
            return out;
        };
        if tag == TAG_SEQUENCE {
            let dn = Self::render_name(subject_body);
            if !dn.is_empty() {
                out.insert(PEER_IDENTITY.to_string(), dn);
            }
            if let Some(cn) = Self::find_common_name(subject_body) {
                out.insert(PEER_CN.to_string(), cn);
            }
        }
        rest = after;
        let Some((_, _, after)) = Self::read_tlv_with_remainder(rest) else {
            return out;
        };
        rest = after;

        while let Some((tag, body, after)) = Self::read_tlv_with_remainder(rest) {
            if tag == TAG_CONTEXT_3 {
                if let Some((TAG_SEQUENCE, ext_seq)) = Self::read_tlv(body) {
                    if let Some((dns, uri)) = Self::find_san_in_extensions(ext_seq) {
                        if !dns.is_empty() {
                            out.insert(PEER_SAN_DNS.to_string(), dns.join(","));
                        }
                        if !uri.is_empty() {
                            out.insert(PEER_SAN_URI.to_string(), uri.join(","));
                        }
                    }
                }
                break;
            }
            rest = after;
        }
        out
    }

    fn read_tlv_with_remainder(data: &[u8]) -> Option<(u8, &[u8], &[u8])> {
        if data.is_empty() {
            return None;
        }
        let tag = data[0];
        let (len, header_len) = Self::read_length(&data[1..])?;
        let total = 1 + header_len + len;
        if data.len() < total {
            return None;
        }
        Some((tag, &data[1 + header_len..total], &data[total..]))
    }

    fn read_tlv(data: &[u8]) -> Option<(u8, &[u8])> {
        Self::read_tlv_with_remainder(data).map(|(t, b, _)| (t, b))
    }

    fn read_length(data: &[u8]) -> Option<(usize, usize)> {
        let first = *data.first()?;
        if first & 0x80 == 0 {
            return Some((first as usize, 1));
        }
        let n = (first & 0x7F) as usize;
        if n == 0 || n > 4 || data.len() < 1 + n {
            return None;
        }
        let mut len = 0usize;
        for &b in &data[1..1 + n] {
            len = (len << 8) | b as usize;
        }
        Some((len, 1 + n))
    }

    fn render_name(name_seq: &[u8]) -> String {
        let mut out = Vec::new();
        let mut rest = name_seq;
        while let Some((tag, body, after)) = Self::read_tlv_with_remainder(rest) {
            if tag == TAG_SET {
                let mut atv_rest = body;
                while let Some((atv_tag, atv_body, atv_after)) =
                    Self::read_tlv_with_remainder(atv_rest)
                {
                    if atv_tag == TAG_SEQUENCE {
                        if let Some((kind, _key, value)) = Self::parse_atv(atv_body) {
                            out.push(format!("{kind}={value}"));
                        }
                    }
                    atv_rest = atv_after;
                }
            }
            rest = after;
        }
        out.join(",")
    }

    fn parse_atv(body: &[u8]) -> Option<(&'static str, &[u8], String)> {
        let (oid_tag, oid_body, after) = Self::read_tlv_with_remainder(body)?;
        if oid_tag != TAG_OID {
            return None;
        }
        let (val_tag, val_body, _) = Self::read_tlv_with_remainder(after)?;
        let kind = Self::oid_short_name(oid_body)?;
        let value = Self::decode_string(val_tag, val_body)?;
        Some((kind, oid_body, value))
    }

    fn oid_short_name(oid: &[u8]) -> Option<&'static str> {
        if oid == OID_COMMON_NAME {
            return Some("CN");
        }
        if oid == [0x55, 0x04, 0x0A] {
            return Some("O");
        }
        if oid == [0x55, 0x04, 0x0B] {
            return Some("OU");
        }
        if oid == [0x55, 0x04, 0x06] {
            return Some("C");
        }
        if oid == [0x55, 0x04, 0x07] {
            return Some("L");
        }
        if oid == [0x55, 0x04, 0x08] {
            return Some("ST");
        }
        None
    }

    fn decode_string(tag: u8, body: &[u8]) -> Option<String> {
        match tag {
            TAG_UTF8 | TAG_PRINTABLE | TAG_IA5 => {
                std::str::from_utf8(body).ok().map(str::to_string)
            }
            _ => None,
        }
    }

    fn find_common_name(name_seq: &[u8]) -> Option<String> {
        let mut rest = name_seq;
        while let Some((tag, body, after)) = Self::read_tlv_with_remainder(rest) {
            if tag == TAG_SET {
                let mut atv_rest = body;
                while let Some((atv_tag, atv_body, atv_after)) =
                    Self::read_tlv_with_remainder(atv_rest)
                {
                    if atv_tag == TAG_SEQUENCE {
                        if let Some((kind, _oid, value)) = Self::parse_atv(atv_body) {
                            if kind == "CN" {
                                return Some(value);
                            }
                        }
                    }
                    atv_rest = atv_after;
                }
            }
            rest = after;
        }
        None
    }

    fn find_san_in_extensions(extensions_seq: &[u8]) -> Option<(Vec<String>, Vec<String>)> {
        let mut rest = extensions_seq;
        while let Some((tag, ext_body, after)) = Self::read_tlv_with_remainder(rest) {
            if tag == TAG_SEQUENCE {
                if let Some((oid_tag, oid_body, ext_rest)) = Self::read_tlv_with_remainder(ext_body)
                {
                    if oid_tag == TAG_OID && oid_body == OID_SUBJECT_ALT_NAME {
                        let mut payload_rest = ext_rest;
                        if let Some((maybe_bool_tag, _bool_body, after_bool)) =
                            Self::read_tlv_with_remainder(payload_rest)
                        {
                            if maybe_bool_tag == TAG_BOOLEAN {
                                payload_rest = after_bool;
                            }
                        }
                        if let Some((octets_tag, octets_body, _)) =
                            Self::read_tlv_with_remainder(payload_rest)
                        {
                            if octets_tag == TAG_OCTET_STRING {
                                if let Some((TAG_SEQUENCE, gn_seq)) = Self::read_tlv(octets_body) {
                                    return Some(Self::parse_general_names(gn_seq));
                                }
                            }
                        }
                    }
                }
            }
            rest = after;
        }
        None
    }

    fn parse_general_names(data: &[u8]) -> (Vec<String>, Vec<String>) {
        let mut dns = Vec::new();
        let mut uri = Vec::new();
        let mut rest = data;
        while let Some((tag, body, after)) = Self::read_tlv_with_remainder(rest) {
            match tag {
                SAN_DNS_TAG => {
                    if let Ok(s) = std::str::from_utf8(body) {
                        dns.push(s.to_string());
                    }
                }
                SAN_URI_TAG => {
                    if let Ok(s) = std::str::from_utf8(body) {
                        uri.push(s.to_string());
                    }
                }
                _ => {}
            }
            rest = after;
        }
        (dns, uri)
    }

    fn hex_lower(bytes: &[u8]) -> String {
        let mut out = String::with_capacity(bytes.len() * 2);
        for b in bytes {
            out.push(Self::hex_char(b >> 4));
            out.push(Self::hex_char(b & 0x0F));
        }
        out
    }

    fn hex_char(n: u8) -> char {
        match n {
            0..=9 => (b'0' + n) as char,
            10..=15 => (b'a' + n - 10) as char,
            _ => '?',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_ingress_grpc::PEER_CERT_FINGERPRINT_SHA256;

    /// Empty DER input must not panic — returns at least the fingerprint key.
    #[test]
    fn test_extract_empty_der_returns_fingerprint_happy() {
        let out = PeerIdentityExtractor::extract(&[]);
        // SHA-256 of empty input is deterministic.
        assert!(
            out.contains_key(PEER_CERT_FINGERPRINT_SHA256),
            "fingerprint must always be present"
        );
        // SHA-256("") is 64 hex chars.
        assert_eq!(out[PEER_CERT_FINGERPRINT_SHA256].len(), 64);
    }

    /// Random garbage DER must not panic — graceful fallback to fingerprint only.
    #[test]
    fn test_extract_garbage_der_does_not_panic_error() {
        let garbage = b"\xFF\xFE\xFD\xFC\xFB";
        let out = PeerIdentityExtractor::extract(garbage);
        assert!(
            out.contains_key(PEER_CERT_FINGERPRINT_SHA256),
            "fingerprint must always be present even for garbage input"
        );
        // Must not contain CN or SAN — no valid structure to parse.
        assert!(!out.contains_key(swe_edge_ingress_grpc::PEER_CN));
    }

    /// A truncated sequence TLV must fall back gracefully rather than panicking.
    #[test]
    fn test_extract_truncated_sequence_falls_back_edge() {
        // 0x30 = SEQUENCE, 0x82 = long-form length (2 bytes), 0x01, 0x00 = 256 bytes,
        // but we only provide 1 body byte — simulates truncation.
        let truncated = &[0x30u8, 0x82, 0x01, 0x00, 0xAB];
        let out = PeerIdentityExtractor::extract(truncated);
        // Must not panic and fingerprint must be present.
        assert!(out.contains_key(PEER_CERT_FINGERPRINT_SHA256));
    }
}
