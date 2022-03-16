use wasm_bindgen::prelude::*;

use crate::types::*;
use crate::state::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct CtorParamsWrapper
{
    value: CtorParams
}
    
#[wasm_bindgen]
pub fn Ctor(ctor: CtorParamsWrapper)
{
    unsafe 
    {
        let state: State;
        
        Utils::BlobOf<CtorParams>(state.params) = ctor.value;
        Utils::BlobOf<Epoch>(state.epoch).SetZero();
        state.enabled = true;
        state.Save();
    }
}




/*


inline bool LoadVar_T(const TKey& key, TVal& val, uint8_t nType = KeyTag::Internal)
    {
        return LoadVar(&key, sizeof(key), &val, sizeof(val), nType) == sizeof(val);
    }

BEAM_EXPORT void Ctor(const Method::Create& r)
{
    MyState s;
    _POD_(s.m_Params) = r.m_Params;
    _POD_(s.m_Epoch).SetZero();
    s.m_Enabled = true;
    s.Save();
}
*/