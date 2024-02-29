#![allow(unused_imports)]

pub fn xor_runt(bytes: &[u8]) -> Vec<u8> {
    let key = [0xab,0xc0,0xab,0xc0,0xab,0xc0,0xab,0xc0];
    let mut out = vec![];
    let mut b = 0;
    while b < bytes.len() {
        out.push(bytes[b] ^ key[b % key.len()]);
        b += 1;
    }

    out
}


pub mod logger {
    #[macro_export]
    macro_rules! dbg_info {
    ($($x:tt)*) => {
        {
            #[cfg(debug_assertions)]
            {
                info!($($x)*)
            }
            #[cfg(not(debug_assertions))]
            {

            }
        }
    }
    }

    #[macro_export]
    macro_rules! dbg_error {
    ($($x:tt)*) => {
        {
            #[cfg(debug_assertions)]
            {
                error!($($x)*)
            }
            #[cfg(not(debug_assertions))]
            {

            }
        }
    }
}
    pub use dbg_error;
    pub use dbg_info;
}

pub mod jni_helpers {
    #[macro_export]
    macro_rules! check_java_exception {
        ($e:expr) => {
            let res = $e.exception_check();
            match res {
                Ok(res) => {
                    if (res) {
                        dbg_info!("exception check: {}", res);
                        let _ = $e.exception_describe();
                        let _ = $e.exception_clear();
                    }
                }
                Err(e) => {
                    dbg_error!("exception check error: {}", e);
                }
            }
        };
    }

    #[macro_export]
    macro_rules! checked_find_class {
        ($env:expr, $cls: expr) => {{
            let cls = $env.find_class($cls);

            match cls {
                Ok(c) => Ok(c),
                Err(e) => {
                    dbg_error!("Error finding class: {} {}", $cls, e);
                    check_java_exception!($env);
                    Err(e)
                }
            }
        }};

        ($env:expr, $cls: expr, $ret: expr) => {{
            let cls = $env.find_class($cls);

            match cls {
                Ok(c) => c,
                Err(e) => {
                    dbg_info!("Error finding class: {} {}", $cls, e);
                    check_java_exception!($env);
                    return $ret;
                }
            }
        }};
    }

    #[macro_export]
    macro_rules! checked_new_object {
        ($env:expr, $cls: expr, $params: expr, $args:expr) => {{
            let constr = $env.new_object($cls, $params, $args);

            match constr {
                Ok(c) => Ok(c),
                Err(e) => {
                    dbg_error!("Error new_object class: {} {}", $params, e);
                    check_java_exception!($env);
                    Err(e)
                }
            }
        }};

        ($env:expr, $cls: expr, $params: expr, $args:expr, $ret:expr) => {{
            let constr = $env.new_object($cls, $params, $args);

            match constr {
                Ok(c) => c,
                Err(e) => {
                    dbg_error!("Error new_object class: {} {}", $params, e);
                    check_java_exception!($env);
                    return $ret;
                }
            }
        }};
    }

    #[macro_export]
    macro_rules! checked_new_global_ref {
        ($env:expr, $obj: expr) => {{
            let constr = $env.new_global_ref($obj);

            match constr {
                Ok(c) => Ok(c),
                Err(e) => {
                    dbg_error!("Error new_global_ref class: {:?} {}", $obj, e);
                    check_java_exception!($env);
                    Err(e)
                }
            }
        }};

        ($env:expr, $obj: expr, $ret:expr) => {{
            let constr = $env.new_global_ref($obj);

            match constr {
                Ok(c) => c,
                Err(e) => {
                    dbg_error!("Error new_global_ref class: {:?} {}", $obj, e);
                    check_java_exception!($env);
                    return $ret;
                }
            }
        }};
    }

    #[macro_export]
    macro_rules! checked_call_method {
        ($env:expr, $cls: expr, $f: expr, $sig: expr, $args:expr) => {{
            let mtd = $env.call_method(&$cls, $f, $sig, $args);

            match mtd {
                Ok(c) => Ok(c),
                Err(e) => {
                    dbg_error!(
                        "Error finding method: {} {} {} {}",
                        stringify!($cls),
                        $f,
                        $sig,
                        e
                    );
                    check_java_exception!($env);
                    Err(e)
                }
            }
        }};

        ($env:expr, $cls: expr, $f: expr, $sig: expr, $args:expr, $ret:expr) => {{
            let mtd = $env.call_method(&$cls, $f, $sig, $args);

            match mtd {
                Ok(c) => c,
                Err(e) => {
                    dbg_error!(
                        "Error finding method: {} {} {} {}",
                        stringify!($cls),
                        $f,
                        $sig,
                        e
                    );
                    check_java_exception!($env);
                    return $ret;
                }
            }
        }};
    }

