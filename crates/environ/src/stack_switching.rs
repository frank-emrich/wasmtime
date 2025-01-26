//! TODO(dhil): Write documentation for this module.
use core::{
    convert::From,
    default::Default,
    marker::{Send, Sync},
};

/// TODO
#[allow(dead_code, reason = "Only accessed in debug builds")]
pub const ENABLE_DEBUG_PRINTING: bool = false;

/// TODO(dhil): Write documentation.
#[macro_export]
macro_rules! debug_println {
    ($( $args:expr ),+ ) => {
        #[cfg(debug_assertions)]
        if ENABLE_DEBUG_PRINTING {
            println!($($args),*);
        }
    }
}

/// Runtime configuration options for stack switching that can be set
/// via the command line.
///
/// Part of wasmtime::config::Config type (which is not in scope in this crate).
#[derive(Debug, Clone)]
pub struct StackSwitchingConfig {
    /// The (fixed) size of a continuation stack.
    pub stack_size: usize,
}

impl Default for StackSwitchingConfig {
    fn default() -> Self {
        /// Default size for continuation stacks
        const DEFAULT_FIBER_SIZE: usize = 2097152; // 2MB = 512 pages of 4k

        Self {
            stack_size: DEFAULT_FIBER_SIZE,
        }
    }
}

/// This type is used to save (and subsequently restore) a subset of the data in
/// `VMRuntimeLimits`. See documentation of `StackChain` for the exact uses.
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct StackLimits {
    /// TODO(dhil): Write documentation.
    pub stack_limit: usize,
    /// TODO(dhil): Write documentation.
    pub last_wasm_entry_fp: usize,
}

/// This type represents "common" information that we need to save both for the
/// initial stack and each continuation.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CommonStackInformation {
    /// TODO(dhil): Write documentation.
    pub limits: StackLimits,
    /// For the initial stack, this field must only have one of the following values:
    /// - Running
    /// - Parent
    pub state: State,

    /// Only in use when state is `Parent`. Otherwise, the list must be empty.
    ///
    /// Represents the handlers that this stack installed when resume-ing a
    /// continuation.
    ///
    /// Note that for any resume instruction, we can re-order the handler
    /// clauses without changing behavior such that all the suspend handlers
    /// come first, followed by all the switch handler (while maintaining the
    /// original ordering within the two groups).
    /// Thus, we assume that the given resume instruction has the following
    /// shape:
    ///
    /// (resume $ct
    ///   (on $tag_0 $block_0) ... (on $tag_{n-1} $block_{n-1})
    ///   (on $tag_n switch) ... (on $tag_m switch)
    /// )
    ///
    /// On resume, the handler list is then filled with m + 1 (i.e., one per
    /// handler clause) entries such that the i-th entry, using 0-based
    /// indexing, is the identifier of $tag_i (represented as *mut
    /// VMTagDefinition).
    /// Further, `first_switch_handler_index` (see below) is set to n (i.e., the
    /// 0-based index of the first switch handler).
    ///
    /// Note that the actual data buffer (i.e., the one `handler.data` points
    /// to) is always allocated on the stack that this `CommonStackInformation`
    /// struct describes.
    pub handlers: HandlerList,

    /// Only used when state is `Parent`. See documentation of `handlers` above.
    pub first_switch_handler_index: u32,
}

impl CommonStackInformation {
    /// TODO(dhil): Write documentation.
    pub fn running_default() -> Self {
        Self {
            limits: StackLimits::default(),
            state: State::Running,
            handlers: HandlerList::empty(),
            first_switch_handler_index: 0,
        }
    }
}

impl StackLimits {
    /// TODO(dhil): Write documentation.
    pub fn with_stack_limit(stack_limit: usize) -> Self {
        Self {
            stack_limit,
            ..Default::default()
        }
    }
}

// Since `StackLimits` objects appear in the `StoreOpaque`,
// they need to be `Send` and `Sync`.
// This is safe for the same reason it is for `VMRuntimeLimits` (see comment
// there): Both types are pod-type with no destructor, and we don't access any
// of their fields from other threads.
unsafe impl Send for StackLimits {}
unsafe impl Sync for StackLimits {}

// Same for HandlerList: They appear in the `StoreOpaque`.
unsafe impl Send for HandlerList {}
unsafe impl Sync for HandlerList {}

