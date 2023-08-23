# exports
Utility to list the exported functions in an object file i.e. PE, DLL, SYS, COFF, ELF, SO, ...

## Usage
Single file

`export obj1.dll`

Multiple files

`exports obj1.dll obj2.exe obj3.sys ...`

From cargo

`cargo run -- obj1.dll obj2.exe`

### Output
`exports C:\Windows\System32\ntdll.dll`

```
\\?\C:\Windows\System32\ntdll.dll
├── 0x18000e6e0: A_SHAFinal
├── 0x18000e810: A_SHAInit
├── 0x18000e850: A_SHAUpdate
├── 0x1800eaff0: AlpcAdjustCompletionListConcurrencyCount
├── 0x18007e290: AlpcFreeCompletionListMessage
├── 0x1800eb020: AlpcGetCompletionListLastMessageInformation
├── 0x1800eb040: AlpcGetCompletionListMessageAttributes
├── 0x1800727d0: AlpcGetHeaderSize
├── 0x180072790: AlpcGetMessageAttribute
...
├── 0x180094e60: wcstol
├── 0x180094f00: wcstombs
└── 0x180094ec0: wcstoul
```
