# exports
Utility to list the exported functions in an object file i.e. PE, DLL, SYS, COFF, ELF, SO, ...

## Usage
Single file

`export obj1.dll`

Multiple files

`exports obj1.dll obj2.exe obj3.sys ...`

From cargo

`cargo run -- obj1.dll obj2.exe`
