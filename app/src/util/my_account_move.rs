pub fn my_account_move(isDeposit: bool, contract_id: ContractID, amount: Amount, asset_id: AssetID) {

    /*
    
    void On_my_account_move(const uint8_t &isDeposit, const ContractID &cid, const Amount &amount, const AssetID &aid, int unused = 0)
{
    if (!amount)
        return OnError("amount should be nnz");

    FundsChange fc;
    fc.m_Amount = amount;
    fc.m_Aid = aid;
    fc.m_Consume = isDeposit;

    if (isDeposit)
    {
        Faucet::Deposit arg;
        arg.m_Aid = fc.m_Aid;
        arg.m_Amount = fc.m_Amount;

        Env::GenerateKernel(&cid, Faucet::Deposit::s_iMethod, &arg, sizeof(arg), &fc, 1, nullptr, 0, "deposit to Faucet", 0);
    }
    else
    {
        Faucet::Withdraw arg;
        arg.m_Amount = fc.m_Amount;
        arg.m_Key.m_Aid = fc.m_Aid;
        DeriveMyPk(arg.m_Key.m_Account, cid);

        SigRequest sig;
        sig.m_pID = &cid;
        sig.m_nID = sizeof(cid);

        Env::GenerateKernel(&cid, Faucet::Withdraw::s_iMethod, &arg, sizeof(arg), &fc, 1, &sig, 1, "withdraw from Faucet", 0);
    }
}

    
    */

}