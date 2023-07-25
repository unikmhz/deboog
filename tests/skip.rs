use deboog::Deboog;

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
