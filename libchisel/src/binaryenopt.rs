/*
 * Binaryen optimiser.
 */

use std::collections::HashMap;

use super::{ModuleError, ModulePreset, ModuleTranslator};
use parity_wasm::elements::*;

// FIXME: change level names
pub enum BinaryenOptimiser {
    O0, // Baseline aka no changes
    O1,
    O2,
    O3,
    O4,
    Os,
    Oz,
}

impl ModulePreset for BinaryenOptimiser {
    fn with_preset(preset: &str) -> Result<Self, ()> {
        match preset {
            "O0" => Ok(BinaryenOptimiser::O0),
            "O1" => Ok(BinaryenOptimiser::O1),
            "O2" => Ok(BinaryenOptimiser::O2),
            "O3" => Ok(BinaryenOptimiser::O3),
            "O4" => Ok(BinaryenOptimiser::O4),
            "Os" => Ok(BinaryenOptimiser::Os),
            "Oz" => Ok(BinaryenOptimiser::Oz),
            _ => Err(()),
        }
    }
}

impl ModuleTranslator for BinaryenOptimiser {
    fn translate_inplace(&self, module: &mut Module) -> Result<bool, ModuleError> {
        Err(ModuleError::NotSupported)
    }

    fn translate(&self, module: &Module) -> Result<Module, ModuleError> {
        // FIXME: could just move this into `BinaryenOptimiser`
        let config = match self {
            O0 => binaryen::CodegenConfig {
                optimization_level: 0,
                shrink_level: 0,
            },
            O1 => binaryen::CodegenConfig {
                optimization_level: 1,
                shrink_level: 0,
            },
            O2 => binaryen::CodegenConfig {
                optimization_level: 2,
                shrink_level: 0,
            },
            O3 => binaryen::CodegenConfig {
                optimization_level: 3,
                shrink_level: 0,
            },
            O4 => binaryen::CodegenConfig {
                optimization_level: 4,
                shrink_level: 0,
            },
            Os => binaryen::CodegenConfig {
                optimization_level: 2,
                shrink_level: 1,
            },
            Oz => binaryen::CodegenConfig {
                optimization_level: 2,
                shrink_level: 2,
            },
        };

        // FIXME: there must be a better way to accomplish this.
        let serialised = parity_wasm::elements::serialize::<Module>(module.clone())
            .expect("invalid input module");
        let output = binaryen_optimiser(&serialised, &config);
        Ok(
            parity_wasm::elements::deserialize_buffer::<Module>(&output[..])
                .expect("invalid output module"),
        )
    }
}

fn binaryen_optimiser(input: &[u8], config: &binaryen::CodegenConfig) -> Vec<u8> {
    // NOTE: this can abort (panic) if the input is invalid
    // NOTE: need to update to last release of binaryen-rs to avoid the panicing version)
    let module = binaryen::Module::read(&input);
    // NOTE: this is a global setting...
    binaryen::set_global_codegen_config(&config);
    module.optimize();
    module.write()
}

#[cfg(test)]
mod tests {
    use super::*;
    use parity_wasm::elements::deserialize_buffer;

    #[test]
    fn start_required_good() {
        let wasm: Vec<u8> = vec![
            0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x04, 0x01, 0x60, 0x00, 0x00,
            0x03, 0x02, 0x01, 0x00, 0x07, 0x08, 0x01, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x00, 0x00,
            0x08, 0x01, 0x00, 0x0a, 0x04, 0x01, 0x02, 0x00, 0x0b,
        ];

        let module = deserialize_buffer::<Module>(&wasm).unwrap();
        let translator = BinaryenOptimiser::with_preset("O0").unwrap();
        let result = translator.translate(&module).unwrap();
        assert_eq!(module, result);
    }
}
