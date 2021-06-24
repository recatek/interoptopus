//! Generates C# bindings for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//!
//! ## Usage
//!
//! In your library or a support project add this:
//!
//! ```
//! # mod my_crate { use interoptopus::{Library}; pub fn ffi_inventory() -> Library { todo!() } }
//! use my_crate::ffi_inventory;
//!
//! #[test]
//! fn generate_csharp_bindings() {
//!     use interoptopus::Interop;
//!     use interoptopus_backend_csharp::{Generator, InteropCSharp, Config};
//!
//!     // Converts an `ffi_inventory()` into C# interop definitions.
//!     Generator::new(Config::default(), ffi_inventory()).write_to("Interop.cs")
//! }
//! ```
//!
//! And we might produce something like this:
//!
//! ```cs
//! using System;
//! using System.Runtime.InteropServices;
//!
//! namespace My.Company
//! {
//!     public static class InteropClass
//!     {
//!         public const string NativeLib = "hello_world";
//!
//!         /// A function which does something with the vector.
//!         [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "my_game_function")]
//!         public static extern Vec3f32 my_game_function(ref Vec3f32 input);
//!     }
//!
//!     [Serializable]
//!     [StructLayout(LayoutKind.Sequential)]
//!     public partial struct Vec3f32
//!     {
//!         public float x;
//!         public float y;
//!         public float z;
//!     }
//! }
//! ```

use interoptopus::lang::c::{
    CType, CompositeType, Constant, ConstantValue, Documentation, EnumType, Field, FnPointerType, Function, Meta, OpaqueType, Parameter, PrimitiveType, PrimitiveValue,
    Variant,
};
use interoptopus::patterns::service::Service;
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::util::{longest_common_prefix, safe_name, IdPrettifier, NamespaceMappings};
use interoptopus::writer::IndentWriter;
use interoptopus::Interop;
use interoptopus::{Error, Library};

/// Configures C# code generation.
#[derive(Clone, Debug)]
pub struct Config {
    /// The file header, e.g., `// (c) My Company`.
    pub file_header_comment: String,
    /// Static class for Interop methods, e.g., `Interop`.
    pub class: String,
    /// DLL to load, e.g., `my_library`.
    pub dll_name: String,
    /// Maps which namespace id belongs into which FQN (e.g., "common" => "MyCompany.Common").
    pub namespace_mappings: NamespaceMappings,
    /// Namespace ID of _this_ namespace to write (default "").
    pub namespace_id: String,
}

impl Config {}

impl Default for Config {
    fn default() -> Self {
        Self {
            file_header_comment: "// Automatically generated by Interoptopus.".to_string(),
            class: "Interop".to_string(),
            dll_name: "library".to_string(),
            namespace_mappings: NamespaceMappings::new("My.Company"),
            namespace_id: "".to_string(),
        }
    }
}

/// Helper type implementing [`InteropCSharp`] and [`Interop`].
pub struct Generator {
    config: Config,
    library: Library,
}

impl Generator {
    pub fn new(config: Config, library: Library) -> Self {
        Self { config, library }
    }
}

/// Contains all C# generators, create sub-trait to customize.
pub trait InteropCSharp {
    /// Returns the user config.
    fn config(&self) -> &Config;

    /// Returns the library to produce bindings for.
    fn library(&self) -> &Library;

    /// Converts a primitive (Rust) type to a native C# type name, e.g., `f32` to `float`.
    fn type_primitive_to_typename(&self, x: &PrimitiveType) -> String {
        match x {
            PrimitiveType::Void => "void".to_string(),
            PrimitiveType::Bool => "bool".to_string(),
            PrimitiveType::U8 => "byte".to_string(),
            PrimitiveType::U16 => "ushort".to_string(),
            PrimitiveType::U32 => "uint".to_string(),
            PrimitiveType::U64 => "ulong".to_string(),
            PrimitiveType::I8 => "sbyte".to_string(),
            PrimitiveType::I16 => "short".to_string(),
            PrimitiveType::I32 => "int".to_string(),
            PrimitiveType::I64 => "long".to_string(),
            PrimitiveType::F32 => "float".to_string(),
            PrimitiveType::F64 => "double".to_string(),
        }
    }

