# rush-svm

> [!IMPORTANT]
> This package contains the shared modules across programs and tests most notably the following:

- **Instruction** Enum
- **State** structs
- **PDA** Helpers
- **Error** - SVM-Space Error
- **Macros**
- **Client** (Non Generated)

> [!CAUTION]
> Onchain programs don't support some dependencies in client-side Solana (e.g. `getrandom`). To prevent unsupported module errors, **Program-specific** code is separated from **Client-specific** code with the `#[cfg(target_os = "solana")` compilation flag.
