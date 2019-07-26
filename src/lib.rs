use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::wrap_pyfunction;

#[pyfunction]
/// Formats the sum of two numbers as string
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn test_array(a: std::vec::Vec<i32>) -> PyResult<i32> {
    let mut sum = 0;
    for v in a {
        sum += v;
    }
    Ok(sum)
}

#[pyfunction]
fn test_return_array(a: std::vec::Vec<i32>) -> PyResult<std::vec::Vec<i32>> {
    let result = a.iter().map(|x| x * x * x).collect::<Vec<_>>();
    Ok(result)
}

#[pyfunction]
fn test_return_dict(a: std::vec::Vec<i32>) -> PyResult<std::collections::HashMap<i64,i64>> {

    let mut x = std::collections::HashMap::new();

    for i in 1..100{
    x.insert(i, i*i);
    }

    Ok(x)
}

#[pyfunction]
fn test_lambda(
    py: Python,
    a: std::vec::Vec<i32>,
    f: PyObject,
) -> PyResult<std::vec::Vec<PyObject>> {
    let mut mapped = std::vec::Vec::new();
    for i in 0..a.len() {
        let val = f.call(py, PyTuple::new(py, &[a[i]]), None)?;
        mapped.push(val)
    }
    Ok(mapped)
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn rust_from_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(test_array))?;
    m.add_wrapped(wrap_pyfunction!(test_return_array))?;
    m.add_wrapped(wrap_pyfunction!(test_lambda))?;

    m.add_wrapped(wrap_pyfunction!(test_return_dict))?;
    Ok(())
}
