use crate::sys::Memory;
use crate::types::{
    FuncSig, GlobalDescriptor, MemoryDescriptor, MemoryIndex, TableDescriptor, TableIndex, Type,
};
use core::borrow::Borrow;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, Error>;
pub type CompileResult<T> = std::result::Result<T, CompileError>;
pub type LinkResult<T> = std::result::Result<T, Vec<LinkError>>;
pub type RuntimeResult<T> = std::result::Result<T, RuntimeError>;
pub type CallResult<T> = std::result::Result<T, CallError>;
pub type ResolveResult<T> = std::result::Result<T, ResolveError>;

/// This is returned when the chosen compiler is unable to
/// successfully compile the provided webassembly module into
/// a `Module`.
///
/// Comparing two `CompileError`s always evaluates to false.
#[derive(Debug, Clone)]
pub enum CompileError {
    ValidationError { msg: String },
    InternalError { msg: String },
}

impl PartialEq for CompileError {
    fn eq(&self, _other: &CompileError) -> bool {
        false
    }
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompileError::InternalError { msg } => {
                write!(f, "Internal compiler error: \"{}\"", msg)
            }
            CompileError::ValidationError { msg } => write!(f, "Validation error \"{}\"", msg),
        }
    }
}

impl std::error::Error for CompileError {}

/// This is returned when the runtime is unable to
/// correctly link the module with the provided imports.
///
/// Comparing two `LinkError`s always evaluates to false.
#[derive(Debug, Clone)]
pub enum LinkError {
    IncorrectImportType {
        namespace: String,
        name: String,
        expected: String,
        found: String,
    },
    IncorrectImportSignature {
        namespace: String,
        name: String,
        expected: Arc<FuncSig>,
        found: Arc<FuncSig>,
    },
    ImportNotFound {
        namespace: String,
        name: String,
    },
    IncorrectMemoryDescriptor {
        namespace: String,
        name: String,
        expected: MemoryDescriptor,
        found: MemoryDescriptor,
    },
    IncorrectTableDescriptor {
        namespace: String,
        name: String,
        expected: TableDescriptor,
        found: TableDescriptor,
    },
    IncorrectGlobalDescriptor {
        namespace: String,
        name: String,
        expected: GlobalDescriptor,
        found: GlobalDescriptor,
    },
}

impl PartialEq for LinkError {
    fn eq(&self, _other: &LinkError) -> bool {
        false
    }
}

impl std::fmt::Display for LinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LinkError::ImportNotFound {namespace, name} => write!(f, "Import not found, namespace: {}, name: {}", namespace, name),
            LinkError::IncorrectGlobalDescriptor {namespace, name,expected,found} => {
                write!(f, "Incorrect global descriptor, namespace: {}, name: {}, expected global descriptor: {:?}, found global descriptor: {:?}", namespace, name, expected, found)
            },
            LinkError::IncorrectImportSignature{namespace, name,expected,found} => {
                write!(f, "Incorrect import signature, namespace: {}, name: {}, expected signature: {}, found signature: {}", namespace, name, expected, found)
            }
            LinkError::IncorrectImportType{namespace, name,expected,found} => {
                write!(f, "Incorrect import type, namespace: {}, name: {}, expected type: {}, found type: {}", namespace, name, expected, found)
            }
            LinkError::IncorrectMemoryDescriptor{namespace, name,expected,found} => {
                write!(f, "Incorrect memory descriptor, namespace: {}, name: {}, expected memory descriptor: {:?}, found memory descriptor: {:?}", namespace, name, expected, found)
            },
            LinkError::IncorrectTableDescriptor{namespace, name,expected,found} => {
                write!(f, "Incorrect table descriptor, namespace: {}, name: {}, expected table descriptor: {:?}, found table descriptor: {:?}", namespace, name, expected, found)
            },
        }
    }
}

impl std::error::Error for LinkError {}

/// This is the error type returned when calling
/// a webassembly function.
///
/// The main way to do this is `Instance.call`.
///
/// Comparing two `RuntimeError`s always evaluates to false.
#[derive(Debug, Clone)]
pub enum RuntimeError {
    OutOfBoundsAccess {
        memory: MemoryIndex,
        addr: Option<u32>,
    },
    TableOutOfBounds {
        table: TableIndex,
    },
    IndirectCallSignature {
        table: TableIndex,
    },
    IndirectCallToNull {
        table: TableIndex,
    },
    IllegalArithmeticOperation,
    User {
        msg: String,
    },
    Unknown {
        msg: String,
    },
}

