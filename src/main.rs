use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use sharedtypes::DbJobType;
use std::collections::BTreeMap;
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;

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
    LoadTable {
        tabletoload: sharedtypes::LoadDBTable,
    },
    /// Mnaages adding files into the fb. NOTE INCOMPLETE
    FileAdd {
        /// Source url to download from
        source_url: String,
        /// Optional - Skips if this tag exists and has a relationship to file
        #[clap(num_args = 1, short = 's')]
        skip_tag_name: Vec<String>,
        /// Optional - Skips if this tag namespace exists and has a relationship to file
        #[clap(num_args = 1, short = 'o')]
        skip_namespace_name: Vec<String>,
        /// Optional - Associates a tag to the file downloaded
        #[clap(num_args = 1, short = 't')]
        tag_name: Vec<String>,
        /// Optional - Associates a tag & namespace to the file downloaded
        #[clap(num_args = 1, short = 'n')]
        namespace_name: Vec<String>,
    },
    FileAddNoBlock {
        /// Source url to download from
        source_url: String,
        /// Optional - Skips if this tag exists and has a relationship to file
        #[clap(num_args = 1, short = 's')]
        skip_tag_name: Vec<String>,
        /// Optional - Skips if this tag namespace exists and has a relationship to file
        #[clap(num_args = 1, short = 'o')]
        skip_namespace_name: Vec<String>,
        /// Optional - Associates a tag to the file downloaded
        #[clap(num_args = 1, short = 't')]
        tag_name: Vec<String>,
        /// Optional - Associates a tag & namespace to the file downloaded
        #[clap(num_args = 1, short = 'n')]
        namespace_name: Vec<String>,
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
        JobType::FileAddNoBlock {
            source_url,
            skip_tag_name,
            skip_namespace_name,
            tag_name,
            namespace_name,
        } => {
            if skip_tag_name.len() != skip_namespace_name.len() {
                println!("Cannot add file add because skip tag names != skip namespace names");
                return;
            }
            if tag_name.len() != namespace_name.len() {
                println!("Cannot add file add because tag names != namespace names");
                return;
            }

            let mut skip = Vec::new();
            for i in 0..skip_tag_name.len() {
                skip.push(sharedtypes::SkipIf::FileTagRelationship(sharedtypes::Tag {
                    tag: skip_tag_name.get(i).unwrap().to_string(),
                    namespace: sharedtypes::GenericNamespaceObj {
                        name: skip_namespace_name.get(i).unwrap().to_string(),
                        description: None,
                    },
                }));
            }

            let mut tags = Vec::new();
            for i in 0..tag_name.len() {
                tags.push(sharedtypes::TagObject {
                    tag: tag_name.get(i).unwrap().to_string(),
                    namespace: sharedtypes::GenericNamespaceObj {
                        name: namespace_name.get(i).unwrap().to_string(),
                        description: None,
                    },
                    tag_type: sharedtypes::TagType::Normal,
                    relates_to: None,
                });
            }

            let file = sharedtypes::FileObject {
                hash: sharedtypes::HashesSupported::None,
                source_url: Some(source_url),
                tag_list: tags,
                skip_if: skip,
            };
            let ratelimit = (1, Duration::from_secs(1));
            client::load_table(sharedtypes::LoadDBTable::All);
            client::add_file(file, ratelimit);
        }

        JobType::FileAdd {
            source_url,
            skip_tag_name,
            skip_namespace_name,
            tag_name,
            namespace_name,
        } => {
            if skip_tag_name.len() != skip_namespace_name.len() {
                println!("Cannot add file add because skip tag names != skip namespace names");
                return;
            }
            if tag_name.len() != namespace_name.len() {
                println!("Cannot add file add because tag names != namespace names");
                return;
            }

            let mut skip = Vec::new();
            for i in 0..skip_tag_name.len() {
                skip.push(sharedtypes::SkipIf::FileTagRelationship(sharedtypes::Tag {
                    tag: skip_tag_name.get(i).unwrap().to_string(),
                    namespace: sharedtypes::GenericNamespaceObj {
                        name: skip_namespace_name.get(i).unwrap().to_string(),
                        description: None,
                    },
                }));
            }

            let mut tags = Vec::new();
            for i in 0..tag_name.len() {
                tags.push(sharedtypes::TagObject {
                    tag: tag_name.get(i).unwrap().to_string(),
                    namespace: sharedtypes::GenericNamespaceObj {
                        name: namespace_name.get(i).unwrap().to_string(),
                        description: None,
                    },
                    tag_type: sharedtypes::TagType::Normal,
                    relates_to: None,
                });
            }

            let file = sharedtypes::FileObject {
                hash: sharedtypes::HashesSupported::None,
                source_url: Some(source_url),
                tag_list: tags,
                skip_if: skip,
            };
            let ratelimit = (1, Duration::from_secs(1));
            client::load_table(sharedtypes::LoadDBTable::All);
            client::add_file(file, ratelimit);
        }
        JobType::LoadTable { tabletoload } => {
            client::load_table(tabletoload);
        }
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
            client::load_table(sharedtypes::LoadDBTable::Tags);
            client::load_table(sharedtypes::LoadDBTable::Namespace);
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
            client::load_table(sharedtypes::LoadDBTable::Tags);
            client::load_table(sharedtypes::LoadDBTable::Namespace);
            client::load_table(sharedtypes::LoadDBTable::Parents);
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