    /// Converts a Rust enum name such as `Error` to a C# enum name `Error`.
    fn type_enum_to_typename(&self, x: &EnumType) -> String {
        x.rust_name().to_string()
    }

    /// TODO Converts an opaque Rust struct `Context` to a C# struct ``.
    fn type_opaque_to_typename(&self, _: &OpaqueType) -> String {
        // x.name().to_string()
        "IntPtr".to_string()
    }

    /// Converts an Rust struct name `Vec2` to a C# struct name `Vec2`.
    fn type_composite_to_typename(&self, x: &CompositeType) -> String {
        x.rust_name().to_string()
    }

    /// Converts an Rust `fn()` to a C# delegate name such as `InteropDelegate`.
    fn type_fnpointer_to_typename(&self, x: &FnPointerType) -> String {
        vec!["InteropDelegate".to_string(), safe_name(&x.internal_name())].join("_")
    }

    /// Converts the `u32` part in a Rust field `x: u32` to a C# equivalent. Might convert pointers to `IntPtr`.
    fn type_to_typespecifier_in_field(&self, x: &CType, _field: &Field, _composite: &CompositeType) -> String {
        match &x {
            CType::Primitive(x) => self.type_primitive_to_typename(x),
            CType::Enum(x) => self.type_enum_to_typename(x),
            CType::Opaque(x) => self.type_opaque_to_typename(x),
            CType::Composite(x) => self.type_composite_to_typename(x),
            CType::ReadPointer(_) => "IntPtr".to_string(),
            CType::ReadWritePointer(_) => "IntPtr".to_string(),
            CType::FnPointer(x) => self.type_fnpointer_to_typename(x),
            CType::Pattern(x) => match x {
                TypePattern::AsciiPointer => "string".to_string(),
                TypePattern::SuccessEnum(e) => self.type_enum_to_typename(e.the_enum()),
                TypePattern::Slice(e) => self.type_composite_to_typename(e),
                TypePattern::Option(e) => self.type_composite_to_typename(e),
            },
        }
    }

    /// Converts the `u32` part in a Rust paramter `x: u32` to a C# equivalent. Might convert pointers to `out X` or `ref X`.
    fn type_to_typespecifier_in_param(&self, x: &CType) -> String {
        match &x {
            CType::Primitive(x) => self.type_primitive_to_typename(x),
            CType::Enum(x) => self.type_enum_to_typename(x),
            CType::Opaque(x) => self.type_opaque_to_typename(x),
            CType::Composite(x) => self.type_composite_to_typename(x),
            CType::ReadPointer(z) => match **z {
                CType::Opaque(_) => "IntPtr".to_string(),
                CType::Primitive(PrimitiveType::Void) => "IntPtr".to_string(),
                CType::ReadPointer(_) => "ref IntPtr".to_string(),
                CType::ReadWritePointer(_) => "ref IntPtr".to_string(),
                _ => format!("ref {}", self.type_to_typespecifier_in_param(z)),
            },
            CType::ReadWritePointer(z) => match **z {
                CType::Opaque(_) => "IntPtr".to_string(),
                CType::Primitive(PrimitiveType::Void) => "IntPtr".to_string(),
                CType::ReadPointer(_) => "out IntPtr".to_string(),
                CType::ReadWritePointer(_) => "out IntPtr".to_string(),
                _ => format!("out {}", self.type_to_typespecifier_in_param(z)),
            },
            CType::FnPointer(x) => self.type_fnpointer_to_typename(x),
            CType::Pattern(x) => match x {
                TypePattern::AsciiPointer => "string".to_string(),
                TypePattern::SuccessEnum(e) => self.type_enum_to_typename(e.the_enum()),
                TypePattern::Slice(x) => self.type_composite_to_typename(x),
                TypePattern::Option(x) => self.type_composite_to_typename(x),
            },
        }
    }