    #[macro_export]
    macro_rules! checked_new_object_array {
        ($env:expr, $length: expr, $cls: expr, $init: expr) => {{
            let arr = $env.new_object_array($length, $cls, $init);

            match arr {
                Ok(c) => Ok(c),
                Err(e) => {
                    dbg_info!("Error finding class: {}", e);
                    check_java_exception!($env);
                    Err(e)
                }
            }
        }};

        ($env:expr, $length: expr, $cls: expr, $init: expr, $ret: expr) => {{
            let arr = $env.new_object_array($length, $cls, $init);

            match arr {
                Ok(c) => c,
                Err(e) => {
                    dbg_info!("Error finding class: {}", e);
                    check_java_exception!($env);
                    return $ret;
                }
            }
        }};
    }

    #[macro_export]
    macro_rules! checked_new_string {
        ($env:expr, $str:expr) => {{
            let s = $env.new_string($str);

            match s {
                Ok(c) => Ok(c),
                Err(e) => {
                    dbg_info!("Error new_string : {}", e);
                    check_java_exception!($env);
                    Err(e)
                }
            }
        }};

        ($env:expr, $str:expr, $ret:expr) => {{
            let s = $env.new_string($str);

            match s {
                Ok(c) => c,
                Err(e) => {
                    dbg_info!("Error new_string: {}", e);
                    check_java_exception!($env);
                    return $ret;
                }
            }
        }};
    }

    #[macro_export]
    macro_rules! checked_call_static_method {
        ($env:expr, $cls: expr, $f: expr, $sig: expr, $args:expr) => {{
            let mtd = $env.call_static_method($cls, $f, $sig, $args);

            match mtd {
                Ok(c) => Ok(c),
                Err(e) => {
                    dbg_info!("Error finding class: {}", e);
                    check_java_exception!($env);
                    Err(e)
                }
            }
        }};

        ($env:expr, &$cls: expr, $f: expr, $sig: expr, $args:expr, $ret: expr) => {{
            let mtd = $env.call_static_method($cls, $f, $sig, $args);

            match mtd {
                Ok(c) => c,
                Err(e) => {
                    dbg_info!("Error finding class: {}", e);
                    check_java_exception!($env);
                    return $ret;
                }
            }
        }};
    }

    #[macro_export]
    macro_rules! checked_get_array_length {
        ($env:expr, $arr:expr) => {{
            let l = $env.get_array_length($arr);

            match l {
                Ok(c) => Ok(c),
                Err(e) => {
                    dbg_info!("Error get_array_length: {}", e);
                    check_java_exception!($env);
                    Err(e)
                }
            }
        }};

        ($env:expr, $arr:expr, $ret:expr) => {{
            let l = $env.get_array_length($arr);

            match l {
                Ok(c) => c,
                Err(e) => {
                    dbg_info!("Error get_array_length: {}", e);
                    check_java_exception!($env);
                    return $ret;
                }
            }
        }};
    }

    #[macro_export]
    macro_rules! checked_get_object_array_element {
        ($env:expr, $arr:expr, $i: expr) => {{
            let l = $env.get_object_array_element($arr, $i);

            match l {
                Ok(c) => Ok(c),
                Err(e) => {
                    dbg_info!("Error get_object_array_element: {}", e);
                    check_java_exception!($env);
                    Err(e)
                }
            }
        }};

        ($env:expr, $arr:expr, $i:expr, $ret:expr) => {{
            let l = $env.get_object_array_element($arr, $i);

            match l {
                Ok(c) => c,
                Err(e) => {
                    dbg_info!("Error get_object_array_element: {}", e);
                    check_java_exception!($env);
                    return $ret;
                }
            }
        }};
    }
    #[macro_export]
    macro_rules! checked_get_string {
        ($env:expr, $s:expr ) => {{
            let l = $env.get_string($s);

            match l {
                Ok(c) => Ok(c),
                Err(e) => {
                    dbg_info!("Error get_string: {}", e);
                    check_java_exception!($env);
                    Err(e)
                }
            }
        }};

        ($env:expr, $s:expr, $ret: expr) => {{
            let l = $env.get_string($s);

            match l {
                Ok(c) => c,
                Err(e) => {
                    dbg_info!("Error get_string: {}", e);
                    check_java_exception!($env);
                    return $ret;
                }
            }
        }};
    }

    pub use check_java_exception;
    pub use checked_call_method;
    pub use checked_call_static_method;
    pub use checked_find_class;
    pub use checked_get_array_length;
    pub use checked_get_object_array_element;
    pub use checked_get_string;
    pub use checked_new_global_ref;
    pub use checked_new_object;
    pub use checked_new_object_array;
    pub use checked_new_string;
}