impl PartialEq for RuntimeError {
    fn eq(&self, _other: &RuntimeError) -> bool {
        false
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuntimeError::IndirectCallSignature { table } => write!(
                f,
                "Indirect call signature error with Table Index \"{:?}\"",
                table
            ),
            RuntimeError::IndirectCallToNull { table } => {
                write!(f, "Indirect call to null with table index \"{:?}\"", table)
            }
            RuntimeError::IllegalArithmeticOperation => write!(f, "Illegal arithmetic operation"),
            RuntimeError::OutOfBoundsAccess { memory, addr } => match addr {
                Some(addr) => write!(
                    f,
                    "Out-of-bounds access with memory index {:?} and address {}",
                    memory, addr
                ),
                None => write!(f, "Out-of-bounds access with memory index {:?}", memory),
            },
            RuntimeError::TableOutOfBounds { table } => {
                write!(f, "Table out of bounds with table index \"{:?}\"", table)
            }
            RuntimeError::Unknown { msg } => {
                write!(f, "Unknown runtime error with message: \"{}\"", msg)
            }
            RuntimeError::User { msg } => write!(f, "User runtime error with message: \"{}\"", msg),
        }
    }
}

impl std::error::Error for RuntimeError {}

/// This error type is produced by resolving a wasm function
/// given its name.
///
/// Comparing two `ResolveError`s always evaluates to false.
#[derive(Debug, Clone)]
pub enum ResolveError {
    Signature {
        expected: Arc<FuncSig>,
        found: Vec<Type>,
    },
    ExportNotFound {
        name: String,
    },
    ExportWrongType {
        name: String,
    },
}

impl PartialEq for ResolveError {
    fn eq(&self, _other: &ResolveError) -> bool {
        false
    }
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResolveError::ExportNotFound { name } => write!(f, "Export not found: {}", name),
            ResolveError::ExportWrongType { name } => write!(f, "Export wrong type: {}", name),
            ResolveError::Signature { expected, found } => {
                let found = found
                    .as_slice()
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                let expected: &FuncSig = expected.borrow();
                write!(
                    f,
                    "Parameters of type [{}] did not match signature {}",
                    found, expected
                )
            }
        }
    }
}

impl std::error::Error for ResolveError {}

/// This error type is produced by calling a wasm function
/// exported from a module.
///
/// If the module traps in some way while running, this will
/// be the `CallError::Runtime(RuntimeError)` variant.
///
/// Comparing two `CallError`s always evaluates to false.
#[derive(Debug, Clone)]
pub enum CallError {
    Resolve(ResolveError),
    Runtime(RuntimeError),
}

impl PartialEq for CallError {
    fn eq(&self, _other: &CallError) -> bool {
        false
    }
}

impl std::fmt::Display for CallError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CallError::Resolve(resolve_error) => write!(f, "Call error: {}", resolve_error),
            CallError::Runtime(runtime_error) => write!(f, "Call error: {}", runtime_error),
        }
    }
}

impl std::error::Error for CallError {}

/// This error type is produced when creating something,
/// like a `Memory` or a `Table`.
#[derive(Debug, Clone)]
pub enum CreationError {
    UnableToCreateMemory,
    UnableToCreateTable,
    InvalidDescriptor(String),
}

impl PartialEq for CreationError {
    fn eq(&self, _other: &CreationError) -> bool {
        false
    }
}

impl std::fmt::Display for CreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CreationError::UnableToCreateMemory => write!(f, "Unable to Create Memory"),
            CreationError::UnableToCreateTable => write!(f, "Unable to Create Table"),
            CreationError::InvalidDescriptor(msg) => write!(
                f,
                "Unable to create because the supplied descriptor is invalid: \"{}\"",
                msg
            ),
        }
    }
}

impl std::error::Error for CreationError {}

/// The amalgamation of all errors that can occur
/// during the compilation, instantiation, or execution
/// of a webassembly module.
///
/// Comparing two `Error`s always evaluates to false.
#[derive(Debug, Clone)]
pub enum Error {
    CompileError(CompileError),
    LinkError(Vec<LinkError>),
    RuntimeError(RuntimeError),
    ResolveError(ResolveError),
    CallError(CallError),
    CreationError(CreationError),
}

