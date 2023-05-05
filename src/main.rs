use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

fn main() -> PyResult<()> {
    
    Python::with_gil(|py| {
        

        let locals = [("pyautogui", py.import("pyautogui").unwrap())].into_py_dict(py);
        let code = "pyautogui.moveTo(1216, 697)";
        py.eval(code, None, Some(&locals)).unwrap(); 

        // println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}

