#[macro_export]
macro_rules! avec {
    ($($element:expr),*) => {{
        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity($crate::avec![@COUNT; $($element),*]);
        $(vs.push($element);)*
        vs
    }};
    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        vs.resize($count, $element);
        vs
    }};

    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::avec![@SUBST; $element]),*])
    };
    (@SUBST; $element:expr) => { () };
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn clone() {
    let x: Vec<u32> = avec![42; 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}

#[test]
fn clone_2_nonliteral() {
    let mut y = Some(2);
    let x: Vec<u32> = avec![y.take().unwrap(); 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 2);
    assert_eq!(x[1], 2);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42, 43];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn trailing() {
    let _: Vec<&str> = avec![
        "ghjklj;sfgdhdjfskhdjghshfklaajfljdjfsdkfksdjflj",
        "ghjklj;sfgdhdjfskhdjghshfklaajfljdjfsdkfksdjflj",
        "ghjklj;sfgdhdjfskhdjghshfklaajfljdjfsdkfksdjflj",
        "ghjklj;sfgdhdjfskhdjghshfklaajfljdjfsdkfksdjflj",
    ];
}
