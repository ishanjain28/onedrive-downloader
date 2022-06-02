use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveItem {
    pub created_by: CreatedBy,
    pub created_date_time: String,
    pub c_tag: String,
    pub e_tag: String,
    pub id: String,
    pub last_modified_by: LastModifiedBy,
    pub last_modified_date_time: String,
    pub name: String,
    pub parent_reference: ParentReference,
    pub size: u64,
    pub web_url: String,
    pub file_system_info: FileSystemInfo,
    pub folder: Folder,
    pub reactions: Reactions,
    pub shared: Shared,
    #[serde(rename = "children@odata.count")]
    pub children_odata_count: i64,
    pub children: Vec<Children>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedBy {
    pub application: Option<Application>,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub display_name: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub display_name: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastModifiedBy {
    pub application: Option<Application>,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentReference {
    pub drive_id: String,
    pub drive_type: String,
    pub id: Option<String>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub share_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileSystemInfo {
    pub created_date_time: String,
    pub last_modified_date_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub child_count: i64,
    pub folder_view: FolderView,
    pub folder_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderView {
    pub view_type: String,
    pub sort_by: String,
    pub sort_order: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reactions {
    pub comment_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shared {
    pub effective_roles: Vec<String>,
    pub owner: Owner,
    pub scope: String,
    pub shared_date_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    #[serde(rename = "@content.downloadUrl")]
    pub content_download_url: Option<String>,
    pub created_by: CreatedBy,
    pub created_date_time: String,
    pub c_tag: String,
    pub e_tag: String,
    pub id: String,
    pub last_modified_by: LastModifiedBy,
    pub last_modified_date_time: String,
    pub name: String,
    pub parent_reference: ParentReference,
    pub size: u64,
    pub web_url: String,
    pub file_system_info: FileSystemInfo,
    pub folder: Option<Folder>,
    pub file: Option<File>,
    pub reactions: Reactions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub hashes: Hashes,
    pub mime_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hashes {
    pub quick_xor_hash: String,
    #[serde(rename = "sha1Hash")]
    pub sha1hash: String,
    #[serde(rename = "sha256Hash")]
    pub sha256hash: String,
}
