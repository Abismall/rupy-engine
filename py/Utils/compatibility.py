import sys
import subprocess
import importlib

REQUIRED_PYTHON_VERSION = (3, 8)
RUST_CHECK_REQUIRED = True


def check_python_version():
    """Check if the current Python version meets the minimum requirement."""
    if sys.version_info < REQUIRED_PYTHON_VERSION:
        print(f"Error: Python {
              REQUIRED_PYTHON_VERSION} or higher is required.")
        return False
    print(f"Python version {sys.version_info.major}.{
          sys.version_info.minor} is compatible.")
    return True


def check_packages(requirements_file="requirements.txt"):
    """Check if all required Python packages listed in the requirements file are installed."""
    try:
        with open(requirements_file, 'r') as file:
            requirements = file.readlines()
    except FileNotFoundError:
        print(f"Error: {
              requirements_file} not found. Please provide a valid requirements file.")
        return False

    all_installed = True
    for requirement in requirements:
        package, required_version = parse_requirement(requirement.strip())
        if package:
            if not check_package_installed(package, required_version):
                all_installed = False
    return all_installed


def parse_requirement(requirement):
    """Parse a requirement line to extract package name and version."""
    if "==" in requirement:
        package, version = requirement.split("==")
        return package, version
    return requirement, None


def check_package_installed(package, required_version=None):
    """Check if a specific package is installed and optionally check the version."""
    try:
        pkg = importlib.import_module(package)
        installed_version = getattr(pkg, '__version__', None)
        if required_version and installed_version:
            if installed_version < required_version:
                print(f"Error: {package} version {
                      required_version} or higher is required (found {installed_version}).")
                return False
            else:
                print(f"{package} version {installed_version} is compatible.")
        else:
            print(f"{package} is installed.")
        return True
    except ImportError:
        print(f"Error: {package} is not installed.")
        return False


def check_rust():
    """Check if Rust is installed and accessible via cargo."""
    try:
        result = subprocess.run(["cargo", "--version"],
                                capture_output=True, text=True, check=True)
        print(f"Rust toolchain is installed: {result.stdout.strip()}")
        return True
    except (subprocess.CalledProcessError, FileNotFoundError):
        print("Error: Rust toolchain is not installed. Install Rust via https://rustup.rs/")
        return False


def run_all_checks():
    """Run all compatibility checks."""
    python_ok = check_python_version()
    packages_ok = check_packages()
    rust_ok = check_rust() if RUST_CHECK_REQUIRED else True

    all_checks_passed = python_ok and packages_ok and rust_ok
    if all_checks_passed:
        print("All compatibility checks passed. You are ready to run the application.")
    else:
        print("Some compatibility checks failed. Please address the issues above.")


# Example usage
if __name__ == "__main__":
    run_all_checks()
