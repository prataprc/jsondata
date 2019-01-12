use json::Json;
use property::Property;

#[test]
fn test_ops_add() {
    // Null as lhs
    assert_eq!(Json::Null, Json::Null + Json::Null);
    let refval: Json = true.into();
    assert_eq!(refval, Json::Null + true.into());
    let refval: Json = false.into();
    assert_eq!(refval, Json::Null + false.into());
    let refval: Json = 10.into();
    assert_eq!(refval, Json::Null + 10.into());
    let refval: Json = 10.2.into();
    assert_eq!(refval, Json::Null + 10.2.into());
    let refval: Json = "hello".into();
    assert_eq!(refval, Json::Null + "hello".into());
    let rhs: Json = vec![Json::new(1), 2.into()].into();
    let refval: Json = vec![Json::new(1), 2.into()].into();
    assert_eq!(refval, Json::Null + rhs);
    let rhs: Json = vec![Property::new("a", 10.into())].into();
    let refval: Json = vec![Property::new("a", 10.into())].into();
    assert_eq!(refval, Json::Null + rhs);

    // Null as rhs
    assert_eq!(Json::Null, Json::Null + Json::Null);
    let refval: Json = true.into();
    assert_eq!(refval, Json::new(true) + Json::Null);
    let refval: Json = false.into();
    assert_eq!(refval, Json::new(false) + Json::Null);
    let refval: Json = 10.into();
    assert_eq!(refval, Json::new(10) + Json::Null);
    let refval: Json = 10.2.into();
    assert_eq!(refval, Json::new(10.2) + Json::Null);
    let refval: Json = "hello".into();
    assert_eq!(refval, Json::new("hello") + Json::Null);
    let lhs: Json = vec![Json::new(1), 2.into()].into();
    let refval: Json = vec![Json::new(1), 2.into()].into();
    assert_eq!(refval, lhs + Json::Null);
    let lhs: Json = vec![Property::new("a", 10.into())].into();
    let refval: Json = vec![Property::new("a", 10.into())].into();
    assert_eq!(refval, lhs + Json::Null);

    // Integers and floats
    assert_eq!(Json::new(20), Json::new(10) + 10.into());
    assert_eq!(Json::new(20.2), Json::new(10.1) + 10.1.into());
    assert_eq!(Json::new(20.2), Json::new(10) + 10.2.into());
    assert_eq!(Json::new(20.2), Json::new(10.2) + 10.into());

    // String addition
    assert_eq!(Json::new("helloworld"), Json::new("hello") + "world".into());

    // Array addition
    let lhs: Json = vec![Json::new(1), 2.into()].into();
    let rhs: Json = vec![Json::new(2), 1.into()].into();
    let refval: Json = vec![Json::new(1), 2.into(), 2.into(), 1.into()].into();
    assert_eq!(refval, lhs + rhs);

    // Object addition
    let lhs: Json = vec![Property::new("a", 10.into()), Property::new("b", 11.into())].into();
    let rhs: Json = vec![Property::new("b", 20.into())].into();
    let refval: Json = vec![Property::new("a", 10.into()), Property::new("b", 20.into())].into();
    assert_eq!(refval, lhs + rhs);
}

