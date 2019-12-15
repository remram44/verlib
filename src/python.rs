//! Implement Python's versioning scheme.
//!
//! Python's versioning scheme is described in
//! [PEP-440](https://www.python.org/dev/peps/pep-0440/). Contrary to semver,
//! it allows for post-releases (`.postN`), development versions (`.devN`), and
//! epochs (`N!` prefix). It also does not mandate any semantics, explicitly
//! allowing date-based releases.
//!
//! It is unusual in that it gives meaning to specific identifiers, such as
//! `post`, `dev`, `rc``, `a` (for alpha), and `b` (for beta).

/// A PEP-440-compliant Python version number.
#[derive(Clone, Debug)]
pub struct PythonVersion(String);
