use super::*;
use crate as deboog;

#[test]
fn unit_struct() {
    #[derive(Deboog)]
    struct Unit;
    assert_eq!(format!("{:?}", Unit), "Unit");
}

#[test]
fn nested_tuple() {
    #[derive(Deboog)]
    struct Inner(i32);
    #[derive(Deboog)]
    struct Outer(Inner);
    let our = Outer(Inner(123));
    assert_eq!(format!("{:?}", our), "Outer(Inner(123))");
}

#[test]
fn nested_tuple_inner_skipped() {
    #[derive(Deboog)]
    struct Inner(#[deboog(skip)] i32);
    #[derive(Deboog)]
    struct Outer(Inner);
    let our = Outer(Inner(123));
    assert_eq!(format!("{:?}", our), "Outer(Inner)");
}

#[test]
fn nested_tuple_outer_skipped() {
    #[derive(Deboog)]
    struct Inner(i32);
    #[derive(Deboog)]
    struct Outer(#[deboog(skip)] Inner);
    let our = Outer(Inner(123));
    assert_eq!(format!("{:?}", our), "Outer");
}

#[test]
fn skip_struct_field() {
    #[allow(dead_code)]
    #[derive(Deboog)]
    struct Test {
        a: i32,
        #[deboog(skip)]
        b: i32,
    }
    let our = Test { a: 111, b: 222 };
    assert_eq!(format!("{:?}", our), "Test { a: 111 }");
}

#[test]
fn skip_tuple_field() {
    #[derive(Deboog)]
    struct Test(i32, #[deboog(skip)] i32);
    let our = Test(111, 222);
    assert_eq!(format!("{:?}", our), "Test(111)");
}

#[test]
fn skip_variant_struct_field() {
    #[allow(dead_code)]
    #[derive(Deboog)]
    enum Test {
        One,
        Two {
            a: i32,
            #[deboog(skip)]
            b: i32,
        },
    }
    let our = Test::One;
    assert_eq!(format!("{:?}", our), "One");
    let our = Test::Two { a: 111, b: 222 };
    assert_eq!(format!("{:?}", our), "Two { a: 111 }");
}

#[test]
fn skip_variant_tuple_field() {
    #[derive(Deboog)]
    enum Test {
        One,
        Two(i32, #[deboog(skip)] i32),
    }
    let our = Test::One;
    assert_eq!(format!("{:?}", our), "One");
    let our = Test::Two(111, 222);
    assert_eq!(format!("{:?}", our), "Two(111)");
}

#[test]
fn mask_pan_struct_field() {
    #[allow(dead_code)]
    #[derive(Deboog)]
    struct Test {
        a: &'static str,
        #[deboog(mask = "pan")]
        b: &'static str,
    }
    let our = Test {
        a: "0123456789012345",
        b: "0123456789012345",
    };
    assert_eq!(
        format!("{:?}", our),
        r#"Test { a: "0123456789012345", b: "012345******2345" }"#
    );
}

#[test]
fn mask_pan_suffix_struct_field() {
    #[allow(dead_code)]
    #[derive(Deboog)]
    struct Test {
        a: &'static str,
        #[deboog(mask = "pan_suffix")]
        b: &'static str,
    }
    let our = Test {
        a: "0123456789012345",
        b: "0123456789012345",
    };
    assert_eq!(
        format!("{:?}", our),
        r#"Test { a: "0123456789012345", b: "*2345" }"#
    );
}

#[test]
fn mask_all_suffix_struct_field() {
    #[allow(dead_code)]
    #[derive(Deboog)]
    struct Test {
        a: &'static str,
        #[deboog(mask = "all")]
        b: &'static str,
    }
    let our = Test {
        a: "0123456789012345",
        b: "0123456789012345",
    };
    assert_eq!(
        format!("{:?}", our),
        r#"Test { a: "0123456789012345", b: "****************" }"#
    );
}

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
