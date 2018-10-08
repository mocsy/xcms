// #[macro_use]
#[allow(unused)]
macro_rules! ructe_block {
    ($func:expr $(, $args:expr)*) => (
        {
            let mut buf = Vec::new();
            $func(&mut buf, $( $args ),*).unwrap();
            let res = String::from_utf8(buf).unwrap();
            res
        }
    );
}
#[allow(unused)]
macro_rules! ructe_page {
    ($func:expr, $title:expr, $meta:expr $(, $args:expr)*) => (
        {
            let block = ructe_block!($func, $( $args ),*);

            let mut buf = Vec::new();
            $crate::templates::page(&mut buf,
                $title,
                &$crate::templates::Html(block), &$crate::templates::Html($meta)
            ).unwrap();
            let res = String::from_utf8(buf).unwrap();
            res
        }
    );
}
#[allow(unused)]
#[macro_export]
macro_rules! ructe_block_res {
    ($func:expr $(, $args:expr)*) => (
        {
            let mut buf = Vec::new();
            match $func(&mut buf, $( $args ),*) {
                Ok(_) => {
                    let res = String::from_utf8(buf)?;
                    Ok(res)
                },
                Err(e) => Err(e),
            }
        }
    );
}
#[allow(unused)]
#[macro_export]
macro_rules! ructe_block_closure {
    ($func:expr $(, $args:expr)*) => (
        ||
        {
            let mut buf = Vec::new();
            match $func(&mut buf, $( $args ),*) {
                Ok(_) => {
                    let res = String::from_utf8(buf)?;
                    Ok(res)
                },
                Err(e) => Err(Failure::from(e)),
            }
        }
    );
}
#[allow(unused)]
#[macro_export]
macro_rules! ructe_page_res {
    ($func:expr, $meta:ident $(, $args:expr)*) => (
        {
            let block = ructe_block_res!($func, $( $args ),*)?;

            let mut buf = Vec::new();
            match $crate::templates::page(&mut buf,
                &$crate::templates::Html(block),
                &$meta) {
                Ok(_) => {
                    let res = String::from_utf8(buf)?;
                    Ok(res)
                },
                Err(e) => Err($crate::render::Failure::from(e)),
            }
        }
    );
}

//ructe_page_res!(crate::templates::navigation::frame, meta, &toplinks, &links, &list)
#[allow(unused)]
#[macro_export]
macro_rules! pnc_page {
    ($meta:ident, $top:ident, $side:ident, $cont:ident) => {
        ructe_page_res!($meta, $top, $side, $cont)
    };
}

// Rust 2018
// pub mod page_compositor;
// pub mod input;

pub struct Failure {}

impl From<std::string::FromUtf8Error> for Failure {
    fn from(_error: std::string::FromUtf8Error) -> Self {
        Failure {}
    }
}

impl From<std::io::Error> for Failure {
    fn from(_error: std::io::Error) -> Self {
        Failure {}
    }
}
