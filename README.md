# FileToVariables
A small CLI tool to split small files into base64 blocks and read and combine base64 blocks from environment variables into the original file data.

Suitable for storing files into CI/CD variables, e.g. certificates.

[![Release](https://github.com/VPKSoftOrg/FileToVariables/actions/workflows/release.yml/badge.svg)](https://github.com/VPKSoftOrg/FileToVariables/actions/workflows/release.yml) [![Rust](https://github.com/VPKSoftOrg/FileToVariables/actions/workflows/rust.yml/badge.svg)](https://github.com/VPKSoftOrg/FileToVariables/actions/workflows/rust.yml)

`.\FileToVariables.exe --help`:
```
A simple program to encode data to base64 and decode it back

Usage: FileToVariables.exe [OPTIONS] --file <FILE> --var-format <VAR_FORMAT> <COMMAND>

Commands:
  read   Reads the variables from environment variables and writes them to a file
  write  Writes the variables from a file to environment variables
  help   Print this message or the help of the given subcommand(s)

Options:
  -f, --file <FILE>                    File to read the data from
  -v, --var-format <VAR_FORMAT>        The format of the environment variable. E.g. "data_xyz{i}"
  -b, --block <BLOCK>                  The base64 block size. This indicates how many bytes are used in a base64 block
      --shell-flavour <SHELL_FLAVOUR>  The optional shell-flavour to use in case the data is written to an environment variable. Allowed values: powershell, cmd, bash [possible values: power-shell, bash, cmd]
  -h, --help                           Print help
  -V, --version                        Print version
```

`.\FileToVariables.exe --file ../../LICENSE --var-format "dataxyz{i}" --block 400 write`:
```
dataxyz01=TUlUIExpY2Vuc2UKCkNvcHlyaWdodCAoYykgMjAyNCBWUEtTb2Z0T3JnCgpQZXJtaXNzaW9uIGlzIGhlcmVieSBncmFudGVkLCBmcmVlIG9mIGNoYXJnZSwgdG8gYW55IHBlcnNvbiBvYnRhaW5pbmcgYSBjb3B5Cm9mIHRoaXMgc29mdHdhcmUgYW5kIGFzc29jaWF0ZWQgZG9jdW1lbnRhdGlvbiBmaWxlcyAodGhlICJTb2Z0d2FyZSIpLCB0byBkZWFsCmluIHRoZSBTb2Z0d2FyZSB3aXRob3V0IHJlc3RyaWN0aW9uLCBpbmNsdWRpbmcgd2l0aG91dCBsaW1pdGF0aW9uIHRoZSByaWdodHMKdG8gdXNlLCBjb3B5LCBtb2RpZnksIG1lcmdlLCBwdWJsaXNoLCBkaXN0cmlidXRlLCBzdWJsaWNlbnNlLCBhbmQvb3Igc2VsbApjb3BpZXMgb2YgdGhlIFNvZnR3YXJlLCBhbmQgdG8gcGVybWl0IHBlcnNvbnMgdG8gdw==
dataxyz02=aG9tIHRoZSBTb2Z0d2FyZSBpcwpmdXJuaXNoZWQgdG8gZG8gc28sIHN1YmplY3QgdG8gdGhlIGZvbGxvd2luZyBjb25kaXRpb25zOgoKVGhlIGFib3ZlIGNvcHlyaWdodCBub3RpY2UgYW5kIHRoaXMgcGVybWlzc2lvbiBub3RpY2Ugc2hhbGwgYmUgaW5jbHVkZWQgaW4gYWxsCmNvcGllcyBvciBzdWJzdGFudGlhbCBwb3J0aW9ucyBvZiB0aGUgU29mdHdhcmUuCgpUSEUgU09GVFdBUkUgSVMgUFJPVklERUQgIkFTIElTIiwgV0lUSE9VVCBXQVJSQU5UWSBPRiBBTlkgS0lORCwgRVhQUkVTUyBPUgpJTVBMSUVELCBJTkNMVURJTkcgQlVUIE5PVCBMSU1JVEVEIFRPIFRIRSBXQVJSQU5USUVTIE9GIE1FUkNIQU5UQUJJTElUWSwKRklUTkVTUyBGT1IgQSBQQVJUSUNVTEFSIFBVUlBPU0UgQU5EIE5PTklORlJJTg==
dataxyz03=R0VNRU5ULiBJTiBOTyBFVkVOVCBTSEFMTCBUSEUKQVVUSE9SUyBPUiBDT1BZUklHSFQgSE9MREVSUyBCRSBMSUFCTEUgRk9SIEFOWSBDTEFJTSwgREFNQUdFUyBPUiBPVEhFUgpMSUFCSUxJVFksIFdIRVRIRVIgSU4gQU4gQUNUSU9OIE9GIENPTlRSQUNULCBUT1JUIE9SIE9USEVSV0lTRSwgQVJJU0lORyBGUk9NLApPVVQgT0YgT1IgSU4gQ09OTkVDVElPTiBXSVRIIFRIRSBTT0ZUV0FSRSBPUiBUSEUgVVNFIE9SIE9USEVSIERFQUxJTkdTIElOIFRIRQpTT0ZUV0FSRS4K
```
`.\FileToVariables.exe --file ../../LICENSE_NEW --var-format "dataxyz{i}" read`:
&rarr; We get the MIT license from the environment variables back.

## Thanks to:
* [upload-rust-binary-action](https://github.com/taiki-e/upload-rust-binary-action)
