//! Binding to [TensorFlow][1].
//!
//! [1]: https://www.tensorflow.org

#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_char, c_uchar, c_int, c_void, int64_t, size_t, c_float};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TF_Buffer {
    pub data: *const c_void,
    pub length: size_t,
    pub data_deallocator: Option<unsafe extern "C" fn(data: *mut c_void, length: size_t)>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TF_Code {
    TF_OK = 0,
    TF_CANCELLED = 1,
    TF_UNKNOWN = 2,
    TF_INVALID_ARGUMENT = 3,
    TF_DEADLINE_EXCEEDED = 4,
    TF_NOT_FOUND = 5,
    TF_ALREADY_EXISTS = 6,
    TF_PERMISSION_DENIED = 7,
    TF_UNAUTHENTICATED = 16,
    TF_RESOURCE_EXHAUSTED = 8,
    TF_FAILED_PRECONDITION = 9,
    TF_ABORTED = 10,
    TF_OUT_OF_RANGE = 11,
    TF_UNIMPLEMENTED = 12,
    TF_INTERNAL = 13,
    TF_UNAVAILABLE = 14,
    TF_DATA_LOSS = 15,
}
pub use TF_Code::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TF_DataType {
    TF_FLOAT = 1,
    TF_DOUBLE = 2,
    TF_INT32 = 3,
    TF_UINT8 = 4,
    TF_INT16 = 5,
    TF_INT8 = 6,
    TF_STRING = 7,
    TF_COMPLEX64 = 8,
    TF_INT64 = 9,
    TF_BOOL = 10,
    TF_QINT8 = 11,
    TF_QUINT8 = 12,
    TF_QINT32 = 13,
    TF_BFLOAT16 = 14,
    TF_QINT16 = 15,
    TF_QUINT16 = 16,
    TF_UINT16 = 17,
    TF_COMPLEX128 = 18,
    TF_HALF = 19,
}
pub use TF_DataType::*;

#[derive(Clone, Copy, Debug)]
pub enum TF_Library {}

#[derive(Clone, Copy, Debug)]
#[deprecated(since="0.5.0", note="Use TF_SessionWithGraph instead.")]
pub enum TF_Session {}

#[derive(Clone, Copy, Debug)]
pub enum TF_SessionOptions {}

#[derive(Clone, Copy, Debug)]
pub enum TF_Status {}

#[derive(Clone, Copy, Debug)]
pub enum TF_Tensor {}

#[derive(Clone, Copy, Debug)]
pub enum TF_Graph {}

#[derive(Clone, Copy, Debug)]
pub enum TF_OperationDescription {}

#[derive(Clone, Copy, Debug)]
pub enum TF_Operation {}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct TF_Port {
    pub operation: *mut TF_Operation,
    pub index: c_int,
}

#[derive(Clone, Copy, Debug)]
pub enum TF_SessionWithGraph {}

extern "C" {
    pub fn TF_NewBuffer() -> *mut TF_Buffer;
    pub fn TF_NewBufferFromString(proto: *const c_void, length: size_t) -> *mut TF_Buffer;
    pub fn TF_DeleteBuffer(buffer: *mut TF_Buffer);
    pub fn TF_GetBuffer(buffer: *mut TF_Buffer) -> TF_Buffer;
}

extern "C" {
    pub fn TF_LoadLibrary(name: *const c_char, status: *mut TF_Status) -> *mut TF_Library;
    pub fn TF_GetOpList(library: *mut TF_Library) -> TF_Buffer;
}

extern "C" {
    #[allow(deprecated)]
    #[deprecated(since="0.5.0", note="Use TF_NewSessionWithGraph instead.")]
    pub fn TF_NewSession(options: *const TF_SessionOptions, status: *mut TF_Status)
                         -> *mut TF_Session;
    #[allow(deprecated)]
    #[deprecated(since="0.5.0", note="Use TF_DeleteSessionWithGraph instead.")]
    pub fn TF_DeleteSession(session: *mut TF_Session, status: *mut TF_Status);
    pub fn TF_Reset(opt: *const TF_SessionOptions, containers: *const *const c_char,
                    ncontainers: c_int, status: *mut TF_Status);
    #[allow(deprecated)]
    #[deprecated(since="0.5.0", note="Use TF_CloseSessionWithGraph instead.")]
    pub fn TF_CloseSession(session: *mut TF_Session, status: *mut TF_Status);
    #[allow(deprecated)]
    #[deprecated(since="0.5.0", note="Use TF_SessionWithGraph API instead.")]
    pub fn TF_ExtendGraph(session: *mut TF_Session, proto: *const c_void, length: size_t,
                          status: *mut TF_Status);
    #[allow(deprecated)]
    #[deprecated(since="0.5.0", note="Use TF_SessionRun instead.")]
    pub fn TF_Run(session: *mut TF_Session, run_options: *const TF_Buffer,
                  input_names: *mut *const c_char, inputs: *mut *mut TF_Tensor, ninputs: c_int,
                  output_names: *mut *const c_char, outputs: *mut *mut TF_Tensor, noutputs: c_int,
                  target_names: *mut *const c_char, ntargets: c_int, run_metadata: *mut TF_Buffer,
                  status: *mut TF_Status);
    #[allow(deprecated)]
    #[deprecated(since="0.5.0", note="Use TF_SessionPRunSetup instead.")]
    pub fn TF_PRunSetup(session: *mut TF_Session, input_names: *mut *const c_char, ninputs: c_int,
                        output_names: *mut *const c_char, noutputs: c_int,
                        target_names: *mut *const c_char, ntargets: c_int,
                        handle: *mut *const c_char, status: *mut TF_Status);
    #[allow(deprecated)]
    #[deprecated(since="0.5.0", note="Use TF_SessionPRun instead.")]
    pub fn TF_PRun(session: *mut TF_Session, handle: *const c_char,
                   input_names: *mut *const c_char, inputs: *mut *mut TF_Tensor, ninputs: c_int,
                   output_names: *mut *const c_char, outputs: *mut *mut TF_Tensor, noutputs: c_int,
                   target_names: *mut *const c_char, ntargets: c_int, status: *mut TF_Status);
}

