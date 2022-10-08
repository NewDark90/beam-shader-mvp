use beam_bvm_util::{util::app::document_writer::*};

#[export_name = "Method_0"]
pub fn method_0() {
    let doc_writer = DocumentWriter {};

    doc_writer.object(|root|{
        root.object_prop("roles\0", |roles| {
            roles
                .object_prop("manager\0", |manager|{
                    manager
                        .object_prop("create\0", |create| {
                            create
                                .string_prop("backlogPeriod\0", "Height\0")
                                .string_prop("withdrawLimit\0", "Amount\0");
                        })
                        .object_prop("destroy\0", |destroy| {
                            destroy.string_prop("cid\0", "ContractID\0");
                        })
                        .object_prop("view\0", |_view| {

                        })
                        .object_prop("view_params\0", |view_params| {
                            view_params.string_prop("cid\0", "ContractID\0");
                        })
                        .object_prop("view_funds\0", |view_funds| {
                            view_funds.string_prop("cid\0", "ContractID\0");
                        })
                        .object_prop("view_accounts\0", |view_accounts| {
                            view_accounts.string_prop("cid\0", "ContractID\0");
                        })
                        .object_prop("view_account\0", |view_account| {
                            view_account
                                .string_prop("cid\0", "ContractID\0")
                                .string_prop("pubKey\0", "PubKey\0");
                        });
                })
                .object_prop("my_account\0", |my_account|{
                    my_account
                        .object_prop("view\0", |view|{
                            view.string_prop("cid\0", "ContractID\0");
                        })
                        .object_prop("deposit\0", |deposit|{
                            deposit
                                .string_prop("cid\0", "ContractID\0")
                                .string_prop("amount\0", "Amount\0")
                                .string_prop("aid\0", "AssetID\0");
                            
                        })
                        .object_prop("withdraw\0", |withdraw|{
                            withdraw
                                .string_prop("cid\0", "ContractID\0")
                                .string_prop("amount\0", "Amount\0")
                                .string_prop("aid\0", "AssetID\0");
                        });
                });
        });
    });
}
