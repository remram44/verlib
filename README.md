What is this?
=============

This is a parser and matcher for the Python version standard, described in [PEP 440](https://www.python.org/dev/peps/pep-0440/#version-scheme).

Why not semantic versioning (semver)?
=====================================

Semver has become very prevalent for people who can use it, and I understand the concern when new standards appear. However semver was designed without regards for some important needs, which is why it is not used by [Debian](https://www.debian.org/doc/debian-policy/ch-controlfields.html), [Fedora](https://fedoraproject.org/wiki/PackagingDrafts/TildeVersioning), [PyPI](https://docs.conda.io/projects/conda/en/latest/user-guide/concepts/pkg-specs.html#version-ordering), [conda](https://docs.conda.io/projects/conda/en/latest/user-guide/concepts/pkg-specs.html#version-ordering).

One important problem is the need for **post-releases**. While it is possible for a package author to release a new package with incremented patch number when he fixes a mistake, downstream package maintainer (people packaging your lib for a Linux distribution, the Conda package manager, as binary installers, ...) can't make changes to version numbers without incurring the risk of conflicting with your next upstream version.

What kind of versions does this package support?
================================================

This package supports both PEP-440 versions of the form `(N!]N(.N)*[{a|b|rc}N][.postN][.devN]+build`, and can convert them to/from semver.
