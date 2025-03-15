use crate::utils::on_drop::on_drop;

#[test]
fn drop() {
    let mut x = false;
    {
        let _v = on_drop(|| x = true);
    }
    assert!(x);
}

#[test]
fn forget() {
    let mut x = false;
    {
        let v = on_drop(|| x = true);
        v.forget();
    }
    assert!(!x);
}

#[test]
fn memory_safety() {
    let mut b = Box::new(0);
    {
        let _v = on_drop(|| {
            *b += 1;
        });
    }
    assert_eq!(*b, 1);
    {
        let v = on_drop(|| {
            *b += 1;
        });
        v.forget();
    }
    assert_eq!(*b, 1);
    {
        let _v = on_drop(move || {
            *b += 1;
        });
    }
    let mut b = Box::new(0);
    {
        let v = on_drop(move || {
            *b += 1;
        });
        v.forget();
    }
}

#[test]
fn memory_safety2() {
    let mut b = Box::new(0);
    {
        let _v = on_drop(move || {
            *b += 1;
        });
    }
    let mut b = Box::new(0);
    {
        let v = on_drop(move || {
            *b += 1;
        });
        v.forget();
    }
}
