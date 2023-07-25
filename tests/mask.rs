use deboog::{Deboog, DeboogField};

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
fn mask_all_optional_struct_field() {
    #[allow(dead_code)]
    #[derive(Deboog)]
    struct Test {
        a: Option<&'static str>,
        #[deboog(mask = "all")]
        b: Option<&'static str>,
        c: Option<&'static str>,
    }
    let our = Test {
        a: Some("0123456789012345"),
        b: Some("0123456789012345"),
        c: None,
    };
    assert_eq!(
        format!("{:?}", our),
        r#"Test { a: Some("0123456789012345"), b: Some("****************"), c: None }"#
    );
}

#[test]
fn mask_all_vec_struct_field() {
    #[allow(dead_code)]
    #[derive(Deboog)]
    struct Test {
        #[deboog(mask = "all")]
        v: Vec<&'static str>,
    }
    let our = Test {
        v: vec!["12345", "23456", "345"],
    };
    assert_eq!(
        format!("{:?}", our),
        r#"Test { v: ["*****", "*****", "***"] }"#
    );
}

#[test]
fn mask_all_vec_optional_struct_field() {
    #[allow(dead_code)]
    #[derive(Deboog)]
    struct Test {
        #[deboog(mask = "all")]
        v: Vec<Option<&'static str>>,
    }
    let our = Test {
        v: vec![Some("12345"), Some("234"), None],
    };
    assert_eq!(
        format!("{:?}", our),
        r#"Test { v: [Some("*****"), Some("***"), None] }"#
    );
}

#[test]
fn mask_all_default_impl() {
    struct Item;

    impl DeboogField for Item {}

    #[allow(dead_code)]
    #[derive(Deboog)]
    struct Test {
        #[deboog(mask = "all")]
        item: Item,
    }
    let our = Test { item: Item };
    assert_eq!(format!("{:?}", our), r#"Test { item: *** }"#);
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
        r#"Test { a: "0123456789012345", b: *** }"#
    );
}
