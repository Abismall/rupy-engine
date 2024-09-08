use pyo3::prelude::*;
use pyo3::types::{PyList, PyString};
use pyo3::types::{PyModule, PyTuple};
use std::env;
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn import_python_module<'py>(py: Python<'py>, module_name: &str) -> PyResult<Py<PyModule>> {
    let module = PyModule::import_bound(py, module_name)?;
    Ok(module.into())
}
fn append_to_sys_path(py: Python, dir_path: &str) -> PyResult<()> {
    let sys_module = PyModule::import_bound(py, "sys")?;
    let path_bound = sys_module.getattr("path")?;
    let path = path_bound.downcast::<pyo3::types::PyList>()?;
    path.append(PyString::new_bound(py, dir_path))?;
    Ok(())
}

fn execute_python_operation(
    module_name: &str,
    function_name: &str,
    args: Vec<PyObject>,
) -> PyResult<()> {
    Python::with_gil(|py| {
        let scripts_path = "./scripts";

        append_to_sys_path(py, scripts_path)?;

        let my_module = import_python_module(py, module_name)?;

        let module = my_module.bind(py);

        let py_args = PyTuple::new_bound(py, args);

        let result = module.call_method1(function_name, py_args)?;
        println!("Result from Python function: {:?}", result);

        Ok(())
    })
}
fn call_python_method(py: Python, module: &PyModule, method_name: &str, arg: f64) -> PyResult<()> {
    module.call_method1(method_name, (arg,))?;
    Ok(())
}

#[pyclass]
pub struct RupyEngine {
    last_update: Instant,
}

impl RupyEngine {
    fn get_scripts_path() -> Result<PathBuf, std::io::Error> {
        let mut path = env::current_exe()?;
        path.pop();

        loop {
            if path.is_dir() {
                for entry in std::fs::read_dir(&path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() && path.file_name().unwrap_or_default() == "scripts" {
                        return Ok(path);
                    }
                }
            }

            if !path.pop() {
                break;
            }
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Scripts directory not found in the path hierarchy",
        ))
    }
}

#[pymethods]
impl RupyEngine {
    #[new]
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
        }
    }
    pub fn tick(&mut self) -> PyResult<()> {
        Python::with_gil(|py| {
            let args = vec!["hello".into_py(py)];

            execute_python_operation("game_logic", "update_game_logic", args)?;

            Ok(())
        })
    }

    pub fn run(&mut self) -> PyResult<()> {
        loop {
            self.tick()?;
            std::thread::sleep(Duration::from_millis(16));
        }
    }
}
