mod tree;
mod trie;

use pyo3::prelude::*;



/// A Python module implemented in Rust.
#[pymodule]
fn ramage(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<tree::Tree>()?;
    m.add_class::<tree::Property>()?;
    m.add_class::<tree::Search>()?;
    m.add_class::<trie::Trie>()?;
    Ok(())
}
