from setuptools import setup, find_packages
from setuptools_rust import (
    RustExtension, Binding
)


with open('README.md') as readme_file:
    readme = readme_file.read()


setup_requirements = ['pytest-runner', ]
test_requirements = ['pytest>=3', ]
requirements = []


setup(
    author="Todd Young",
    author_email='youngmt1@ornl.gov',
    python_requires='>=3.5',
    classifiers=[
        'Development Status :: 2 - Pre-Alpha',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: MIT License',
        'Natural Language :: English',
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8',
    ],
    description="Vocabulary building with Rust.",
    install_requires=requirements,
    license="MIT license",
    long_description=readme,
    include_package_data=True,
    keywords='monte',
    name='monte',
    rust_extensions=[RustExtension("monte", "Cargo.toml", debug=False)],
    setup_requires=setup_requirements,
    test_suite='tests',
    tests_require=test_requirements,
    url='https://github.com/DSE512/monte',
    version='0.1.0',
    zip_safe=False,
)
