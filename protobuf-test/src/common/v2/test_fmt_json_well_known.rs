use protobuf::well_known_types::*;

use protobuf_test_common::*;

use super::test_fmt_json_well_known_pb::*;

#[test]
fn test_duration() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    let mut d = Duration::new();
    d.set_seconds(1);
    d.set_nanos(340012);
    m.set_duration(d);
    test_json_print_parse_message("{duration: \"1.000340012s\"}", &m);

    let mut m = TestFmtJsonWellKnownTypes::new();
    let mut d = Duration::new();
    d.set_seconds(1);
    m.set_duration(d);
    test_json_parse_message("{duration: \"1s\"}", &m);
}

#[test]
fn test_timestamp() {
    // TODO
}

#[test]
fn test_null_value() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.set_null_values(vec![NullValue::NULL_VALUE]);
    test_json_print_parse_message("{nullValues: [null]}", &m);
}

#[test]
fn test_value() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_value().set_bool_value(true);
    test_json_print_parse_message("{value: true}", &m);
    m.mut_value().set_number_value(12.0);
    test_json_print_parse_message("{value: 12.0}", &m);
    m.mut_value().set_string_value("ab".to_owned());
    test_json_print_parse_message("{value: \"ab\"}", &m);
    m.mut_value().set_null_value(NullValue::NULL_VALUE);
    test_json_print_parse_message("{value: null}", &m);
    // TODO: list
    // TODO: struct
}

#[test]
fn test_list_value() {
    // TODO
}

#[test]
fn test_struct() {
    // TODO
}

#[test]
fn test_wrappers() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_double_value().value = 10.0;
    test_json_print_parse_message("{doubleValue: 10.0}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_float_value().value = 12.0;
    test_json_print_parse_message("{floatValue: 12.0}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_int64_value().value = -13;
    test_json_print_parse_message("{int64Value: \"-13\"}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_uint64_value().value = 13;
    test_json_print_parse_message("{uint64Value: \"13\"}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_int32_value().value = -13;
    test_json_print_parse_message("{int32Value: -13}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_uint32_value().value = 13;
    test_json_print_parse_message("{uint32Value: 13}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_bool_value().value = true;
    test_json_print_parse_message("{boolValue: true}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_string_value().value = "ab".to_owned();
    test_json_print_parse_message("{stringValue: \"ab\"}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_bytes_value().value = b"ab".to_vec();
    test_json_print_parse_message("{bytesValue: \"YWI=\"}", &m);
}

#[test]
fn test_any() {
    // TODO
}

#[test]
fn test_field_mask() {
    // TODO
}