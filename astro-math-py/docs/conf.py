# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

import sys
import os
import tomllib

# Add parent directory to path
sys.path.insert(0, os.path.abspath('..'))

# Read version from pyproject.toml
with open('../pyproject.toml', 'rb') as f:
    pyproject = tomllib.load(f)

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = 'astro-math'
copyright = '2025, astro-math contributors'
author = 'astro-math contributors'
release = pyproject['project']['version']

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.autosummary',
    'sphinx.ext.viewcode',
    'sphinx.ext.napoleon',
    'sphinx.ext.intersphinx',
    'autoapi.extension',
]

templates_path = ['_templates']
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']

# -- Options for HTML output ------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'sphinx_rtd_theme'
html_static_path = ['_static']

# -- AutoAPI configuration --------------------------------------------------
autoapi_type = 'python'
autoapi_dirs = ['../python']
autoapi_root = 'api'
autoapi_add_toctree_entry = False
autoapi_options = [
    'members',
    'undoc-members', 
    'show-inheritance',
    'show-module-summary',
    'special-members',
    'imported-members',
]

# AutoAPI settings
autoapi_python_class_content = 'both'
autoapi_member_order = 'groupwise'
autoapi_keep_files = True

# Suppress warnings about missing modules during build
autoapi_ignore = ['**/astro_math.astro_math*']

# -- Napoleon configuration --------------------------------------------------
napoleon_google_docstring = False
napoleon_numpy_docstring = True
napoleon_include_init_with_doc = False
napoleon_include_private_with_doc = False
napoleon_include_special_with_doc = True
napoleon_use_admonition_for_examples = False
napoleon_use_admonition_for_notes = False
napoleon_use_admonition_for_references = False
napoleon_use_ivar = False
napoleon_use_param = True
napoleon_use_rtype = True
napoleon_preprocess_types = False
napoleon_type_aliases = None
napoleon_attr_annotations = True

# -- Intersphinx mapping ----------------------------------------------------
intersphinx_mapping = {
    'python': ('https://docs.python.org/3/', None),
    'numpy': ('https://numpy.org/doc/stable/', None),
    'astropy': ('https://docs.astropy.org/en/stable/', None),
}

# -- Autosummary configuration ----------------------------------------------
autosummary_generate = True
autosummary_generate_overwrite = True