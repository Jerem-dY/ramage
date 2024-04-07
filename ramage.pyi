from typing import Any


class Tree:

    def __init__(self): ...

    # _add_node(&mut self, parent: Option<usize>, conns: Vec<usize>, trans: Vec<Option<Py<PyAny>>>, value: &Bound<'_, PyAny>, parent_transition: Option<&Bound<'_, PyAny>>) -> PyResult<usize> 
    def _add_node(self, parent: int, conns: list[int], trans: list[Any], value: Any, parent_transition: Any) -> None: ...