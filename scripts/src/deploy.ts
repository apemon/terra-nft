import {create_wallet, init, query, upload} from './util'

require('dotenv').config()
const { MNEMONIC } = process.env

const wallet = create_wallet(MNEMONIC);

(async() => {
    // upload code
    const token_code_path = '../artifacts/cw721_base.wasm'
    const token_code_id = await upload(wallet, token_code_path)
    const options_code_path = '../artifacts/options.wasm'
    const options_code_id = await upload(wallet, options_code_path)
    // initialize contract
    const options_response = await init(wallet, options_code_id, {
        token_code_id: token_code_id
    })
    console.log(options_response)
    //const config_response = await query()
})()