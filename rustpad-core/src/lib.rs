//! Core logic for Rustpad, shared with the client through WebAssembly

#![warn(missing_docs)]

use operational_transform::OperationSeq;
use wasm_bindgen::prelude::*;

pub mod utils;

/// This is an wrapper around `operational_transform::OperationSeq`, which is
/// necessary for Wasm compatibility through `wasm-bindgen`.
#[wasm_bindgen]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpSeq(OperationSeq);

/// This is a pair of `OpSeq` structs, which is needed to handle some return
/// values from `wasm-bindgen`.
#[wasm_bindgen]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpSeqPair(OpSeq, OpSeq);

#[wasm_bindgen]
impl OpSeq {
    /// Creates a store for operatations which does not need to allocate  until
    /// `capacity` operations have been stored inside.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(OperationSeq::with_capacity(capacity))
    }

    /// Merges the operation with `other` into one operation while preserving
    /// the changes of both. Or, in other words, for each input string S and a
    /// pair of consecutive operations A and B.
    ///     `apply(apply(S, A), B) = apply(S, compose(A, B))`
    /// must hold.
    ///
    /// # Error
    ///
    /// Returns `None` if the operations are not composable due to length
    /// conflicts.
    pub fn compose(&self, other: &OpSeq) -> Option<OpSeq> {
        self.0.compose(&other.0).ok().map(Self)
    }

    /// Deletes `n` characters at the current cursor position.
    pub fn delete(&mut self, n: u64) {
        self.0.delete(n)
    }

    /// Inserts a `s` at the current cursor position.
    pub fn insert(&mut self, s: &str) {
        self.0.insert(s)
    }

    /// Moves the cursor `n` characters forwards.
    pub fn retain(&mut self, n: u64) {
        self.0.retain(n)
    }

    /// Transforms two operations A and B that happened concurrently and produces
    /// two operations A' and B' (in an array) such that
    ///     `apply(apply(S, A), B') = apply(apply(S, B), A')`.
    /// This function is the heart of OT.
    ///
    /// # Error
    ///
    /// Returns `None` if the operations cannot be transformed due to
    /// length conflicts.
    pub fn transform(&self, other: &OpSeq) -> Option<OpSeqPair> {
        let (a, b) = self.0.transform(&other.0).ok()?;
        Some(OpSeqPair(Self(a), Self(b)))
    }

    /// Applies an operation to a string, returning a new string.
    ///
    /// # Error
    ///
    /// Returns an error if the operation cannot be applied due to length
    /// conflicts.
    pub fn apply(&self, s: &str) -> Option<String> {
        self.0.apply(s).ok()
    }

    /// Computes the inverse of an operation. The inverse of an operation is the
    /// operation that reverts the effects of the operation, e.g. when you have
    /// an operation 'insert("hello "); skip(6);' then the inverse is
    /// 'delete("hello "); skip(6);'. The inverse should be used for
    /// implementing undo.
    pub fn invert(&self, s: &str) -> Self {
        Self(self.0.invert(s))
    }

    /// Checks if this operation has no effect.
    #[inline]
    pub fn is_noop(&self) -> bool {
        self.0.is_noop()
    }

    /// Returns the length of a string these operations can be applied to
    #[inline]
    pub fn base_len(&self) -> usize {
        self.0.base_len()
    }

    /// Returns the length of the resulting string after the operations have
    /// been applied.
    #[inline]
    pub fn target_len(&self) -> usize {
        self.0.target_len()
    }
}

#[wasm_bindgen]
impl OpSeqPair {
    /// Returns the first element of the pair.
    pub fn first(&self) -> OpSeq {
        self.0.clone()
    }

    /// Returns the second element of the pair.
    pub fn second(&self) -> OpSeq {
        self.1.clone()
    }
}
