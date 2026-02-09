use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn square_even_numbers_in_list(v: Vec<usize>) -> PyResult<Vec<usize>> {
    let v_s: Vec<usize> = v
        .into_iter()
        .filter_map(|num| match num % 2 == 0 {
            true => Some(num.pow(2)),
            false => None,
        })
        .collect();

    Ok(v_s)
}

#[pyfunction]
fn just_return(a: usize) -> PyResult<usize> {
    Ok(a)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(square_even_numbers_in_list, m)?)?;
    m.add_function(wrap_pyfunction!(just_return, m)?)?;
    Ok(())
}