extern "C" {
    pub fn TF_NewSessionOptions() -> *mut TF_SessionOptions;
    pub fn TF_DeleteSessionOptions(options: *mut TF_SessionOptions);
    pub fn TF_SetTarget(options: *mut TF_SessionOptions, target: *const c_char);
    pub fn TF_SetConfig(options: *mut TF_SessionOptions, proto: *const c_void, length: size_t,
                        status: *mut TF_Status);
}

extern "C" {
    pub fn TF_NewStatus() -> *mut TF_Status;
    pub fn TF_DeleteStatus(status: *mut TF_Status);
    pub fn TF_SetStatus(status: *mut TF_Status, code: TF_Code, message: *const c_char);
    pub fn TF_GetCode(status: *const TF_Status) -> TF_Code;
    pub fn TF_Message(status: *const TF_Status) -> *const c_char;
}

extern "C" {
    pub fn TF_NewTensor(datatype: TF_DataType, dims: *const int64_t, ndims: c_int,
                        data: *mut c_void, length: size_t,
                        deallocator: Option<unsafe extern "C" fn(data: *mut c_void,
                                                                 length: size_t,
                                                                 arg: *mut c_void)>,
                        deallocator_arg: *mut c_void) -> *mut TF_Tensor;
    pub fn TF_DeleteTensor(tensor: *mut TF_Tensor);
    pub fn TF_TensorType(tensor: *const TF_Tensor) -> TF_DataType;
    pub fn TF_NumDims(tensor: *const TF_Tensor) -> c_int;
    pub fn TF_Dim(tensor: *const TF_Tensor, index: c_int) -> int64_t;
    pub fn TF_TensorByteSize(tensor: *const TF_Tensor) -> size_t;
    pub fn TF_TensorData(tensor: *const TF_Tensor) -> *mut c_void;
}

