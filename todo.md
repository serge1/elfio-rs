# TODO

## Not yet done

- Implement 'set' functions for ELF header
- Learn and add documentation
- Try to move 'load' function to a separate trait
  with default implementation
- Start to use endianness converter
- Add Copyright notes

## Done

- Add all ELF header fields
- Make header functions be a part of Elfio struct
- Add tests
- Imlement proper constructor for the header.
  Give correct initial values. Note: the rest of the header
  initialization will be done during implemention of 'save' functions