#[test]
#[allow(clippy::eq_op)]
fn test_ops_sub() {
    // Null as lhs
    assert_eq!(Json::Null, Json::Null - Json::Null);
    let refval: Json = true.into();
    assert_eq!(refval, Json::Null - true.into());
    let refval: Json = false.into();
    assert_eq!(refval, Json::Null - false.into());
    let refval: Json = 10.into();
    assert_eq!(refval, Json::Null - 10.into());
    let refval: Json = 10.2.into();
    assert_eq!(refval, Json::Null - 10.2.into());
    let refval: Json = "hello".into();
    assert_eq!(refval, Json::Null - "hello".into());
    let rhs: Json = vec![Json::new(1), 2.into()].into();
    let refval: Json = vec![Json::new(1), 2.into()].into();
    assert_eq!(refval, Json::Null - rhs);
    let rhs: Json = vec![Property::new("a", 10.into())].into();
    let refval: Json = vec![Property::new("a", 10.into())].into();
    assert_eq!(refval, Json::Null - rhs);

    // Null as rhs
    assert_eq!(Json::Null, Json::Null - Json::Null);
    let refval: Json = true.into();
    assert_eq!(refval, Json::new(true) - Json::Null);
    let refval: Json = false.into();
    assert_eq!(refval, Json::new(false) - Json::Null);
    let refval: Json = 10.into();
    assert_eq!(refval, Json::new(10) - Json::Null);
    let refval: Json = 10.2.into();
    assert_eq!(refval, Json::new(10.2) - Json::Null);
    let refval: Json = "hello".into();
    assert_eq!(refval, Json::new("hello") - Json::Null);
    let lhs: Json = vec![Json::new(1), 2.into()].into();
    let refval: Json = vec![Json::new(1), 2.into()].into();
    assert_eq!(refval, lhs - Json::Null);
    let lhs: Json = vec![Property::new("a", 10.into())].into();
    let refval: Json = vec![Property::new("a", 10.into())].into();
    assert_eq!(refval, lhs - Json::Null);

    // Integers and floats
    assert_eq!(Json::new(10), Json::new(20) - 10.into());
    assert_eq!(Json::new(20.1 - 10.1), Json::new(20.1) - 10.1.into());
    assert_eq!(Json::new(9.8), Json::new(20) - 10.2.into());
    assert_eq!(
        Json::new(10.2 - (f64::from(10))),
        Json::new(10.2) - 10.into()
    );

    // Array substraction
    let lhs: Json = vec![Json::new(1), 1.into(), 2.into(), 2.into(), 2.into()].into();
    let rhs: Json = vec![Json::new(2), 2.into(), 1.into()].into();
    let refval: Json = vec![Json::new(1), 2.into()].into();
    assert_eq!(refval, lhs - rhs);

    // Object substraction
    let lhs: Json = vec![Property::new("a", 10.into()), Property::new("b", 20.into())].into();
    let rhs: Json = vec![Property::new("b", 20.into())].into();
    let refval: Json = vec![Property::new("a", 10.into())].into();
    assert_eq!(refval, lhs - rhs);
}

#[test]
fn test_ops_mul() {
    // Null as lhs
    assert_eq!(Json::Null, Json::Null * Json::Null);
    assert_eq!(Json::Null, Json::Null * true.into());
    assert_eq!(Json::Null, Json::Null * false.into());
    assert_eq!(Json::Null, Json::Null * 10.into());
    assert_eq!(Json::Null, Json::Null * 10.2.into());
    assert_eq!(Json::Null, Json::Null * "hello".into());
    let rhs: Json = vec![Json::new(1), 2.into()].into();
    assert_eq!(Json::Null, Json::Null * rhs);
    let rhs: Json = vec![Property::new("a", 10.into())].into();
    assert_eq!(Json::Null, Json::Null * rhs);

    // Null as rhs
    assert_eq!(Json::Null, Json::Null * Json::Null);
    assert_eq!(Json::Null, Json::new(true) * Json::Null);
    assert_eq!(Json::Null, Json::new(false) * Json::Null);
    assert_eq!(Json::Null, Json::new(10) * Json::Null);
    assert_eq!(Json::Null, Json::new(10.2) * Json::Null);
    assert_eq!(Json::Null, Json::new("hello") * Json::Null);
    let lhs: Json = vec![Json::new(1), 2.into()].into();
    assert_eq!(Json::Null, lhs * Json::Null);
    let lhs: Json = vec![Property::new("a", 10.into())].into();
    assert_eq!(Json::Null, lhs * Json::Null);

    // Integers and floats
    assert_eq!(Json::new(200), Json::new(20) * 10.into());
    assert_eq!(Json::new(20.1 * 10.1), Json::new(20.1) * 10.1.into());
    assert_eq!(
        Json::new((f64::from(20)) * 10.2),
        Json::new(20) * 10.2.into()
    );
    assert_eq!(
        Json::new(10.2 * (f64::from(10))),
        Json::new(10.2) * 10.into()
    );

    // String multiplication
    assert_eq!(Json::new("okokok"), Json::new("ok") * 3.into());
    assert_eq!(Json::Null, Json::new("ok") * 0.into());
    assert_eq!(Json::new("okokok"), Json::new(3) * "ok".into());
    assert_eq!(Json::Null, Json::new(0) * "ok".into());
}