#[repr(C)]
#[derive(Debug, Clone)]
/// Reference to a stack-allocated buffer ("array"), storing data of some type
/// `T`.
pub struct Array<T> {
    /// Number of currently occupied slots.
    pub length: u32,
    /// Number of slots in the data buffer. Note that this is *not* the size of
    /// the buffer in bytes!
    pub capacity: u32,
    /// The actual data buffer
    pub data: *mut T,
}

impl<T> Array<T> {
    /// Creates empty `Array`
    pub fn empty() -> Self {
        Self {
            length: 0,
            capacity: 0,
            data: std::ptr::null_mut(),
        }
    }

    /// Makes `Array` empty.
    pub fn clear(&mut self) {
        *self = Self::empty();
    }
}

/// Type used for passing payloads to and from continuations. The actual type
/// argument should be wasmtime::runtime::vm::vmcontext::ValRaw, but we don't
/// have access to that here.
pub type Payloads = Array<u128>;

/// Type for a list of handlers, represented by the handled tag. Thus, the
/// stored data is actually `*mut VMTagDefinition`, but we don't havr access to
/// that here.
pub type HandlerList = Array<*mut u8>;

/// Discriminant of variant `Absent` in
/// `wasmtime_runtime::continuation::StackChain`.
pub const STACK_CHAIN_ABSENT_DISCRIMINANT: usize = 0;
/// Discriminant of variant `InitialStack` in
/// `wasmtime_runtime::continuation::StackChain`.
pub const STACK_CHAIN_INITIAL_STACK_DISCRIMINANT: usize = 1;
/// Discriminant of variant `Continiation` in
/// `wasmtime_runtime::continuation::StackChain`.
pub const STACK_CHAIN_CONTINUATION_DISCRIMINANT: usize = 2;

/// Encodes the life cycle of a `VMContRef`.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum State {
    /// The `VMContRef` has been created, but neither `resume` or `switch` has ever been
    /// called on it. During this stage, we may add arguments using `cont.bind`.
    Fresh,
    /// The continuation is running, meaning that it is the one currently
    /// executing code.
    Running,
    /// The continuation is suspended because it executed a resume instruction
    /// that has not finished yet. In other words, it became the parent of
    /// another continuation (which may itself be `Running`, a `Parent`, or
    /// `Suspended`).
    Parent,
    /// The continuation was suspended by a `suspend` or `switch` instruction.
    Suspended,
    /// The function originally passed to `cont.new` has returned normally.
    /// Note that there is no guarantee that a VMContRef will ever
    /// reach this status, as it may stay suspended until being dropped.
    Returned,
}

impl State {
    /// TODO(dhil): Write documentation.
    pub fn discriminant(&self) -> i32 {
        // This is well-defined for an enum with repr(i32).
        unsafe { *(self as *const Self as *const i32) }
    }
}

impl From<State> for i32 {
    fn from(st: State) -> Self {
        st.discriminant()
    }
}

/// Defines offsets of the fields in the continuation-related types
pub mod offsets {
    /// Offsets of fields in `Array`.
    /// Note that these are independent from the type parameter `T`.
    pub mod array {
        use crate::stack_switching::*;
        use memoffset::offset_of;

        /// Offset of `capacity` field
        pub const CAPACITY: usize = offset_of!(Array<()>, capacity);
        /// Offset of `data` field
        pub const DATA: usize = offset_of!(Array<()>, data);
        /// Offset of `length` field
        pub const LENGTH: usize = offset_of!(Array<()>, length);
    }

    /// Offsets of fields in `wasmtime_runtime::continuation::VMContRef`.
    /// We uses tests there to ensure these values are correct.
    pub mod vm_cont_ref {
        use crate::stack_switching::*;

        /// Offset of `common_stack_information` field
        pub const COMMON_STACK_INFORMATION: usize = 0;
        /// Offset of `parent_chain` field
        pub const PARENT_CHAIN: usize =
            COMMON_STACK_INFORMATION + core::mem::size_of::<CommonStackInformation>();
        /// Offset of `last_ancestor` field
        pub const LAST_ANCESTOR: usize = PARENT_CHAIN + 2 * core::mem::size_of::<usize>();
        /// Offset of `stack` field
        pub const STACK: usize = LAST_ANCESTOR + core::mem::size_of::<usize>();
        /// Offset of `args` field
        pub const ARGS: usize = STACK + super::FIBER_STACK_SIZE;
        /// Offset of `values` field
        pub const VALUES: usize = ARGS + core::mem::size_of::<Payloads>();
        /// Offset of `revision` field
        pub const REVISION: usize = VALUES + core::mem::size_of::<Payloads>();
    }

