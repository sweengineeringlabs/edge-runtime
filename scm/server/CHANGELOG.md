# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.3.2]

### Fixed

- Resolve a duplicate-`edge-domain` diamond that prevented `swe-edge-runtime-server`
  from compiling as a standalone git dependency (#9). The `v0.3.1` graph pulled two
  semver-incompatible `edge-domain` versions — `v0.3.0` (via the then-bundled
  `swe-edge-ingress-http v0.3.0` and `swe-edge-ingress-verifier v0.4.1` from
  `edge-ingress-security`) alongside `v0.8.0` — so trait `impl`s in `ingress`
  adapters failed type-checking with `RequestContext` / `TokenVerifier` /
  `IngressTlsConfig` / `Handler` mismatches. All ingress dependencies are now
  aligned to a single `edge-domain v0.8.1`, and `swe-edge-ingress-http` is consumed
  from its standalone repository. Verified by an external git-dependency build.
