import { assert } from "chai"
import { readFileSync }  from 'fs';
import { Program } from '../wasm/node/ezbpf_wasm';

describe('Program', () => {
  it('Disasseumbles a program', async () => {
    const programBytes = Buffer.from("7F454C460201010000000000000000000300F700010000007800000000000000400000000000000090000000000000000000000040003800010040000300020001000000050000007800000000000000780000000000000078000000000000000800000000000000080000000000000000100000000000009500000000000000002E74657874002E7300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000001000000060000000000000078000000000000007800000000000000080000000000000000000000000000000400000000000000000000000000000007000000030000000000000000000000000000000000000080000000000000000A00000000000000000000000000000001000000000000000000000000000000", "hex");
    let p = Program.from_bytes(programBytes);
    console.log(JSON.stringify(p.to_json()));
  })
});