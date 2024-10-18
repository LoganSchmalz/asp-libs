// TEMPLATE.txt
// General structure for ASP's written in rust

// Common Packages
use rust_am_lib::copland::*;
use anyhow::{Context, Result};
use std::env;
use hex;

use std::collections::HashMap;

// Packages required to perform specific ASP action.
// e.g.
use sha2::{Sha256, Digest};



// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body() -> Result<String> {
    

    let golden_filename_key = "cds_config_1_targ";
    let golden_filename_key_2 = "cds_config_2_targ";
    let golden_filename_key_3 = "cds_config_3_targ";


    let golden_filename = 
        "/Users/adampetz/Documents/Summer_2024/asp-libs/attestation_asps/rust_asps/golden_files/targFileGolden.txt";
    
    let golden_filename_2 = 
        "/Users/adampetz/Documents/Summer_2024/asp-libs/attestation_asps/rust_asps/golden_files/targFileGolden_2.txt";

    let golden_filename_3 = 
        "/Users/adampetz/Documents/Summer_2024/asp-libs/attestation_asps/rust_asps/golden_files/targFileGolden_3.txt";
    let mut goldenPathMap = HashMap::new();
    goldenPathMap.insert(golden_filename_key.to_string(), golden_filename.to_string());
    goldenPathMap.insert(golden_filename_key_2.to_string(), golden_filename_2.to_string());
    goldenPathMap.insert(golden_filename_key_3.to_string(), golden_filename_3.to_string());



    // For every ASP, an ASPRunRequest appears as the single command-line argument
    let args: Vec <String> = env::args().collect();

    if args.len() < 2 {
        return Err(anyhow::anyhow!("ASPRunRequest not supplied as command line argument"));
    }

    let json_request = &args[1];
    // May fail.  If so, return an Err Result
    let req: ASPRunRequest = serde_json::from_str(json_request)?;

    // Code for specific for this ASP.
    // This example computes the HASH of the file named in an argument for the ASP.
    // May return an Err Result, which will be captured in main.


    let golden_filename_key_targid = req.ASP_TARG_ID;
    let golden_filename = goldenPathMap[&golden_filename_key_targid].clone();


    let bytes = std::fs::read(golden_filename)?; // Vec<u8>

    //Code sample for accessing input evidence within the ASP.

    // Suppose the file contents are to be extracted from evidence...

    let evidence_in = match req.RAWEV {RawEv::RawEv(x) => x,};

    let latest_evidence = &evidence_in[0];

    // Evidence is always hex encoded, so decode this
    let file_bytes = hex::decode(latest_evidence)?;
    let bytes_equal : bool = bytes.eq(&file_bytes);


    // End of code specific for this ASP.

    // Common code to bundle computed value.
    // Step 1:
    // The return value for an ASP, must be
    // encoded in BASE64, and converted to ascii for JSON transmission

    let out_contents: String =
        match bytes_equal {
            true => {"PASSED".to_string()}
            false => {"FAILED".to_string()}
        };

    let out_contents_hex: String = hex::encode_upper(out_contents);

    // Step 2:
    // wrap the value as Evidence
    let evidence = RawEv::RawEv(vec![out_contents_hex]);

    // Step 3:
    // Construct the ASPRunResponse with this evidence.

    let response = successfulASPRunResponse (evidence);
    let response_json = serde_json::to_string(&response)?;
    Ok (response_json)
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()

fn main() {

    let response_json = match body() {
        Ok(resp) => resp,
        Err(_error) => {
            let  response = failureASPRunResponse (_error.to_string());
            // If an error occurs converting the failure response to JSON
            // there is nothing else to do but panic.
            // This should never happen.
            serde_json::to_string(&response).unwrap_or_else(|error| {panic!("Failed to json.encode failure response: {error:?}");})
        }
    };
    // The ASP output (ASPRunRequest) is written to stdout.
    // The caller will capture stdout to receive the response from this ASP.
    println!("{response_json}");
}
