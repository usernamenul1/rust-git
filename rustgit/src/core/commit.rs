use crate::core::object::Object;

pub struct CommitBuilder;

impl CommitBuilder {
    pub fn create_commit(
        repo_path: &str,
        tree_hash: String,
        parent_hash: Option<String>,
        author_info: String,
        commit_messsage: String,
    ) -> String {
        //get current timestamp RFC 2822
        let timestamp = chrono::Utc::now().to_rfc2822();

        panic!("CommitBuilder::create_commit is not implemented yet");
        // let commit_content =
        // format!(
        //     "tree {}\n{}",
        //     tree_hash,
        //     match parent_hash {
        //         Some(hash) => format!("parent {}\n", hash),
        //         None => String::new(),
        //     }
        // );

        // let commit_obj = Object::Commit(commit_content);
        // commit_obj.save(repo_path);
    }
}
