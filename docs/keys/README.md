# Release signing keys

Java 0.2.0 and future releases use the replacement **IronDome Release Signing**
OpenPGP key generated for the 2026-10-27 release:

```text
0CA8 828C D11C 0096 9478 3F29 ACFB 5AAA 1CEE 5253
```

The [current public key](irondome-release-2026-10.asc) is available in this
repository and on `keyserver.ubuntu.com`. The previous private signing key was
unavailable on the release VM; the owner authorized this replacement. Existing
artifacts and signatures are unchanged.

Java 0.1.x artifacts retain their original signing key:

```text
A4C4 2DDB 3A8F CF2E EE98 84BF 0EE6 1259 1D3E FD14
```

Keep the [original public key](irondome-release.asc) to verify those releases.
Only public keys belong in this repository. Private keys, passphrases and
publishing credentials stay in private release storage and encrypted CI secrets.

For Java 0.2.0:

```sh
gpg --import irondome-release-2026-10.asc
gpg --verify mail-canon-0.2.0.jar.asc mail-canon-0.2.0.jar
```

Check that the signature's fingerprint matches the corresponding release above.
A valid signature establishes which key signed the artifact; it does not establish
mailbox ownership or change the normalization policy.