    fn type_to_typespecifier_in_rval(&self, x: &CType) -> String {
        match &x {
            CType::Primitive(x) => self.type_primitive_to_typename(x),
            CType::Enum(x) => self.type_enum_to_typename(x),
            CType::Opaque(x) => self.type_opaque_to_typename(x),
            CType::Composite(x) => self.type_composite_to_typename(x),
            CType::ReadPointer(_) => "IntPtr".to_string(),
            CType::ReadWritePointer(_) => "IntPtr".to_string(),
            CType::FnPointer(x) => self.type_fnpointer_to_typename(x),
            CType::Pattern(x) => match x {
                TypePattern::AsciiPointer => "string".to_string(),
                TypePattern::SuccessEnum(e) => self.type_enum_to_typename(e.the_enum()),
                TypePattern::Slice(x) => self.type_composite_to_typename(x),
                TypePattern::Option(x) => self.type_composite_to_typename(x),
            },
        }
    }

    fn constant_value_to_value(&self, value: &ConstantValue) -> String {
        match value {
            ConstantValue::Primitive(x) => match x {
                PrimitiveValue::Bool(x) => format!("{}", x),
                PrimitiveValue::U8(x) => format!("{}", x),
                PrimitiveValue::U16(x) => format!("{}", x),
                PrimitiveValue::U32(x) => format!("{}", x),
                PrimitiveValue::U64(x) => format!("{}", x),
                PrimitiveValue::I8(x) => format!("{}", x),
                PrimitiveValue::I16(x) => format!("{}", x),
                PrimitiveValue::I32(x) => format!("{}", x),
                PrimitiveValue::I64(x) => format!("{}", x),
                PrimitiveValue::F32(x) => format!("{}", x),
                PrimitiveValue::F64(x) => format!("{}", x),
            },
        }
    }

    fn function_parameter_to_csharp_typename(&self, x: &Parameter, _function: &Function) -> String {
        self.type_to_typespecifier_in_param(x.the_type())
    }

    fn function_rval_to_csharp_typename(&self, function: &Function) -> String {
        self.type_to_typespecifier_in_rval(function.signature().rval())
    }

    fn function_name_to_csharp_name(&self, function: &Function) -> String {
        function.name().to_string()
    }

    fn write_file_header_comments(&self, w: &mut IndentWriter) -> Result<(), Error> {
        writeln!(w.writer(), "{}", &self.config().file_header_comment)?;
        Ok(())
    }

