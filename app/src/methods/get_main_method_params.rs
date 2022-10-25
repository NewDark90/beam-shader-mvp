use beam_bvm_util::common::extensions::*;
use crate::methods::main_method_events::*;
use crate::util::doc_writer::*;

#[export_name = "Method_0"]
pub fn get_main_method_params() {

/* 
    doc_object!({ 
        doc_object!("roles\0", {
            doc_add_num32(to_c_string("alskj\0"), 1);
        });
    });*/
    DOC_WRITER.object(|root|{
        root.object_prop(&"roles".to_c_string(), |roles| {
            roles
                .object_prop(&"manager".to_c_string(), |manager|{
                    manager
                        .object_prop(&"create".to_c_string(), |create| {
                            manager_create::write_props(create);
                        })
                        .object_prop(&"destroy".to_c_string(), |destroy| {
                            manager_destroy::write_props(destroy);
                        })
                        .object_prop(&"view".to_c_string(), |view| {
                            manager_view::write_props(view);
                        })
                        .object_prop(&"view_params".to_c_string(), |view_params| {
                            manager_view_params::write_props(view_params);
                        })
                        .object_prop(&"view_funds".to_c_string(), |view_funds| {
                            manager_view_funds::write_props(view_funds);
                        })
                        .object_prop(&"view_accounts".to_c_string(), |view_accounts| {
                            manager_view_accounts::write_props(view_accounts);
                        })
                        .object_prop(&"view_account".to_c_string(), |view_account| {
                            manager_view_account::write_props(view_account);
                        });
                })
                .object_prop(&"my_account".to_c_string(), |my_account|{
                    my_account
                        .object_prop(&"view".to_c_string(), |view|{
                            my_account_view::write_props(view);
                        })
                        .object_prop(&"deposit".to_c_string(), |deposit|{
                            my_account_deposit::write_props(deposit);
                        })
                        .object_prop(&"withdraw".to_c_string(), |withdraw|{
                            my_account_withdraw::write_props(withdraw);
                        });
                });
        });
    });
}
