macro_rules! match_ident {
    ( $input:expr => { $( $ident:literal => $ident_expr:expr ),+ , $else_id:ident => $($else_expr:tt)+ } ) => {{
        let ident = $input.step(|cursor|{
            if let Some((ident, rest)) = cursor.ident() {
                if false {}
                $( else if ident == $ident { return Ok(($ident, rest)); } )+
            }
            Err(cursor.error(concat!("Expected one of: ", $(stringify!($ident), " "),+)))
        });
        match ident {
            $( Ok($ident) => $ident_expr ),+,
            $else_id => $($else_expr)+
        }
    }};
}
