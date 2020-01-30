//! Sources of JVM metadata such as .jars, jimage files, etc.

use crate::Class;
use zip::ZipArchive;
use std::borrow::Cow;
use std::cell::RefCell;
use std::default::Default;
use std::fs::File;
use std::ffi::*;
use std::io::{BufReader, Cursor, Error, ErrorKind, Result};
use std::iter::Extend;
use std::path::*;

enum SourceInt {
    Jar(Jar),
    JImage(JImage),
}

pub struct Source(SourceInt);

impl Source {
    pub fn from_jar(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self(SourceInt::Jar(Jar::open(path)?)))
    }

    pub fn from_jdk_dir(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let modules = path.join("lib").join("modules");
        if modules.exists() {
            let jimage = path.join("bin").join(jimage::Library::NAME);
            if jimage.exists() {
                return Self::from_jimage_modules(jimage, modules);
            } else {
                return Err(Error::new(ErrorKind::InvalidInput, format!("JDK or JRE contains lib/modules, but no bin/{} to read it with: {}", jimage::Library::NAME, path.display())));
            }
        }

        let rt_jar = path.join("jre").join("lib").join("rt.jar");
        if rt_jar.exists() {
            return Self::from_jar(rt_jar);
        }

        let rt_jar = path.join("lib").join("rt.jar");
        if rt_jar.exists() {
            return Self::from_jar(rt_jar);
        }

        Err(Error::new(ErrorKind::InvalidInput, format!("Unable to find lib/modules, jre/lib/rt.jar, or lib/rt.jar in: {}", path.display())))
    }

    pub fn read_class(&self, path: impl AsRef<str>) -> Result<Class> {
        let path = path.as_ref();
        match &self.0 {
            SourceInt::Jar(jar)     => jar.read_class(path),
            SourceInt::JImage(img)  => img.read_class(path),
        }
    }

    pub fn for_each_class(&self, mut f: impl FnMut(Cow<'_, str>) -> Result<()>) -> Result<()> {
        match &self.0 {
            SourceInt::Jar(jar)     => jar.for_each_class(|c| f(c.into())),
            SourceInt::JImage(img)  => img.for_each_class(|c| f(c.into())),
        }
    }

    pub fn classes<C: Default + Extend<String>>(&self) -> Result<C> {
        let mut collection = C::default();
        self.for_each_class(|class|{
            collection.extend(Some(class.to_string()));
            Ok(())
        })?;
        Ok(collection)
    }

    fn from_jimage_modules(jimage: impl AsRef<Path>, modules: impl AsRef<Path>) -> Result<Self> {
        Ok(Self(SourceInt::JImage(JImage::open(jimage, modules)?)))
    }
}

struct Jar(RefCell<ZipArchive<BufReader<File>>>);
impl Jar {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self(RefCell::new(ZipArchive::new(BufReader::new(File::open(path)?))?)))
    }

    pub fn read_class(&self, path: &str) -> Result<Class> {
        let mut zip = self.0.borrow_mut();
        let mut entry = zip.by_name(&format!("{}.class", path))?;
        Class::read(&mut entry)
    }

    pub fn for_each_class(&self, mut f: impl FnMut(&str) -> Result<()>) -> Result<()> {
        let mut zip = self.0.borrow_mut();
        for i in 0..zip.len() {
            let entry = zip.by_index(i)?;
            let name = entry.name();
            if name.ends_with(".class") {
                f(&name[..name.len()-6])?;
            }
        }
        Ok(())
    }
}

struct JImage(jimage::File);
impl JImage {
    pub fn open(jimage: impl AsRef<Path>, modules: impl AsRef<Path>) -> Result<Self> {
        let jimage = jimage::Library::load(jimage.as_ref())?;
        let modules = jimage.open(modules.as_ref())?;
        Ok(Self(modules))
    }

    pub fn read_class(&self, path: &str) -> Result<Class> {
        let err = |e: Error| Error::new(e.kind(), format!("Failed to jimage.read_class({:?}): {}", path, e));

        let slash = path.rfind('/');
        let package = match slash {
            Some(slash) => path.split_at(slash).0,
            None        => "",
        };

        let path    = CString::new(format!("{}.class", path)).expect("path cannot have any '\\0' characters");
        let package = CString::new(package).unwrap(); // Shouldn't fail if path didn't fail

        let module      = self.0.package_to_module(&package).map_err(err)?;
        let resource    = self.0.find_resource(module, Self::v9(), &path).map_err(err)?;
        let size        = resource.size();

        let size = if size > 100_000_000 {
            return Err(err(Error::new(ErrorKind::InvalidData, "exceeds 100MB in size")));
        } else {
            size as usize
        };

        let mut mem = Vec::new();
        mem.resize(size, 0);
        assert_eq!(size as u64, resource.get(&mut mem[..]).map_err(err)?);
        let mut mem = Cursor::new(mem);
        Class::read(&mut mem).map_err(err)
    }

    pub fn for_each_class(&self, mut f: impl FnMut(String) -> Result<()>) -> Result<()> {
        let mut result = Ok(());
        self.0.visit(|r|{
            if r.extension_cstr().to_bytes() != b"class" {
                // Possibly a gif, or any number of other resource types
                return jimage::VisitResult::Continue;
            }

            let package = match r.package() {
                Ok(p) => p,
                Err(e) => { result = Err(e); return jimage::VisitResult::Cancel; },
            };

            let name = match r.name() {
                Ok(n) => n,
                Err(e) => { result = Err(e); return jimage::VisitResult::Cancel; },
            };

            if package == "" && name == "module-info" {
                // Magic metadata nonsense
                return jimage::VisitResult::Continue;
            }

            let path = if package != "" {
                format!("{}/{}", package, name)
            } else {
                name.to_string()
            };

            match f(path) {
                Ok(()) => {},
                Err(e) => { result = Err(e); return jimage::VisitResult::Cancel; },
            }

            jimage::VisitResult::Continue
        });
        result
    }

    fn v9() -> &'static CStr { CStr::from_bytes_with_nul(b"9.0\0").unwrap() }
}
