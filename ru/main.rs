use pyo3::PyResult;
use rupyengine::engine;

pub fn main() -> PyResult<()> {
    let mut engine = engine::RupyEngine::new();
    engine.run()
}
