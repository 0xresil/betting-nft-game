import { Connection, clusterApiUrl } from "@solana/web3.js";
export const GLOBAL_STATE_SEED = "GLOBAL_STATE_SEED";
export const USER_STATE_SEED = "USER_STATE_SEED";
export const ARENA_STATE_SEED = "ARENA_STATE_SEED";
export const USER_BET_SEED = "USER_BET_SEED";

export const USDC_DECIMALS = 6;

export const DEVNET_MODE = true;
export const TREASURY = "JAwNgkoSRMJzMndLtxBVSVp3ZPUfw1MEJ5GaAQ2gWcDT";
export const PROGRAM_ID = "7RQaqufWuf9px2fz6Rc3z3XghhKn77Vwf7KTXGSNsqzH";
export const USDC_MINT = DEVNET_MODE ? "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr":
                                      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
export const SOL_PYTH_ACCOUNT = DEVNET_MODE ? "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix" :
        "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG";