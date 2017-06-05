error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
    }

    errors {
        InvalidEscape {
        }

        InvalidVariable {
        }

        VariableUnderflow {
        }
    }
}
