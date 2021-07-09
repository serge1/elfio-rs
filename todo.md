# TODO

## Not yet done

- Implement Debug for Elfio and (maybe) other Structures/Traits
- Decide regarding template args. Maybe a single param is enough

## Done

- Add all ELF header fields
- Make header functions be a part of Elfio struct
- Add tests
- Imlement proper constructor for the header.
  Give correct initial values. Note: the rest of the header
  initialization will be done during implemention of 'save' functions
- Add Copyright notes
- Implement 'set' functions for ELF header
- Try to move 'load' function to a separate trait
  The default implementation cannot be done - it requires type Size
- Learn and add documentation
- Start to use endianness converter
- Investigate use std::convert::TryInto
- Implement Section Trait and Struct
- Implement Segment Trait and Struct
- Implement data access for sections and segments
- Implement string table section accessor
- All 'load' functions accept a generic ElfioReadSeek parameter for file/memory access
- Implement symbol table section accessor
- Implement Note section accessor
- Add examples for Note and Symbol accessors
- Implement Relocation section accessor
- Implement Array section accessor
- Implement Dynamic section accessor
- Implement ModInfo section accessor
