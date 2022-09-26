// Copyright 2018 The Beam Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#pragma once

struct WalkerContracts
{
#pragma pack (push, 1)
	struct SidCid {
		ShaderID m_Sid;
		ContractID m_Cid;
	};
#pragma pack (pop)

	typedef Env::Key_T<SidCid> KeySidCid;

	KeySidCid m_Key;
	Height m_Height;

	Env::VarReaderEx<true> m_Reader;

	void Enum(const ShaderID* pSid)
	{
		KeySidCid k0, k1;
		_POD_(k0).SetZero();
		k0.m_Prefix.m_Tag = KeyTag::SidCid;

		if (pSid)
			_POD_(k0.m_KeyInContract.m_Sid) = *pSid;

		_POD_(k1) = k0;
		_POD_(k1.m_KeyInContract.m_Cid).SetObject(0xff);
		if (!pSid)
			_POD_(k1.m_KeyInContract.m_Sid).SetObject(0xff);

		m_Reader.Enum_T(k0, k1);
	}

	void Enum()
	{
		Enum(nullptr);
	}

	void Enum(const ShaderID& sid)
	{
		Enum(&sid);
	}

	bool MoveNext()
	{
		if (!m_Reader.MoveNext_T(m_Key, m_Height))
			return false;

		m_Height = Utils::FromBE(m_Height);
		return true;
	}
};

inline void EnumAndDumpContracts(const ShaderID& sid)
{
	Env::DocArray gr("contracts");

	WalkerContracts wlk;
	for (wlk.Enum(sid); wlk.MoveNext(); )
	{
		Env::DocGroup root("");

		Env::DocAddBlob_T("cid", wlk.m_Key.m_KeyInContract.m_Cid);
		Env::DocAddNum("Height", wlk.m_Height);
	}
}

struct WalkerFunds
{
#pragma pack (push, 1)

	typedef Env::Key_T<AssetID> KeyFunds; // AssetID is in big-endian format

	struct ValueFunds
	{
		Amount m_Hi;
		Amount m_Lo;
	};

#pragma pack (pop)

	AssetID m_Aid;
	ValueFunds m_Val;
	Env::VarReaderEx<true> m_Reader;

	void Enum(const ContractID& cid, const AssetID* pAid = nullptr)
	{
		KeyFunds k0;
		k0.m_Prefix.m_Cid = cid;
		k0.m_Prefix.m_Tag = KeyTag::LockedAmount;

		if (pAid)
		{
			k0.m_KeyInContract = Utils::FromBE(*pAid);
			m_Reader.Enum_T(k0, k0);
		}
		else
		{
			KeyFunds k1;
			_POD_(k1.m_Prefix) = k0.m_Prefix;

			k0.m_KeyInContract = 0;
			k1.m_KeyInContract = static_cast<AssetID>(-1);

			m_Reader.Enum_T(k0, k1);
		}
	}

	bool MoveNext()
	{
		KeyFunds key;
		ValueFunds val;

		if (!m_Reader.MoveNext_T(key, val))
			return false;

		m_Aid = Utils::FromBE(key.m_KeyInContract);
		m_Val.m_Lo = Utils::FromBE(val.m_Lo);
		m_Val.m_Hi = Utils::FromBE(val.m_Hi);

		return true;
	}
};

namespace Utils
{
	inline void get_ShaderID(ShaderID& sid, const void* p, uint32_t n)
	{
		HashProcessor::Sha256 hp;
		hp
			<< "bvm.shader.id"
			<< n;

		hp.Write(p, n);
		hp >> sid;
	}

	inline void get_Cid(ContractID& cid, const ShaderID& sid, const void* pArg, uint32_t nArg)
	{
		HashProcessor::Sha256 hp;
		hp
			<< "bvm.cid"
			<< sid
			<< nArg;

		hp.Write(pArg, nArg);
		hp >> cid;
	}

	inline bool get_ShaderID_FromArg(ShaderID& sid)
	{
		static const char szName[] = "contract.shader";
		uint32_t nSize = Env::DocGetBlob(szName, nullptr, 0);
		if (!nSize)
			return 0;

		void* p = Env::Heap_Alloc(nSize);
		Env::DocGetBlob(szName, p, nSize);

		get_ShaderID(sid, p, nSize);

		Env::Heap_Free(p);

		return true;
	}

	inline bool get_ShaderID_FromContract(ShaderID& sid, const ContractID& cid)
	{
		Env::VarReader r(cid, cid);

		uint32_t nKey = 0, nVal = 0;
		if (!r.MoveNext(nullptr, nKey, nullptr, nVal, 0))
			return false;

		void* pVal = Env::Heap_Alloc(nVal);

		nKey = 0;
		r.MoveNext(nullptr, nKey, pVal, nVal, 1);

		get_ShaderID(sid, pVal, nVal);

		Env::Heap_Free(pVal);
		return true;
	}

};
