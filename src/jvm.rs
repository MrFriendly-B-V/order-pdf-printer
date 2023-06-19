use crate::error::Error;
use jni::{AttachGuard, InitArgsBuilder, JNIVersion, JavaVM};
use std::fs;
use std::io::Write;
use tempfile::TempDir;

/// Java Virtual Machine
pub struct JVM {
    jvm: JavaVM,
    _dep_dir: TempDir,
}

impl JVM {
    /// Create a new JVM instance
    ///
    /// # Errors
    ///
    /// - If an IO error occurs
    /// - If a JNI error occurs
    /// - If the JVM cannot be launched
    pub fn new() -> Result<Self, Error> {
        let dependency_dir = TempDir::new()?;

        // Save logger adapter
        let logger_jar = dependency_dir.path().join("logger.jar");
        let mut file = fs::File::create(&logger_jar)?;
        file.write_all(tracing_slf4j::DEPENDENCIES)?;

        // Save itext
        let itext_jar = dependency_dir.path().join("itext.jar");
        let mut file = fs::File::create(&itext_jar)?;
        file.write_all(itext::bundle::DEPENDENCIES)?;

        let classpath = vec![
            logger_jar.to_string_lossy().to_string(),
            itext_jar.to_string_lossy().to_string(),
        ]
        .join(":");

        // Configure JVM
        let init_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option(format!("-Djava.class.path={classpath}"))
            .build()?;

        let jvm = JavaVM::new(init_args)?;

        let mut env = jvm.attach_current_thread()?;
        tracing_slf4j::register_log_fn(&mut env)?;
        drop(env);

        Ok(Self {
            _dep_dir: dependency_dir,
            jvm,
        })
    }

    /// Attach the current thread to the JVM.
    /// The thread is detached when the returned AttachGuard is dropped.
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    pub fn attach(&self) -> Result<AttachGuard, Error> {
        Ok(self.jvm.attach_current_thread()?)
    }
}
