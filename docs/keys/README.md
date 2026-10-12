# Release signing key

Java artifacts are signed with the **IronDome Release Signing** OpenPGP key:

```text
A4C4 2DDB 3A8F CF2E EE98 84BF 0EE6 1259 1D3E FD14
```

The [public key](irondome-release.asc) is also available from `keyserver.ubuntu.com`. Only the public key belongs in this repository. The private key and publishing credentials remain on the release workstation.

To verify a downloaded Maven artifact and its detached signature:

```sh
gpg --import irondome-release.asc
gpg --verify mail-canon-0.1.0.jar.asc mail-canon-0.1.0.jar
```

Check that the reported fingerprint matches the one above. A valid signature establishes which key signed the artifact; it does not establish mailbox ownership or change the normalization policy.
