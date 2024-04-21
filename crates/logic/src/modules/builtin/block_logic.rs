use crate::*;

pub fn and(input: BlockInput, mut output: BlockOutputMut) {
    output[0] = input[0] & input[1];
}
