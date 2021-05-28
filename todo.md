# TODO

## Not yet done

- Start to use endianness converter
- Investigate use std::convert::TryInto

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
