use clap::Parser;
use clap::Subcommand;
use sharedtypes::DbJobType;
use std::collections::BTreeMap;

#[path = "../Rust-Hydrus/src/scr/sharedtypes.rs"]
mod sharedtypes;

#[path = "../Rust-Hydrus/src/scr/intcoms/client.rs"]
mod client;

#[derive(Subcommand, Debug, Clone)]
pub enum JobType {
    JobsAdd {
        site: String,
        param: String,
        dbjobstype: DbJobType,
        #[clap(num_args = 1, short = 'k')]
        user_data_key: Vec<String>,
        #[clap(num_args = 1, short = 'v')]
        user_data_val: Vec<String>,
    },
    TagAdd {
        tag: String,
        namespace_name: String,
        namespace_description: Option<String>,
    },
    TransactionFlush,
    RelateTagToTag {
        tag: String,
        namespace_name: String,
        namespace_description: Option<String>,
        relate_tag: String,
        relate_namespace_name: String,
        relate_namespace_description: Option<String>,
        limit_tag: Option<String>,
        limit_namespace_name: Option<String>,
        limit_namespace_description: Option<String>,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Jobs to add into db
    #[command(subcommand)]
    jobtype: JobType,
}
fn main() {
    let jobtype = Args::parse().jobtype;
    match jobtype {
        JobType::JobsAdd {
            site,
            param,
            dbjobstype,
            user_data_key,
            user_data_val,
        } => {
            if user_data_key.len() != user_data_val.len() {
                println!("Cannot run jobs add because user_key_dat != user_data_val");
                return;
            }

            let mut user_data = BTreeMap::new();
            for i in 0..user_data_key.len() {
                user_data.insert(
                    user_data_key.get(i).unwrap().clone(),
                    user_data_val.get(i).unwrap().clone(),
                );
            }
            client::load_table(sharedtypes::LoadDBTable::Jobs);
            client::job_add(
                None,
                0,
                0,
                site,
                param,
                true,
                sharedtypes::CommitType::StopOnNothing,
                dbjobstype,
                BTreeMap::new(),
                user_data,
                sharedtypes::DbJobsManager {
                    jobtype: sharedtypes::DbJobType::Scraper,
                    recreation: None,
                },
            );
        }
        JobType::TagAdd {
            tag,
            namespace_name,
            namespace_description,
        } => {
            let nid = client::namespace_put(namespace_name, namespace_description, true);
            client::tag_add(tag, nid, true, None);
        }
        JobType::TransactionFlush => {
            client::transaction_flush();
        }
        JobType::RelateTagToTag {
            tag,
            namespace_name,
            namespace_description,
            relate_tag,
            relate_namespace_name,
            relate_namespace_description,
            limit_tag,
            limit_namespace_name,
            limit_namespace_description,
        } => {
            let nid = client::namespace_put(namespace_name, namespace_description, true);

            let rel_nid =
                client::namespace_put(relate_namespace_name, relate_namespace_description, true);

            let mut limit_tid = None;

            if limit_tag.is_some() && limit_namespace_name.is_some() {
                let limit_nid = client::namespace_put(
                    limit_namespace_name.unwrap(),
                    limit_namespace_description,
                    true,
                );
                limit_tid = Some(client::tag_add(limit_tag.unwrap(), limit_nid, true, None));
            }

            let tid = client::tag_add(tag, nid, true, None);
            let rel_tid = client::tag_add(relate_tag, rel_nid, true, None);
            let _ = client::parents_put(sharedtypes::DbParentsObj {
                tag_id: tid,
                relate_tag_id: rel_tid,
                limit_to: limit_tid,
            });
        }
    }
}