extern "C" {
    pub fn TF_NewGraph() -> *mut TF_Graph;
    pub fn TF_DeleteGraph(graph: *mut TF_Graph);
    pub fn TF_NewOperation(
        graph: *mut TF_Graph,
        op_type: *const c_char,
        operation_name: *const c_char) -> *mut TF_OperationDescription;
    pub fn TF_SetDevice(desc: *mut TF_OperationDescription, device: *const c_char);
    pub fn TF_AddInput(desc: *mut TF_OperationDescription, input: TF_Port);
    pub fn TF_AddInputList(
        desc: *mut TF_OperationDescription,
        inputs: *const TF_Port,
        num_inputs: c_int);
    pub fn TF_AddControlInput(desc: *mut TF_OperationDescription, input: *mut TF_Operation);
    pub fn TF_SetAttrString(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        value: *const c_void,
        length: c_int);
    pub fn TF_SetAttrStringList(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        values: *const *const c_void,
        lengths: *const c_int,
        num_values: c_int);
    pub fn TF_SetAttrInt(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        value: int64_t);
    pub fn TF_SetAttrIntList(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        values: *const int64_t,
        num_values: c_int);
    pub fn TF_SetAttrFloat(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        value: c_float);
    pub fn TF_SetAttrFloatList(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        values: *const c_float,
        num_values: c_int);
    pub fn TF_SetAttrBool(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        value: c_uchar);
    pub fn TF_SetAttrBoolList(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        values: *const c_uchar,
        num_values: c_int);
    pub fn TF_SetAttrType(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        value: TF_DataType);
    pub fn TF_SetAttrTypeList(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        values: *const TF_DataType,
        num_values: c_int);
    pub fn TF_SetAttrShape(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        dims: *const int64_t,
        num_dims: c_int);
    pub fn TF_SetAttrShapeList(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        dims: *const *const int64_t,
        num_dims: *const c_int,
        num_shapes: c_int);
    pub fn TF_SetAttrTensorShapeProto(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        proto: *const c_void,
        proto_len: c_int,
        status: *mut TF_Status);
    pub fn TF_SetAttrTensorShapeProtoList(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        protos: *const *const c_void,
        proto_lens: *const c_int,
        num_shapes: c_int,
        status: *mut TF_Status);
    pub fn TF_SetAttrTensor(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        value: *mut TF_Tensor,
        status: *mut TF_Status);
    pub fn TF_SetAttrTensorList(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        values: *const *mut TF_Tensor,
        num_values: c_int,
        status: *mut TF_Status);
    pub fn TF_SetAttrToAttrValueProto(
        desc: *mut TF_OperationDescription,
        attr_name: *const c_char,
        proto: *const c_void,
        proto_len: size_t,
        status: *mut TF_Status);
    pub fn TF_FinishOperation(desc: *mut TF_OperationDescription, status: *mut TF_Status) -> *mut TF_Operation;
    pub fn TF_OperationName(operation: *mut TF_Operation) -> *const c_char;
    pub fn TF_OperationOpType(operation: *mut TF_Operation) -> *const c_char;
    pub fn TF_OperationDevice(operation: *mut TF_Operation) -> *const c_char;
    pub fn TF_OperationNumOutputs(operation: *mut TF_Operation) -> c_int;
    pub fn TF_OperationOutputType(operation_out: TF_Port) -> TF_DataType;
    pub fn TF_OperationOutputListLength(
        operation: *mut TF_Operation,
        arg_name: *const c_char,
        status: *mut TF_Status) -> c_int;
    pub fn TF_OperationNumInputs(operation: *mut TF_Operation) -> c_int;
    pub fn TF_OperationInputType(operation_in: TF_Port) -> TF_DataType;
    pub fn TF_OperationInputListLength(
        operation: *mut TF_Operation,
        arg_name: *const c_char,
        status: *mut TF_Status) -> c_int;
    pub fn TF_OperationInput(operation_in: TF_Port) -> TF_Port;
    pub fn TF_OperationOutputNumConsumers(operation_out: TF_Port) -> c_int;
    pub fn TF_OperationOutputConsumers(
        operation_out: TF_Port,
        consumers: *mut TF_Port,
        max_consumers: c_int) -> c_int;
    pub fn TF_OperationNumControlInputs(operation: *mut TF_Operation) -> c_int;
    pub fn TF_OperationGetControlInputs(
        operation: *mut TF_Operation,
        control_inputs: *mut *mut TF_Operation,
        max_control_inputs: c_int) -> c_int;
    pub fn TF_OperationNumControlOutputs(operation: *mut TF_Operation) -> c_int;
    pub fn TF_OperationGetControlOutputs(
        operation: *mut TF_Operation,
        control_outputs: *mut *mut TF_Operation,
        max_control_outputs: c_int) -> c_int;
    pub fn TF_OperationGetAttrValueProto(
        operation: *mut TF_Operation,
        attr_name: *const c_char,
        output_attr_value: *mut TF_Buffer,
        status: *mut TF_Status);
    pub fn TF_GraphOperationByName(graph: *mut TF_Graph, operation_name: *const c_char) -> *mut TF_Operation;
    pub fn TF_GraphNextOperation(graph: *mut TF_Graph, pos: *mut size_t) -> *mut TF_Operation;
    pub fn TF_GraphToGraphDef(
        graph: *mut TF_Graph,
        output_graph_def: *mut TF_Buffer,
        status: *mut TF_Status);
    pub fn TF_OperationToNodeDef(
        operation: *mut TF_Operation,
        output_operation_def: *mut TF_Buffer,
        status: *mut TF_Status);
}

extern "C" {
    pub fn TF_NewSessionWithGraph(
        graph: *mut TF_Graph,
        opts: *const TF_SessionOptions,
        status: *mut TF_Status) -> *mut TF_SessionWithGraph;
    pub fn TF_CloseSessionWithGraph(session: *mut TF_SessionWithGraph, status: *mut TF_Status);
    pub fn TF_DeleteSessionWithGraph(session: *mut TF_SessionWithGraph, status: *mut TF_Status);
    pub fn TF_SessionRun(
        session: *mut TF_SessionWithGraph,
        run_options: *const TF_Buffer,
        inputs: *const TF_Port,
        input_values: *const *mut TF_Tensor,
        ninputs: c_int,
        outputs: *const TF_Port,
        output_values: *mut *mut TF_Tensor,
        noutputs: c_int,
        target_operations: *const *const TF_Operation,
        ntargets: c_int,
        run_metadata: *mut TF_Buffer,
        status: *mut TF_Status);
    pub fn TF_SessionPRunSetup(
        session: *mut TF_SessionWithGraph,
        inputs: *const TF_Port,
        ninputs: c_int,
        outputs: *const TF_Port,
        noutputs: c_int,
        target_operations: *const *const TF_Operation,
        ntargets: c_int,
        handle: *const *const c_char,
        status: *mut TF_Status);
    pub fn TF_SessionPRun(
        session: *mut TF_SessionWithGraph,
        handle: *const c_char,
        inputs: *const TF_Port,
        input_values: *const *mut TF_Tensor,
        ninputs: c_int,
        outputs: *const TF_Port,
        output_values: *mut *mut TF_Tensor,
        noutputs: c_int,
        target_operations: *const *const TF_Operation,
        ntargets: c_int,
        status: *mut TF_Status);
}