    /// TODO(dhil): Write documentation.
    pub mod stack_limits {
        use crate::stack_switching::*;
        use memoffset::offset_of;

        /// TODO(dhil): Write documentation.
        pub const STACK_LIMIT: usize = offset_of!(StackLimits, stack_limit);
        /// TODO(dhil): Write documentation.
        pub const LAST_WASM_ENTRY_FP: usize = offset_of!(StackLimits, last_wasm_entry_fp);
    }

    /// TODO(dhil): Write documentation.
    pub mod common_stack_information {
        use crate::stack_switching::*;
        use memoffset::offset_of;

        /// TODO(dhil): Write documentation.
        pub const LIMITS: usize = offset_of!(CommonStackInformation, limits);
        /// TODO(dhil): Write documentation.
        pub const STATE: usize = offset_of!(CommonStackInformation, state);
        /// TODO(dhil): Write documentation.
        pub const HANDLERS: usize = offset_of!(CommonStackInformation, handlers);
        /// TODO(dhil): Write documentation.
        pub const FIRST_SWITCH_HANDLER_INDEX: usize =
            offset_of!(CommonStackInformation, first_switch_handler_index);
    }

    /// Size of wasmtime_runtime::continuation::FiberStack.
    /// We test there that this value is correct.
    pub const FIBER_STACK_SIZE: usize = 3 * core::mem::size_of::<usize>();

    /// Size of type `wasmtime_runtime::continuation::StackChain`.
    /// We test there that this value is correct.
    pub const STACK_CHAIN_SIZE: usize = 2 * core::mem::size_of::<usize>();
}

/// Discriminant of variant `Return` in
/// `ControlEffect`.
pub const CONTROL_EFFECT_RETURN_DISCRIMINANT: u32 = 0;
/// Discriminant of variant `Resume` in
/// `ControlEffect`.
pub const CONTROL_EFFECT_RESUME_DISCRIMINANT: u32 = 1;
/// Discriminant of variant `Suspend` in
/// `ControlEffect`.
pub const CONTROL_EFFECT_SUSPEND_DISCRIMINANT: u32 = 2;
/// Discriminant of variant `Switch` in
/// `ControlEffect`.
pub const CONTROL_EFFECT_SWITCH_DISCRIMINANT: u32 = 3;

/// Universal control effect. This structure encodes return signal,
/// resume signal, suspension signal, and the handler to suspend to in a single variant type.
/// This instance is used at runtime. There is a codegen
/// counterpart in `cranelift/src/stack-switching/control_effect.rs`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ControlEffect {
    /// TODO(dhil): Write documentation.
    Return = CONTROL_EFFECT_RETURN_DISCRIMINANT,
    /// TODO(dhil): Write documentation.
    Resume = CONTROL_EFFECT_RESUME_DISCRIMINANT,
    /// TODO(dhil): Write documentation.
    Suspend {
        /// TODO(dhil): Write documentation.
        handler_index: u32,
    } = CONTROL_EFFECT_SUSPEND_DISCRIMINANT,
    /// TODO(dhil): Write documentation.
    Switch = CONTROL_EFFECT_SWITCH_DISCRIMINANT,
}

// TODO(frank-emrich) This conversion assumes little-endian data layout.
// We convert to and from u64 as follows: The 4 LSBs of the u64 are the
// discriminant, the 4 MSBs are the handler_index (if `Suspend`)
impl From<u64> for ControlEffect {
    fn from(val: u64) -> ControlEffect {
        unsafe { core::mem::transmute::<u64, ControlEffect>(val) }
    }
}

// TODO(frank-emrich) This conversion assumes little-endian data layout.
// We convert to and from u64 as follows: The 4 LSBs of the u64 are the
// discriminant, the 4 MSBs are the handler_index (if `Suspend`)
impl From<ControlEffect> for u64 {
    fn from(val: ControlEffect) -> u64 {
        unsafe { core::mem::transmute::<ControlEffect, u64>(val) }
    }
}
