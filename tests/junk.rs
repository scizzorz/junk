use junk::*;

fn ok(input: &str) -> String {
    parse_junk(input).expect("parse failed")
}

fn err(input: &str) {
    assert!(parse_junk(input).is_err(), "expected parse error for: {input:?}");
}

// root is an object

#[test]
fn root_object_single_value_def() {
    assert_eq!(ok("x: 42"), r#"{"x": 42}"#);
}

#[test]
fn root_object_multiple_defs() {
    assert_eq!(ok("x: 1\ny: 2\nz: 3"), r#"{"x": 1, "y": 2, "z": 3}"#);
}

#[test]
fn root_object_pos_flag() {
    assert_eq!(ok("active"), r#"{"active": true}"#);
}

#[test]
fn root_object_neg_flag() {
    assert_eq!(ok("!disabled"), r#"{"disabled": false}"#);
}

#[test]
fn root_object_mixed_flags_and_defs() {
    assert_eq!(
        ok("active\n!disabled\ncount: 5"),
        r#"{"active": true, "disabled": false, "count": 5}"#,
    );
}

#[test]
fn root_object_blank_lines_ignored() {
    assert_eq!(ok("x: 1\n\n\ny: 2"), r#"{"x": 1, "y": 2}"#);
}

#[test]
fn root_object_leading_and_trailing_newlines() {
    assert_eq!(ok("\n\nx: 1\n\n"), r#"{"x": 1}"#);
}

#[test]
fn root_object_comment_ignored() {
    assert_eq!(ok("// a comment\nx: 1"), r#"{"x": 1}"#);
}

#[test]
fn root_object_inline_comment() {
    assert_eq!(ok("x: 1 // set x\ny: 2"), r#"{"x": 1, "y": 2}"#);
}

// root is a list

#[test]
fn root_list_empty() {
    assert_eq!(ok(""), "[]");
}

#[test]
fn root_list_single_object_with_id() {
    assert_eq!(ok("#foo {x: 1}"), r#"[{"id": "foo", "x": 1}]"#);
}

#[test]
fn root_list_multiple_objects() {
    assert_eq!(
        ok("#foo {x: 1}\n#bar {y: 2}"),
        r#"[{"id": "foo", "x": 1}, {"id": "bar", "y": 2}]"#,
    );
}

#[test]
fn root_list_object_without_id() {
    assert_eq!(ok("{x: 1}"), r#"[{"x": 1}]"#);
}

#[test]
fn root_list_string_literals() {
    assert_eq!(ok("\"hello\"\n\"world\""), r#"["hello", "world"]"#);
}

#[test]
fn root_list_nested_list() {
    assert_eq!(ok("[1, 2, 3]"), "[[1, 2, 3]]");
}

// int values

#[test]
fn int_zero() {
    assert_eq!(ok("x: 0"), r#"{"x": 0}"#);
}

#[test]
fn int_positive() {
    assert_eq!(ok("x: 42"), r#"{"x": 42}"#);
}

#[test]
fn int_negative() {
    assert_eq!(ok("x: -7"), r#"{"x": -7}"#);
}

#[test]
fn int_large() {
    assert_eq!(ok("x: 1000000"), r#"{"x": 1000000}"#);
}

// float values

#[test]
fn float_positive() {
    assert_eq!(ok("x: 3.14"), r#"{"x": 3.14}"#);
}

#[test]
fn float_negative() {
    assert_eq!(ok("x: -2.5"), r#"{"x": -2.5}"#);
}

#[test]
fn float_less_than_one() {
    assert_eq!(ok("x: 0.5"), r#"{"x": 0.5}"#);
}

#[test]
fn float_whole_number_drops_decimal() {
    // Rust's f64 Display omits the trailing ".0": 1.0 -> "1"
    assert_eq!(ok("x: 1.0"), r#"{"x": 1}"#);
}

// bool values

#[test]
fn bool_true_literal() {
    assert_eq!(ok("x: true"), r#"{"x": true}"#);
}

#[test]
fn bool_false_literal() {
    assert_eq!(ok("x: false"), r#"{"x": false}"#);
}

// string values

#[test]
fn str_plain() {
    assert_eq!(ok(r#"x: "hello""#), r#"{"x": "hello"}"#);
}

#[test]
fn str_empty() {
    assert_eq!(ok(r#"x: """#), r#"{"x": ""}"#);
}

#[test]
fn str_with_spaces() {
    assert_eq!(ok(r#"x: "hello world""#), r#"{"x": "hello world"}"#);
}

#[test]
fn str_escape_newline() {
    // \n in source → actual newline in value → re-escaped as \n in JSON output
    assert_eq!(ok(r#"x: "line1\nline2""#), "{\"x\": \"line1\\nline2\"}");
}

#[test]
fn str_escape_tab() {
    assert_eq!(ok(r#"x: "col1\tcol2""#), "{\"x\": \"col1\\tcol2\"}");
}

#[test]
fn str_escape_backslash() {
    assert_eq!(ok(r#"x: "a\\b""#), r#"{"x": "a\\b"}"#);
}

#[test]
fn str_escape_quote() {
    assert_eq!(ok(r#"x: "say \"hi\"""#), r#"{"x": "say \"hi\""}"#);
}

#[test]
fn str_multiple_escapes() {
    assert_eq!(
        ok(r#"x: "a\tb\\c\"d\ne""#),
        "{\"x\": \"a\\tb\\\\c\\\"d\\ne\"}",
    );
}

#[test]
fn str_unicode_passthrough() {
    // FIXME I wonder if this is legal JSON
    assert_eq!(ok(r#"x: "héllo wörld""#), r#"{"x": "héllo wörld"}"#);
}

// list values

#[test]
fn list_empty() {
    assert_eq!(ok("x: []"), r#"{"x": []}"#);
}

#[test]
fn list_single_element() {
    assert_eq!(ok("x: [1]"), r#"{"x": [1]}"#);
}

#[test]
fn list_ints() {
    assert_eq!(ok("x: [1, 2, 3]"), r#"{"x": [1, 2, 3]}"#);
}

#[test]
fn list_mixed_types() {
    assert_eq!(
        ok(r#"x: [1, "hello", true, 3.14]"#),
        r#"{"x": [1, "hello", true, 3.14]}"#,
    );
}

#[test]
fn list_nested() {
    assert_eq!(ok("x: [[1, 2], [3, 4]]"), r#"{"x": [[1, 2], [3, 4]]}"#);
}

#[test]
fn list_of_objects() {
    assert_eq!(
        ok("x: [#foo {a: 1}, #bar {b: 2}]"),
        r#"{"x": [{"id": "foo", "a": 1}, {"id": "bar", "b": 2}]}"#,
    );
}

#[test]
fn list_newline_separated() {
    assert_eq!(ok("x: [\n  1\n  2\n  3\n]"), r#"{"x": [1, 2, 3]}"#);
}

// object values

#[test]
fn object_empty_with_id() {
    assert_eq!(ok("x: #foo {}"), r#"{"x": {"id": "foo"}}"#);
}

#[test]
fn object_empty_without_id() {
    assert_eq!(ok("x: {}"), r#"{"x": {}}"#);
}

#[test]
fn object_comma_separated_defs() {
    assert_eq!(ok("x: {a: 1, b: 2}"), r#"{"x": {"a": 1, "b": 2}}"#);
}

#[test]
fn object_newline_separated_defs() {
    assert_eq!(ok("x: {\n  a: 1\n  b: 2\n}"), r#"{"x": {"a": 1, "b": 2}}"#);
}

#[test]
fn object_with_flags() {
    assert_eq!(
        ok("x: {active, !disabled}"),
        r#"{"x": {"active": true, "disabled": false}}"#,
    );
}

#[test]
fn object_with_id_and_defs() {
    assert_eq!(
        ok(r#"x: #item {name: "sword", damage: 10}"#),
        r#"{"x": {"id": "item", "name": "sword", "damage": 10}}"#,
    );
}

#[test]
fn object_nested() {
    assert_eq!(ok("x: {y: {z: 42}}"), r#"{"x": {"y": {"z": 42}}}"#);
}

#[test]
fn object_id_with_punct() {
    assert_eq!(ok("#health-potion {}"), r#"[{"id": "health-potion"}]"#);
}

// compound values

#[test]
fn compound_health_potion() {
    let input = "#health-potion {\n    heal: 50\n    consumable\n    !stackable\n    tags: [\"item\", \"consumable\"]\n}";
    assert_eq!(
        ok(input),
        r#"[{"id": "health-potion", "heal": 50, "consumable": true, "stackable": false, "tags": ["item", "consumable"]}]"#,
    );
}

#[test]
fn compound_multiple_entities() {
    let input = "#foo {\n  one\n  two\n  // four\n  three\n  x: #foo {one, two}\n  y: {one, two}\n  actions: [\n    #dmg {amount: 2}\n    #combo {\n      1: #dmg {amount: 3}\n      2: #heal {self, amount: 2}\n    }\n  ]\n}";
    assert_eq!(
        ok(input),
        r#"[{"id": "foo", "one": true, "two": true, "three": true, "x": {"id": "foo", "one": true, "two": true}, "y": {"one": true, "two": true}, "actions": [{"id": "dmg", "amount": 2}, {"id": "combo", "1": {"id": "dmg", "amount": 3}, "2": {"id": "heal", "self": true, "amount": 2}}]}]"#,
    );
}

#[test]
fn compound_pure_data_object() {
    let input = "name: \"Sword\"\ndamage: 10\ncrit: 0.15\nmagic: false";
    assert_eq!(
        ok(input),
        r#"{"name": "Sword", "damage": 10, "crit": 0.15, "magic": false}"#,
    );
}

// errors

#[test]
fn error_unclosed_string() {
    err(r#"x: "unclosed"#);
}

#[test]
fn error_unclosed_list() {
    err("x: [1, 2");
}

#[test]
fn error_unclosed_object() {
    err("x: {a: 1");
}

#[test]
fn error_invalid_escape() {
    err(r#"x: "\q""#);
}

#[test]
fn error_missing_value_after_colon() {
    err("x:");
}
