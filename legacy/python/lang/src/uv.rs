use cached::proc_macro::cached;
use moon_lang::{config_cache_container, LockfileDependencyVersions};
use pyproject_toml::PyProjectToml;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use starbase_utils::{fs, toml};
use std::path::{Path, PathBuf};

fn read_file(path: &Path) -> miette::Result<PyProjectToml> {
    Ok(toml::parse(fs::read_file(path)?)?)
}

config_cache_container!(
    PyProjectTomlCache,
    PyProjectToml,
    "package-lock.json",
    read_file
);

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct UvLockPackageSdist {
    pub url: String,
    pub hash: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct UvLockPackage {
    pub name: String,
    pub version: String,
    pub sdist: UvLockPackageSdist,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct UvLock {
    pub packages: Vec<UvLockPackage>,
}

#[cached(result)]
pub fn load_lockfile_dependencies(path: PathBuf) -> miette::Result<LockfileDependencyVersions> {
    let mut deps: LockfileDependencyVersions = FxHashMap::default();
    let lockfile: UvLock = toml::read_file(&path)?;

    for package in lockfile.packages {
        let dep = deps.entry(package.name).or_default();
        dep.push(package.version);
        dep.push(package.sdist.hash);
    }

    Ok(deps)
}
