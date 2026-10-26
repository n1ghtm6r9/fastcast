#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MulAddKernel {
    Scalar,
    Ssse3,
    Avx2,
}