#[test]
#[allow(clippy::eq_op)]
fn test_ops_div() {
    // Null as lhs
    assert_eq!(Json::Null, Json::Null / Json::Null);
    assert_eq!(Json::Null, Json::Null / true.into());
    assert_eq!(Json::Null, Json::Null / false.into());
    assert_eq!(Json::Null, Json::Null / 10.into());
    assert_eq!(Json::Null, Json::Null / 10.2.into());
    assert_eq!(Json::Null, Json::Null / "hello".into());
    let rhs: Json = vec![Json::new(1), 2.into()].into();
    assert_eq!(Json::Null, Json::Null / rhs);
    let rhs: Json = vec![Property::new("a", 10.into())].into();
    assert_eq!(Json::Null, Json::Null / rhs);

    // Null as rhs
    assert_eq!(Json::Null, Json::Null / Json::Null);
    assert_eq!(Json::Null, Json::new(true) / Json::Null);
    assert_eq!(Json::Null, Json::new(false) / Json::Null);
    assert_eq!(Json::Null, Json::new(10) / Json::Null);
    assert_eq!(Json::Null, Json::new(10.2) / Json::Null);
    assert_eq!(Json::Null, Json::new("hello") / Json::Null);
    let lhs: Json = vec![Json::new(1), 2.into()].into();
    assert_eq!(Json::Null, lhs / Json::Null);
    let lhs: Json = vec![Property::new("a", 10.into())].into();
    assert_eq!(Json::Null, lhs / Json::Null);

    // Integers and floats
    assert_eq!(Json::new(2), Json::new(20) / 10.into());
    assert_eq!(Json::new(20.1 / 10.1), Json::new(20.1) / 10.1.into());
    assert_eq!(
        Json::new((f64::from(20)) / 10.2),
        Json::new(20) / 10.2.into()
    );
    assert_eq!(
        Json::new(10.2 / (f64::from(10))),
        Json::new(10.2) / 10.into()
    );
}

#[test]
fn test_ops_rem() {
    // Null as lhs
    assert_eq!(Json::Null, Json::Null % Json::Null);
    assert_eq!(Json::Null, Json::Null % true.into());
    assert_eq!(Json::Null, Json::Null % false.into());
    assert_eq!(Json::Null, Json::Null % 10.into());
    assert_eq!(Json::Null, Json::Null % 10.2.into());
    assert_eq!(Json::Null, Json::Null % "hello".into());
    let rhs: Json = vec![Json::new(1), 2.into()].into();
    assert_eq!(Json::Null, Json::Null % rhs);
    let rhs: Json = vec![Property::new("a", 10.into())].into();
    assert_eq!(Json::Null, Json::Null % rhs);

    // Null as rhs
    assert_eq!(Json::Null, Json::Null % Json::Null);
    assert_eq!(Json::Null, Json::new(true) % Json::Null);
    assert_eq!(Json::Null, Json::new(false) % Json::Null);
    assert_eq!(Json::Null, Json::new(10) % Json::Null);
    assert_eq!(Json::Null, Json::new(10.2) % Json::Null);
    assert_eq!(Json::Null, Json::new("hello") % Json::Null);
    let lhs: Json = vec![Json::new(1), 2.into()].into();
    assert_eq!(Json::Null, lhs % Json::Null);
    let lhs: Json = vec![Property::new("a", 10.into())].into();
    assert_eq!(Json::Null, lhs % Json::Null);

    // Integers and floats
    assert_eq!(Json::new(2), Json::new(202) % 10.into());
    assert_eq!(Json::new(20.1 % 10.1), Json::new(20.1) % 10.1.into());
    assert_eq!(
        Json::new((f64::from(20)) % 10.2),
        Json::new(20) % 10.2.into()
    );
    assert_eq!(
        Json::new(10.2 % (f64::from(10))),
        Json::new(10.2) % 10.into()
    );
}