    fn write_imports(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"using System;"#))?;
        w.indented(|w| writeln!(w, r#"using System.Runtime.InteropServices;"#))?;

        for namespace_id in self.library().namespaces() {
            let namespace = self
                .config()
                .namespace_mappings
                .get(namespace_id)
                .unwrap_or_else(|| panic!("Must have namespace for '{}' ID", namespace_id));
            w.indented(|w| writeln!(w, r#"using {};"#, namespace))?;
        }

        Ok(())
    }

    fn write_native_lib_string(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"public const string NativeLib = "{}";"#, self.config().dll_name))?;
        Ok(())
    }

    fn write_constants(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for constant in self.library().constants() {
            if self.should_emit(constant.meta()) {
                self.write_constant(w, constant)?;
                w.newline()?;
            }
        }

        Ok(())
    }

    fn write_constant(&self, w: &mut IndentWriter, constant: &Constant) -> Result<(), Error> {
        self.write_documentation(w, constant.meta().documentation())?;

        w.indented(|w| write!(w, r#"public const "#))?;

        write!(w.writer(), "{} ", self.type_to_typespecifier_in_rval(&constant.the_type()))?;
        write!(w.writer(), "{} = ", constant.name())?;
        write!(w.writer(), "{};", self.constant_value_to_value(constant.value()))?;

        w.newline()?;

        Ok(())
    }

    fn write_functions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for function in self.library().functions() {
            if self.should_emit(function.meta()) {
                self.write_function(w, function)?;
                w.newline()?;
            }
        }

        Ok(())
    }

    fn write_function(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        self.write_documentation(w, function.meta().documentation())?;
        self.write_function_annotation(w, function)?;
        self.write_function_declaration(w, function)?;
        Ok(())
    }

    fn write_documentation(&self, w: &mut IndentWriter, documentation: &Documentation) -> Result<(), Error> {
        for line in documentation.lines() {
            w.indented(|w| writeln!(w, r#"/// {}"#, line))?;
        }

        Ok(())
    }

    fn write_function_annotation(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        w.indented(|w| {
            writeln!(
                w,
                r#"[DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "{}")]"#,
                function.name()
            )
        })?;
        Ok(())
    }

    fn write_function_declaration(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        w.indented(|w| write!(w, r#"public static extern "#))?;

        write!(w.writer(), "{}", self.function_rval_to_csharp_typename(function))?;
        write!(w.writer(), " {}(", self.function_name_to_csharp_name(function))?;

        let params = function.signature().params();
        for (i, p) in params.iter().enumerate() {
            write!(w.writer(), "{}", self.function_parameter_to_csharp_typename(p, function))?;
            write!(w.writer(), " {}", p.name())?;
            if i < params.len() - 1 {
                write!(w.writer(), ", ")?;
            }
        }

        writeln!(w.writer(), ");")?;
        Ok(())
    }

    fn write_type_definitions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for the_type in self.library().ctypes() {
            self.write_type_definition(w, the_type)?;
        }

        Ok(())
    }

    fn write_type_definition(&self, w: &mut IndentWriter, the_type: &CType) -> Result<(), Error> {
        match the_type {
            CType::Primitive(_) => {}
            CType::Enum(e) => {
                if self.should_emit(e.meta()) {
                    self.write_type_definition_enum(w, e)?;
                    w.newline()?;
                }
            }
            CType::Opaque(_) => {}
            CType::Composite(c) => {
                if self.should_emit(c.meta()) {
                    self.write_type_definition_composite(w, c)?;
                    w.newline()?;
                }
            }
            CType::FnPointer(f) => {
                if self.should_emit_delegate() {
                    self.write_type_definition_fn_pointer(w, f)?;
                    w.newline()?;
                }
            }
            CType::ReadPointer(_) => {}
            CType::ReadWritePointer(_) => {}
            CType::Pattern(x) => match x {
                TypePattern::AsciiPointer => {}
                TypePattern::SuccessEnum(e) => {
                    if self.should_emit(e.the_enum().meta()) {
                        self.write_type_definition_enum(w, e.the_enum())?;
                        w.newline()?;
                    }
                }
                TypePattern::Slice(x) => {
                    if self.should_emit(x.meta()) {
                        self.write_type_definition_composite(w, x)?;
                        w.newline()?;
                        self.write_pattern_slice(w, x)?;
                        w.newline()?;
                    }
                }
                TypePattern::Option(x) => {
                    if self.should_emit(x.meta()) {
                        self.write_type_definition_composite(w, x)?;
                        w.newline()?;
                    }
                }
            },
        }
        Ok(())
    }

    fn write_type_definition_fn_pointer(&self, w: &mut IndentWriter, the_type: &FnPointerType) -> Result<(), Error> {
        self.write_type_definition_fn_pointer_annotation(w, the_type)?;
        self.write_type_definition_fn_pointer_body(w, the_type)?;
        Ok(())
    }

    fn write_type_definition_fn_pointer_annotation(&self, w: &mut IndentWriter, _the_type: &FnPointerType) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"[UnmanagedFunctionPointer(CallingConvention.Cdecl)]"#))?;
        Ok(())
    }

    fn write_type_definition_fn_pointer_body(&self, w: &mut IndentWriter, the_type: &FnPointerType) -> Result<(), Error> {
        w.indented(|w| write!(w, "public delegate {} ", self.type_to_typespecifier_in_rval(the_type.signature().rval())))?;
        write!(w.writer(), "{}(", self.type_fnpointer_to_typename(the_type))?;

        let params = the_type.signature().params();
        for (i, param) in params.iter().enumerate() {
            write!(w.writer(), "{} x{}", self.type_to_typespecifier_in_param(param.the_type()), i)?;

            if i < params.len() - 1 {
                write!(w.writer(), ", ")?;
            }
        }

        writeln!(w.writer(), ");")?;
        Ok(())
    }

    fn write_type_definition_enum(&self, w: &mut IndentWriter, the_type: &EnumType) -> Result<(), Error> {
        self.write_documentation(w, the_type.meta().documentation())?;
        w.indented(|w| writeln!(w, r#"public enum {}"#, the_type.rust_name()))?;
        w.indented(|w| writeln!(w, r#"{{"#))?;
        w.indent();

        for variant in the_type.variants() {
            self.write_type_definition_enum_variant(w, variant, the_type)?;
        }

        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;
        Ok(())
    }

    fn write_type_definition_enum_variant(&self, w: &mut IndentWriter, variant: &Variant, _the_type: &EnumType) -> Result<(), Error> {
        let variant_name = variant.name();
        let variant_value = variant.value();
        self.write_documentation(w, variant.documentation())?;
        w.indented(|w| writeln!(w, r#"{} = {},"#, variant_name, variant_value))?;
        Ok(())
    }

    fn write_type_definition_composite(&self, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
        self.write_documentation(w, the_type.meta().documentation())?;
        self.write_type_definition_composite_annotation(w, the_type)?;
        self.write_type_definition_composite_body(w, the_type)?;
        Ok(())
    }

    fn write_type_definition_composite_annotation(&self, w: &mut IndentWriter, _the_type: &CompositeType) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"[Serializable]"#))?;
        w.indented(|w| writeln!(w, r#"[StructLayout(LayoutKind.Sequential)]"#))?;

        Ok(())
    }

    fn write_type_definition_composite_body(&self, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"public partial struct {}"#, the_type.rust_name()))?;
        w.indented(|w| writeln!(w, r#"{{"#))?;
        w.indent();

        for field in the_type.fields() {
            self.write_documentation(w, field.documentation())?;
            self.write_type_definition_composite_body_field(w, field, the_type)?;
        }

        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;
        Ok(())
    }

    fn write_type_definition_composite_body_field(&self, w: &mut IndentWriter, field: &Field, the_type: &CompositeType) -> Result<(), Error> {
        let field_name = field.name();
        let type_name = self.type_to_typespecifier_in_field(field.the_type(), field, the_type);
        w.indented(|w| writeln!(w, r#"public {} {};"#, type_name, field_name))?;
        Ok(())
    }

    fn namespace_for_id(&self, id: &str) -> String {
        self.config()
            .namespace_mappings
            .get(id)
            .unwrap_or_else(|| panic!("Found a namespace not mapped '{}'. You should specify this one in the config.", id))
            .to_string()
    }

    fn write_namespace_context(&self, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"namespace {}"#, self.namespace_for_id(&self.config().namespace_id)))?;
        w.indented(|w| writeln!(w, r#"{{"#))?;
        w.indent();

        f(w)?;

        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;

        Ok(())
    }

    fn write_class_context(&self, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"public static partial class {}"#, self.config().class))?;
        w.indented(|w| writeln!(w, r#"{{"#))?;
        w.indent();

        f(w)?;

        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;

        Ok(())
    }

    fn should_emit_delegate(&self) -> bool {
        self.config().namespace_id.is_empty()
    }

    fn has_emittable_functions(&self, functions: &[Function]) -> bool {
        functions.iter().any(|x| self.should_emit(x.meta()))
    }

    fn should_emit(&self, meta: &Meta) -> bool {
        let rval = meta.namespace() == self.config().namespace_id;
        rval
    }

    fn write_patterns(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for pattern in self.library().patterns() {
            match pattern {
                LibraryPattern::Class(cls) => {
                    if self.should_emit(cls.the_type().meta()) {
                        self.write_pattern_class(w, cls)?
                    }
                }
            }
        }

        Ok(())
    }

    fn write_pattern_slice(&self, w: &mut IndentWriter, slice: &CompositeType) -> Result<(), Error> {
        let context_type_name = slice.rust_name();
        let data_type = slice
            .fields()
            .iter()
            .find(|x| x.name().contains("data"))
            .expect("Slice must contain field called 'data'.")
            .the_type()
            .deref_pointer()
            .expect("data must be a pointer type");

        let type_string = self.type_to_typespecifier_in_rval(data_type);

        w.indented(|w| writeln!(w, r#"public partial struct {}"#, context_type_name))?;
        w.indented(|w| writeln!(w, r#"{{"#))?;
        w.indent();
        w.indented(|w| writeln!(w, r#"public {} this[int i]"#, type_string))?;
        w.indented(|w| writeln!(w, r#"{{"#))?;
        w.indent();
        w.indented(|w| writeln!(w, r#"get"#))?;
        w.indented(|w| writeln!(w, r#"{{"#))?;
        w.indent();
        w.indented(|w| writeln!(w, r#"var size = Marshal.SizeOf(typeof({}));"#, type_string))?;
        w.indented(|w| writeln!(w, r#"var ptr = new IntPtr(data.ToInt64() + i * size);"#))?;
        w.indented(|w| writeln!(w, r#"return  Marshal.PtrToStructure<{}>(ptr);"#, type_string))?;
        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;
        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;

        w.indented(|w| writeln!(w, r#"public {}[] Copied"#, type_string))?;
        w.indented(|w| writeln!(w, r#"{{"#))?;
        w.indent();
        w.indented(|w| writeln!(w, r#"get"#))?;
        w.indented(|w| writeln!(w, r#"{{"#))?;
        w.indent();
        w.indented(|w| writeln!(w, r#"var rval = new {}[len];"#, type_string))?;
        w.indented(|w| writeln!(w, r#"for (var i = 0; i < (int) len; i++) {{"#))?;
        w.indent();
        w.indented(|w| writeln!(w, r#"rval[i] = this[i];"#))?;
        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;
        w.indented(|w| writeln!(w, r#"return rval;"#))?;
        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;
        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;

        w.indented(|w| writeln!(w, r#"public int Count"#))?;
        w.indented(|w| writeln!(w, r#"{{"#))?;
        w.indent();
        w.indented(|w| writeln!(w, r#"get"#))?;
        w.indented(|w| writeln!(w, r#"{{"#))?;
        w.indent();
        w.indented(|w| writeln!(w, r#"return (int) len;"#))?;
        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;
        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;

        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;
        w.newline()?;

        Ok(())
    }

    fn write_pattern_class(&self, w: &mut IndentWriter, class: &Service) -> Result<(), Error> {
        let context_type_name = class.the_type().rust_name();

        let mut all_functions = vec![class.constructor().clone(), class.destructor().clone()];
        all_functions.extend_from_slice(class.methods());
        let common_prefix = longest_common_prefix(&all_functions);

        self.write_documentation(w, class.the_type().meta().documentation())?;
        w.indented(|w| writeln!(w, r#"public partial class {} : IDisposable"#, context_type_name))?;
        w.indented(|w| writeln!(w, r#"{{"#))?;
        w.indent();
        w.indented(|w| writeln!(w, r#"private IntPtr _context;"#))?;

        // Ctor
        let args = self.pattern_class_args_without_first_to_string(class.constructor(), true);
        self.write_documentation(w, class.constructor().meta().documentation())?;
        w.indented(|w| writeln!(w, r#"public {}({})"#, context_type_name, args))?;
        w.indented(|w| writeln!(w, r#"{{"#))?;
        w.indent();
        self.write_pattern_class_success_enum_aware_rval(w, class, class.constructor(), false)?;
        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;
        w.newline()?;

        // Dtor
        w.indented(|w| writeln!(w, r#"public void Dispose()"#))?;
        w.indented(|w| writeln!(w, r#"{{"#))?;
        w.indent();
        self.write_pattern_class_success_enum_aware_rval(w, class, class.destructor(), false)?;
        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;
        w.newline()?;

        for function in class.methods() {
            let args = self.pattern_class_args_without_first_to_string(function, true);
            let without_common_prefix = function.name().replace(&common_prefix, "");
            let prettified = IdPrettifier::from_rust_lower(&without_common_prefix);
            let rval = match function.signature().rval() {
                CType::Pattern(TypePattern::SuccessEnum(_)) => "void".to_string(),
                _ => self.type_to_typespecifier_in_rval(function.signature().rval()),
            };

            self.write_documentation(w, function.meta().documentation())?;

            w.indented(|w| writeln!(w, r#"public {} {}({})"#, rval, prettified.to_camel_case(), &args))?;
            w.indented(|w| writeln!(w, r#"{{"#))?;
            w.indent();
            self.write_pattern_class_success_enum_aware_rval(w, class, function, true)?;
            w.unindent();
            w.indented(|w| writeln!(w, r#"}}"#))?;
            w.newline()?;
        }

        w.unindent();
        w.indented(|w| writeln!(w, r#"}}"#))?;
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_pattern_class_success_enum_aware_rval(&self, w: &mut IndentWriter, _class: &Service, function: &Function, deref_context: bool) -> Result<(), Error> {
        let mut args = self.pattern_class_args_without_first_to_string(function, false);

        // Make sure we don't have a `,` when only single parameter
        if !args.is_empty() {
            args = format!(", {}", args);
        }

        let context = if deref_context { "_context".to_string() } else { "out _context".to_string() };

        match function.signature().rval() {
            CType::Pattern(TypePattern::SuccessEnum(e)) => {
                w.indented(|w| writeln!(w, r#"var rval = {}.{}({} {});"#, self.config().class, function.name(), context, args))?;
                w.indented(|w| writeln!(w, r#"if (rval != {}.{})"#, e.the_enum().rust_name(), e.success_variant().name()))?;
                w.indented(|w| writeln!(w, r#"{{"#))?;
                w.indent();
                w.indented(|w| writeln!(w, r#"throw new Exception("Something went wrong");"#))?;
                w.unindent();
                w.indented(|w| writeln!(w, r#"}}"#))?;
            }
            CType::Primitive(PrimitiveType::Void) => {
                w.indented(|w| writeln!(w, r#"{}.{}({} {});"#, self.config().class, function.name(), context, args))?;
            }
            _ => {
                w.indented(|w| writeln!(w, r#"return {}.{}({} {});"#, self.config().class, function.name(), context, args))?;
            }
        }

        Ok(())
    }

    fn pattern_class_args_without_first_to_string(&self, function: &Function, with_types: bool) -> String {
        function
            .signature()
            .params()
            .iter()
            .skip(1)
            .map(|x| {
                format!(
                    "{} {}",
                    if with_types {
                        self.type_to_typespecifier_in_param(x.the_type())
                    } else {
                        "".to_string()
                    },
                    x.name().to_string()
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl Interop for Generator {
    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_file_header_comments(w)?;
        w.newline()?;

        self.write_imports(w)?;
        w.newline()?;

        self.write_namespace_context(w, |w| {
            if self.has_emittable_functions(self.library().functions()) {
                self.write_class_context(w, |w| {
                    self.write_native_lib_string(w)?;
                    w.newline()?;

                    self.write_constants(w)?;
                    w.newline()?;

                    self.write_functions(w)?;
                    Ok(())
                })?;
            }

            w.newline()?;
            self.write_type_definitions(w)?;

            w.newline()?;
            self.write_patterns(w)?;

            Ok(())
        })?;

        Ok(())
    }
}

impl InteropCSharp for Generator {
    fn config(&self) -> &Config {
        &self.config
    }

    fn library(&self) -> &Library {
        &self.library
    }
}
