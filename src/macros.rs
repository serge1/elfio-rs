// --------------------------------------------------------------------------
macro_rules! ELFIO_HEADER_ACCESS_GET {
    ($type: ident, $name: ident) => {
        paste::paste! {
            /// Read access to the corresponding ELF header field
            pub fn [<get_ $name>](&self) -> $type {
                self.header.[<get_ $name>]()
            }
        }
    };
}

macro_rules! ELFIO_HEADER_ACCESS_GET_SET {
    ($type: ident, $name: ident) => {
        paste::paste! {
            /// Read access to the corresponding ELF header field
            pub fn [<get_ $name>](&self) -> $type {
                self.header.[<get_ $name>]()
            }

            /// Write access to the corresponding ELF header field
            pub fn [<set_ $name>](&mut self, value: $type) -> () {
                self.header.[<set_ $name>](value);
            }
        }
    };
}

// --------------------------------------------------------------------------
macro_rules! ELFIO_GET_ACCESS_DECL {
    ($type: ident, $name: ident) => {
        paste::paste! {
            fn [<get_ $name>](&self) -> $type;
        }
    };
}

macro_rules! ELFIO_GET_SET_ACCESS_DECL {
    ($type: ident, $name: ident) => {
        paste::paste! {
            fn [<get_ $name>](&self) -> $type;
            fn [<set_ $name>](&mut self, value: $type);
        }
    };
}

macro_rules! ELFIO_GET_ACCESS {
    ($type: ident, $name: ident, $field: expr) => {
        paste::paste! {
            fn [<get_ $name>](&self) -> $type {
                paste::paste! [self. $field].as_()
            }
        }
    };
}

macro_rules! ELFIO_GET_SET_ACCESS {
    ($type: ident, $name: ident, $field: expr) => {
        paste::paste! {
            fn [<get_ $name>](&self) -> $type {
                paste::paste! [self. $field].as_()
            }
            fn [<set_ $name>](&mut self, value: $type) {
                paste::paste! [self. $field] = (value).as_();
            }
        }
    };
}