#[test]
fn test_ops_neg() {
    // Null as lhs
    assert_eq!(Json::Null, -Json::Null);

    // Integers and floats
    assert_eq!(Json::new(-202), -Json::new(202));
    assert_eq!(Json::new(-20.1), -Json::new(20.1));
}

#[test]
fn test_ops_shl() {
    assert_eq!(Json::new(2), Json::new(1) << 1.into());
    let v = -170_141_183_460_469_231_731_687_303_715_884_105_728;
    assert_eq!(Json::new(v), Json::new(1) << 127.into());
}

#[test]
fn test_ops_shr() {
    assert_eq!(Json::new(0), Json::new(1) >> 1.into());
    assert_eq!(Json::new(-1), (Json::new(1) << 127.into()) >> 127.into());
}

#[test]
fn test_ops_bitand() {
    assert_eq!(Json::new(0xABCD), Json::new(0xABCD) & 0xFFFF.into());
    assert_eq!(Json::new(0), Json::new(0xABCD) & 0.into());
}

#[test]
fn test_ops_bitor() {
    assert_eq!(Json::new(0xFFFF), Json::new(0xABCD) | 0xFFFF.into());
    assert_eq!(Json::new(0xABCD), Json::new(0xABCD) | 0.into());
}

#[test]
fn test_ops_bitxor() {
    assert_eq!(Json::new(0x5432), Json::new(0xABCD) ^ 0xFFFF.into());
    assert_eq!(Json::new(0xABCD), Json::new(0xABCD) ^ 0.into());
}

#[test]
fn test_ops_and() {
    assert_eq!(Json::new(true), Json::new(true) & true.into());
    assert_eq!(Json::new(false), Json::new(false) & true.into());
    assert_eq!(Json::new(false), Json::new(true) & false.into());
    assert_eq!(Json::new(false), Json::new(false) & false.into());
}

#[test]
fn test_ops_or() {
    assert_eq!(Json::new(true), Json::new(true) | true.into());
    assert_eq!(Json::new(true), Json::new(false) | true.into());
    assert_eq!(Json::new(true), Json::new(true) | false.into());
    assert_eq!(Json::new(false), Json::new(false) | false.into());
}

#[test]
fn test_ops_xor() {
    assert_eq!(Json::new(false), Json::new(true) ^ true.into());
    assert_eq!(Json::new(true), Json::new(false) ^ true.into());
    assert_eq!(Json::new(true), Json::new(true) ^ false.into());
    assert_eq!(Json::new(false), Json::new(false) ^ false.into());
}

#[test]
fn test_index_arr() {
    let item: Json = vec![Json::new(1), 2.into()].into();
    let value: Json = vec![
        Json::new(1), 2.into(), true.into(), Json::Null, 3.4.into(), item.clone()
    ].into();

    assert_eq!(value[0], Json::new(1));
    assert_eq!(value[1], Json::new(2));
    assert_eq!(value[5], item);
    assert_eq!(value[-1], item);
    assert_eq!(value[-2], Json::new(3.4));
    assert_eq!(value[-6], Json::new(1));

    assert!(value[-7].is_error());
    assert!(value[6].is_error());
}

#[test]
fn test_index_obj() {
    let value: Json = vec![
        Property::new("a", 10.into()), Property::new("b", 10.into()), Property::new("c", 10.into()),
    ].into();
    assert_eq!(value["a"], Json::new(10));
    assert!(value["z"].is_error());
}

#[test]
fn test_range_arr() {
    let arr: Vec<Json> = vec![ Json::new(1), 2.into(), true.into(), Json::Null, 3.4.into(), 6.into() ].into();
    let value: Json = arr.clone().into();

    assert_eq!(value.range(1..), Json::new(arr[1..].to_vec()));
    assert_eq!(value.range(1..3), Json::new(arr[1..3].to_vec()));
    assert_eq!(value.range(..3), Json::new(arr[..3].to_vec()));
    assert_eq!(value.range(..), Json::new(arr[..].to_vec()));
    assert_eq!(value.range(1..=3), Json::new(arr[1..=3].to_vec()));
    assert_eq!(value.range(..=3), Json::new(arr[..=3].to_vec()));
}
