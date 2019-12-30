macro_rules! expect {
    ( ok: $e:expr; else $on_err:expr; $($tt:tt)* ) => {
        match $e {
            Ok(r) => r,
            Err(error) => {
                eprintln!($($tt)*, error = error);
                $on_err
            },
        }
    };
    ( some: $e:expr; else $on_err:expr; $($tt:tt)* ) => {
        match $e {
            Some(r) => r,
            None => {
                eprintln!($($tt)*);
                $on_err
            },
        }
    };
    ( failed: $on_err:expr; $($tt:tt)* ) => {
        {
            eprintln!($($tt)*);
            $on_err
        }
    };
    ( $value:expr; $e:expr; else $on_err:expr; $($tt:tt)* ) => {
        match $e {
            r @ $value => r,
            _ => {
                eprintln!($($tt)*);
                $on_err
            },
        }
    };
}
