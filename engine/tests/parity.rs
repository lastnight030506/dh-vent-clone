//! Parity tests against oracle captures in `fixtures/`.

use std::path::PathBuf;

use serde_json::Value;

fn close(a: f64, b: f64) -> bool {
    let d = (a - b).abs();
    d <= 1e-9 || d / a.abs().max(b.abs()).max(1.0) <= 1e-6
}

fn compare(path: &str, exp: &Value, act: &Value, out: &mut Vec<String>) {
    match (exp, act) {
        (Value::Number(a), Value::Number(b)) => {
            let (a, b) = (a.as_f64().unwrap(), b.as_f64().unwrap());
            if !close(a, b) {
                out.push(format!("{path}: {a} != {b}"));
            }
        }
        (Value::Object(a), Value::Object(b)) => {
            for (k, v) in a {
                match b.get(k) {
                    Some(w) => compare(&format!("{path}.{k}"), v, w, out),
                    None => out.push(format!("{path}.{k}: missing (expected {v})")),
                }
            }
            for k in b.keys() {
                if !a.contains_key(k) {
                    out.push(format!("{path}.{k}: unexpected {}", b[k]));
                }
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                out.push(format!("{path}: length {} != {}", a.len(), b.len()));
            }
            for i in 0..a.len().min(b.len()) {
                compare(&format!("{path}[{i}]"), &a[i], &b[i], out);
            }
        }
        _ => {
            if exp != act {
                out.push(format!("{path}: {exp} != {act}"));
            }
        }
    }
}

#[test]
fn all_fixtures_match() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../fixtures");
    let mut inputs: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("fixtures directory")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with(".input.json"))
        })
        .collect();
    inputs.sort();
    assert!(!inputs.is_empty(), "no fixtures found");

    let mut failures = Vec::new();
    for input_path in inputs {
        let file = input_path.file_name().unwrap().to_str().unwrap();
        let name = file.trim_end_matches(".input.json");
        let expected_path = dir.join(format!("{name}.expected.json"));
        let input: Value =
            serde_json::from_str(&std::fs::read_to_string(&input_path).unwrap()).unwrap();
        let expected: Value =
            serde_json::from_str(&std::fs::read_to_string(&expected_path).unwrap()).unwrap();
        let actual = engine::compute_value(input).expect("compute");
        let mut diffs = Vec::new();
        compare(name, &expected, &actual, &mut diffs);
        if diffs.is_empty() {
            println!("ok {name}");
        } else {
            failures.push(format!("{name}: {} differences\n{}", diffs.len(), diffs.join("\n")));
        }
    }
    if !failures.is_empty() {
        panic!("{}", failures.join("\n\n"));
    }
}
