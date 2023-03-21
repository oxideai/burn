use super::conv;
use crate::backend::Backend;

/// Gradient computed during the backward pass for each tensor used by [conv2d](ModuleOps::conv2d).
#[derive(new)]
pub struct Conv2dBackward<B: Backend> {
    pub x_grad: B::TensorPrimitive<4>,
    pub weights_grad: B::TensorPrimitive<4>,
    pub bias_grad: Option<B::TensorPrimitive<1>>,
}

/// Gradient computed during the backward pass for each tensor used by [max_pool2d](ModuleOps::max_pool2d).
#[derive(new)]
pub struct MaxPool2dBackward<B: Backend> {
    pub x_grad: B::TensorPrimitive<4>,
}

/// Results from [max_pool2d](ModuleOps::max_pool2d_with_indexes).
#[derive(new)]
pub struct MaxPool2dWithIndexes<B: Backend> {
    pub output: B::TensorPrimitive<4>,
    pub indexes: B::IntTensorPrimitive<4>,
}

/// Gradient computed during the backward pass for each tensor used by [conv1d](ModuleOps::conv1d).
#[derive(new)]
pub struct Conv1dBackward<B: Backend> {
    pub x_grad: B::TensorPrimitive<3>,
    pub weights_grad: B::TensorPrimitive<3>,
    pub bias_grad: Option<B::TensorPrimitive<1>>,
}

pub trait ModuleOps<B: Backend> {
    fn embedding(
        weights: B::TensorPrimitive<2>,
        indexes: B::IntTensorPrimitive<2>,
    ) -> B::TensorPrimitive<3>;
    fn embedding_backward(
        weights: B::TensorPrimitive<2>,
        output: B::TensorPrimitive<3>,
        indexes: B::IntTensorPrimitive<2>,
    ) -> B::TensorPrimitive<2>;
    /// Two dimensional convolution.
    ///
    /// # Shapes
    ///
    /// x:      [batch_size, channels_in, height, width],
    /// weight: [channels_out, channels_in, kernel_size_1, kernel_size_2],
    /// bias:   [channels_out],
    fn conv2d(
        x: B::TensorPrimitive<4>,
        weight: B::TensorPrimitive<4>,
        bias: Option<B::TensorPrimitive<1>>,
        stride: [usize; 2],
        padding: [usize; 2],
    ) -> B::TensorPrimitive<4>;
    /// Backward pass for the [conv2d](ModuleOps::conv2d) operation.
    fn conv2d_backward(
        x: B::TensorPrimitive<4>,
        weight: B::TensorPrimitive<4>,
        bias: Option<B::TensorPrimitive<1>>,
        stride: [usize; 2],
        output_grad: B::TensorPrimitive<4>,
    ) -> Conv2dBackward<B> {
        conv::conv2d_backward(x, weight, bias, stride, output_grad)
    }
    /// One dimensional convolution.
    ///
    /// # Shapes
    ///
    /// x:      [batch_size, channels_in, length],
    /// weight: [channels_out, channels_in, kernel_size],
    /// bias:   [channels_out],
    fn conv1d(
        x: B::TensorPrimitive<3>,
        weight: B::TensorPrimitive<3>,
        bias: Option<B::TensorPrimitive<1>>,
        stride: usize,
        padding: usize,
    ) -> B::TensorPrimitive<3> {
        conv::conv1d_from_conv2d::<B>(x, weight, bias, stride, padding)
    }
    /// Backward pass for the [conv1d](ModuleOps::conv1d) operation.
    fn conv1d_backward(
        x: B::TensorPrimitive<3>,
        weight: B::TensorPrimitive<3>,
        bias: Option<B::TensorPrimitive<1>>,
        stride: usize,
        output_grad: B::TensorPrimitive<3>,
    ) -> Conv1dBackward<B> {
        conv::conv1d_backward(x, weight, bias, stride, output_grad)
    }
    /// Two dimensional max pooling.
    ///
    /// # Shapes
    ///
    /// x: [batch_size, channels, height, width],
    fn max_pool2d(
        x: B::TensorPrimitive<4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
    ) -> B::TensorPrimitive<4>;
    /// Two dimensional max pooling with indexes.
    ///
    /// # Shapes
    ///
    /// x: [batch_size, channels, height, width],
    fn max_pool2d_with_indexes(
        x: B::TensorPrimitive<4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
    ) -> MaxPool2dWithIndexes<B>;
    /// Backward pass for the [max pooling 2d](ModuleOps::max_pool2d_with_indexes) operation.
    fn max_pool2d_with_indexes_backward(
        x: B::TensorPrimitive<4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        output_grad: B::TensorPrimitive<4>,
        indexes: B::IntTensorPrimitive<4>,
    ) -> MaxPool2dBackward<B>;
}