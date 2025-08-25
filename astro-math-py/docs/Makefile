# Makefile for Sphinx documentation
#

# You can set these variables from the command line, and also
# from the environment for the first two.
SPHINXOPTS    ?=
SPHINXBUILD  ?= sphinx-build
SOURCEDIR     = .
BUILDDIR      = _build

# Put it first so that "make" without argument is like "make help".
help:
	@$(SPHINXBUILD) -M help "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

.PHONY: help Makefile clean html dirhtml singlehtml pickle json htmlhelp qthelp devhelp epub latex latexpdf text man texinfo info gettext changes xml pseudoxml linkcheck doctest coverage

# Catch-all target: route all unknown targets to Sphinx-build using the
# "make mode" option.  $(O) is meant as a shortcut for $(SPHINXOPTS).
%: Makefile
	@$(SPHINXBUILD) -M $@ "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

# Custom targets for common tasks
clean:
	@$(SPHINXBUILD) -M clean "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

html:
	@$(SPHINXBUILD) -M html "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)
	@echo
	@echo "Build finished. The HTML pages are in $(BUILDDIR)/html."

livehtml:
	sphinx-autobuild "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

# Development convenience targets
.PHONY: install serve check-deps
check-deps:
	@command -v sphinx-build >/dev/null 2>&1 || { echo >&2 "Sphinx not found. Install with: pip install -e '.[docs]'"; exit 1; }

install:
	@echo "Installing documentation dependencies..."
	pip install -e "..[docs]"
	@echo "Building Rust extension..."
	maturin develop
	@echo "Ready to build docs with: make html"

serve: html
	@cd $(BUILDDIR)/html && python -m http.server 8000

# Remove duplicate html target to avoid warning