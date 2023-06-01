use acvm::acir::native_types::Witness;

use acvm::pwg::range;
use acvm::pwg::signature;
use acvm::OpcodeResolutionError;
use acvm::PartialWitnessGenerator;

mod gadget_call;

use crate::pse_halo2::PseHalo2;

impl PartialWitnessGenerator for PseHalo2 {
    fn aes(
        &self,
        _initial_witness: &mut acvm::acir::native_types::WitnessMap,
        _inputs: &[acvm::acir::circuit::opcodes::FunctionInput],
        _outputs: &[Witness],
    ) -> Result<acvm::pwg::OpcodeResolution, OpcodeResolutionError> {
        todo!()
    }

    fn and(
        &self,
        _initial_witness: &mut acvm::acir::native_types::WitnessMap,
        _lhs: &acvm::acir::circuit::opcodes::FunctionInput,
        _rhs: &acvm::acir::circuit::opcodes::FunctionInput,
        _output: &Witness,
    ) -> Result<acvm::pwg::OpcodeResolution, OpcodeResolutionError> {
        todo!()
    }

    fn xor(
        &self,
        _initial_witness: &mut acvm::acir::native_types::WitnessMap,
        _lhs: &acvm::acir::circuit::opcodes::FunctionInput,
        _rhs: &acvm::acir::circuit::opcodes::FunctionInput,
        _output: &Witness,
    ) -> Result<acvm::pwg::OpcodeResolution, OpcodeResolutionError> {
        todo!()
    }

    fn range(
        &self,
        initial_witness: &mut acvm::acir::native_types::WitnessMap,
        input: &acvm::acir::circuit::opcodes::FunctionInput,
    ) -> Result<acvm::pwg::OpcodeResolution, OpcodeResolutionError> {
        range::solve_range_opcode(initial_witness, input)
    }

    fn sha256(
        &self,
        _initial_witness: &mut acvm::acir::native_types::WitnessMap,
        _inputs: &[acvm::acir::circuit::opcodes::FunctionInput],
        _outputs: &[Witness],
    ) -> Result<acvm::pwg::OpcodeResolution, OpcodeResolutionError> {
        todo!()
    }

    fn blake2s(
        &self,
        _initial_witness: &mut acvm::acir::native_types::WitnessMap,
        _inputs: &[acvm::acir::circuit::opcodes::FunctionInput],
        _outputs: &[Witness],
    ) -> Result<acvm::pwg::OpcodeResolution, OpcodeResolutionError> {
        todo!()
    }

    fn compute_merkle_root(
        &self,
        _initial_witness: &mut acvm::acir::native_types::WitnessMap,
        _leaf: &acvm::acir::circuit::opcodes::FunctionInput,
        _index: &acvm::acir::circuit::opcodes::FunctionInput,
        _hash_path: &[acvm::acir::circuit::opcodes::FunctionInput],
        _output: &Witness,
    ) -> Result<acvm::pwg::OpcodeResolution, OpcodeResolutionError> {
        todo!()
    }

    fn schnorr_verify(
        &self,
        _initial_witness: &mut acvm::acir::native_types::WitnessMap,
        _public_key_x: &acvm::acir::circuit::opcodes::FunctionInput,
        _public_key_y: &acvm::acir::circuit::opcodes::FunctionInput,
        _signature: &[acvm::acir::circuit::opcodes::FunctionInput],
        _message: &[acvm::acir::circuit::opcodes::FunctionInput],
        _output: &Witness,
    ) -> Result<acvm::pwg::OpcodeResolution, OpcodeResolutionError> {
        todo!()
    }

    fn pedersen(
        &self,
        _initial_witness: &mut acvm::acir::native_types::WitnessMap,
        _inputs: &[acvm::acir::circuit::opcodes::FunctionInput],
        _outputs: &[Witness],
    ) -> Result<acvm::pwg::OpcodeResolution, OpcodeResolutionError> {
        todo!()
    }

    fn hash_to_field_128_security(
        &self,
        _initial_witness: &mut acvm::acir::native_types::WitnessMap,
        _inputs: &[acvm::acir::circuit::opcodes::FunctionInput],
        _outputs: &Witness,
    ) -> Result<acvm::pwg::OpcodeResolution, OpcodeResolutionError> {
        todo!()
    }

    fn ecdsa_secp256k1(
        &self,
        initial_witness: &mut acvm::acir::native_types::WitnessMap,
        public_key_x: &[acvm::acir::circuit::opcodes::FunctionInput],
        public_key_y: &[acvm::acir::circuit::opcodes::FunctionInput],
        signature: &[acvm::acir::circuit::opcodes::FunctionInput],
        message: &[acvm::acir::circuit::opcodes::FunctionInput],
        outputs: &Witness,
    ) -> Result<acvm::pwg::OpcodeResolution, OpcodeResolutionError> {
        signature::ecdsa::secp256k1_prehashed(
            initial_witness,
            public_key_x,
            public_key_y,
            signature,
            message,
            *outputs,
        )
    }

    fn fixed_base_scalar_mul(
        &self,
        _initial_witness: &mut acvm::acir::native_types::WitnessMap,
        _input: &acvm::acir::circuit::opcodes::FunctionInput,
        _outputs: &[Witness],
    ) -> Result<acvm::pwg::OpcodeResolution, OpcodeResolutionError> {
        todo!()
    }

    fn keccak256(
        &self,
        _initial_witness: &mut acvm::acir::native_types::WitnessMap,
        _inputs: &[acvm::acir::circuit::opcodes::FunctionInput],
        _outputs: &[Witness],
    ) -> Result<acvm::pwg::OpcodeResolution, OpcodeResolutionError> {
        todo!()
    }
}