impl PartialEq for Error {
    fn eq(&self, _other: &Error) -> bool {
        false
    }
}

impl From<CompileError> for Error {
    fn from(compile_err: CompileError) -> Self {
        Error::CompileError(compile_err)
    }
}

impl From<RuntimeError> for Error {
    fn from(runtime_err: RuntimeError) -> Self {
        Error::RuntimeError(runtime_err)
    }
}

impl From<ResolveError> for Error {
    fn from(resolve_err: ResolveError) -> Self {
        Error::ResolveError(resolve_err)
    }
}

impl From<CallError> for Error {
    fn from(call_err: CallError) -> Self {
        Error::CallError(call_err)
    }
}

impl From<CreationError> for Error {
    fn from(creation_err: CreationError) -> Self {
        Error::CreationError(creation_err)
    }
}

impl From<Vec<LinkError>> for Error {
    fn from(link_errs: Vec<LinkError>) -> Self {
        Error::LinkError(link_errs)
    }
}

impl From<RuntimeError> for CallError {
    fn from(runtime_err: RuntimeError) -> Self {
        CallError::Runtime(runtime_err)
    }
}

impl From<ResolveError> for CallError {
    fn from(resolve_err: ResolveError) -> Self {
        CallError::Resolve(resolve_err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub enum GrowError {
    MemoryGrowError,
    TableGrowError,
    ExceededMaxPages(PageError),
    ExceededMaxPagesForMemory(usize, usize),
    CouldNotProtectMemory(MemoryProtectionError),
    CouldNotCreateMemory(MemoryCreationError),
}

impl std::fmt::Display for GrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GrowError::MemoryGrowError => write!(f, "Unable to grow memory"),
            GrowError::TableGrowError => write!(f, "Unable to grow table"),
            GrowError::ExceededMaxPages(e) => write!(f, "Grow Error: {}", e),
            GrowError::ExceededMaxPagesForMemory(left, added) => write!(f, "Failed to add pages because would exceed maximum number of pages for the memory. Left: {}, Added: {}", left, added),
            GrowError::CouldNotCreateMemory(e) => write!(f, "Grow Error: {}", e),
            GrowError::CouldNotProtectMemory(e) => write!(f, "Grow Error: {}", e),
        }
    }
}

impl std::error::Error for GrowError {}

#[derive(Debug)]
pub enum PageError {
    // left, right, added
    ExceededMaxPages(usize, usize, usize),
}

impl std::fmt::Display for PageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PageError::ExceededMaxPages(left, right, added) => write!(f, "Failed to add pages because would exceed maximum number of pages. Left: {}, Right: {}, Pages added: {}", left, right, added),
        }
    }
}
impl std::error::Error for PageError {}

impl Into<GrowError> for PageError {
    fn into(self) -> GrowError {
        GrowError::ExceededMaxPages(self)
    }
}

#[derive(Debug)]
pub enum MemoryCreationError {
    VirtualMemoryAllocationFailed(usize, String),
    CouldNotCreateMemoryFromFile(std::io::Error),
}

impl std::fmt::Display for MemoryCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MemoryCreationError::VirtualMemoryAllocationFailed(size, msg) => write!(
                f,
                "Allocation virtual memory with size {} failed. \nErrno message: {}",
                size, msg
            ),
            MemoryCreationError::CouldNotCreateMemoryFromFile(e) => write!(f, "IO Error: {}", e),
        }
    }
}
impl std::error::Error for MemoryCreationError {}

impl Into<GrowError> for MemoryCreationError {
    fn into(self) -> GrowError {
        GrowError::CouldNotCreateMemory(self)
    }
}

impl From<std::io::Error> for MemoryCreationError {
    fn from(io_error: std::io::Error) -> Self {
        MemoryCreationError::CouldNotCreateMemoryFromFile(io_error)
    }
}

#[derive(Debug)]
pub enum MemoryProtectionError {
    ProtectionFailed(usize, usize, String),
}

impl std::fmt::Display for MemoryProtectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MemoryProtectionError::ProtectionFailed(start, size, msg) => write!(
                f,
                "Allocation virtual memory starting at {} with size {} failed. \nErrno message: {}",
                start, size, msg
            ),
        }
    }
}
impl std::error::Error for MemoryProtectionError {}

impl Into<GrowError> for MemoryProtectionError {
    fn into(self) -> GrowError {
        GrowError::CouldNotProtectMemory(self)
    }
}
