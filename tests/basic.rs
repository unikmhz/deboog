use deboog::Deboog;

#[test]
fn unit_struct() {
    #[derive(Deboog)]
    struct Unit;
    assert_eq!(format!("{:?}", Unit), r#"Unit"#);
}

#[test]
fn tuple_struct() {
    #[derive(Deboog)]
    struct Tuple(i32, &'static str);
    let our = Tuple(123, "test");
    assert_eq!(format!("{:?}", our), r#"Tuple(123, "test")"#);
}

#[test]
fn nested_tuple_struct() {
    #[derive(Deboog)]
    struct Inner(i32);
    #[derive(Deboog)]
    struct Outer(Inner);
    let our = Outer(Inner(123));
    assert_eq!(format!("{:?}", our), r#"Outer(Inner(123))"#);
}

#[test]
fn normal_struct() {
    #[allow(dead_code)]
    #[derive(Deboog)]
    struct Struct {
        a: i32,
        b: &'static str,
    }
    let our = Struct { a: 123, b: "test" };
    assert_eq!(format!("{:?}", our), r#"Struct { a: 123, b: "test" }"#);
}

#[test]
fn nested_normal_struct() {
    #[allow(dead_code)]
    #[derive(Deboog)]
    struct Inner {
        a: i32,
        b: &'static str,
    }
    #[allow(dead_code)]
    #[derive(Deboog)]
    struct Outer {
        inner: Inner,
    }
    let inner = Inner { a: 123, b: "test" };
    let our = Outer { inner };
    assert_eq!(
        format!("{:?}", our),
        r#"Outer { inner: Inner { a: 123, b: "test" } }"#
    );
}
