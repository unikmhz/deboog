use deboog::Deboog;

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
fn mask_all_struct_field() {
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
fn mask_hidden_struct_field() {
    #[allow(dead_code)]
    #[derive(Deboog)]
    struct Test {
        a: &'static str,
        #[deboog(mask = "hidden")]
        b: &'static str,
    }
    let our = Test {
        a: "0123456789012345",
        b: "0123456789012345",
    };
    assert_eq!(
        format!("{:?}", our),
        r#"Test { a: "0123456789012345", b: "***" }"#
    );
}
