use deboog::Deboog;

#[test]
fn eq_unit_struct() {
    let standard = {
        #[derive(Debug)]
        struct Unit;
        Unit
    };
    let our = {
        #[derive(Deboog)]
        struct Unit;
        Unit
    };
    assert_eq!(format!("{:?}", standard), format!("{:?}", our));
}

#[test]
fn eq_normal_struct() {
    let standard = {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct Normal {
            a: i32,
            b: &'static str,
        }
        Normal { a: 123, b: "test" }
    };
    let our = {
        #[allow(dead_code)]
        #[derive(Deboog)]
        struct Normal {
            a: i32,
            b: &'static str,
        }
        Normal { a: 123, b: "test" }
    };
    assert_eq!(format!("{:?}", standard), format!("{:?}", our));
}

#[test]
fn eq_tuple_struct() {
    let standard = {
        #[derive(Debug)]
        struct Tuple(i32, &'static str);
        Tuple(123, "test")
    };
    let our = {
        #[derive(Deboog)]
        struct Tuple(i32, &'static str);
        Tuple(123, "test")
    };
    assert_eq!(format!("{:?}", standard), format!("{:?}", our));
}

#[test]
fn eq_enum() {
    let standard = {
        #[derive(Debug)]
        enum Basic {
            Variant,
        }
        Basic::Variant
    };
    let our = {
        #[derive(Deboog)]
        enum Basic {
            Variant,
        }
        Basic::Variant
    };
    assert_eq!(format!("{:?}", standard), format!("{:?}", our));
}

#[test]
fn eq_enum_discr() {
    let standard = {
        #[derive(Debug)]
        enum Discr {
            Variant = 20,
        }
        Discr::Variant
    };
    let our = {
        #[derive(Deboog)]
        enum Discr {
            Variant = 20,
        }
        Discr::Variant
    };
    assert_eq!(format!("{:?}", standard), format!("{:?}", our));
}

#[test]
fn eq_enum_w_fields() {
    let standard = {
        #[allow(dead_code)]
        #[derive(Debug)]
        enum With {
            Fields { a: i32, b: &'static str },
        }
        With::Fields { a: 123, b: "test" }
    };
    let our = {
        #[allow(dead_code)]
        #[derive(Deboog)]
        enum With {
            Fields { a: i32, b: &'static str },
        }
        With::Fields { a: 123, b: "test" }
    };
    assert_eq!(format!("{:?}", standard), format!("{:?}", our));
}

#[test]
fn eq_enum_w_tuple() {
    let standard = {
        #[derive(Debug)]
        enum With {
            Tuple(i32, &'static str),
        }
        With::Tuple(123, "test")
    };
    let our = {
        #[derive(Deboog)]
        enum With {
            Tuple(i32, &'static str),
        }
        With::Tuple(123, "test")
    };
    assert_eq!(format!("{:?}", standard), format!("{:?}", our));
}
