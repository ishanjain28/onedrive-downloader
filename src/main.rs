use crate::types::{Children, DriveItem};
use async_recursion::async_recursion;
use futures::StreamExt;
use std::{collections::HashMap, path::PathBuf, process::Command};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

mod types;

#[derive(Debug, Clone)]
enum Node {
    Folder(FolderNode),
    File(FileNode),
}

#[derive(Debug, Default, Clone)]
struct FolderNode {
    name: String,
    childrens: HashMap<String, Node>,
}

#[derive(Debug, Default, Clone)]
struct FileNode {
    name: String,
    size: u64,
    content_download_url: String,
}

#[derive(Debug, Clone)]
struct DownloadEntry {
    file: FileNode,
    path: PathBuf,
}

#[tokio::main]
async fn main() {
    let share_ids = [
        // Add the IDs heree
    ];

    for share_id in share_ids {
        let tree = match generate_directory_structure(share_id).await {
            Ok(v) => v,
            Err(e) => panic!("error in getting share details: {}", e),
        };

        let mut path = PathBuf::new();
        path.push(std::env::current_dir().unwrap());
        path.push("downloads");
        path.push(share_id.replace('!', "-").replace("/", "-"));

        let mut store = vec![];
        download_tree(&tree, &mut path, &mut store);

        println!("{:?}", store);

        let fetches = futures::stream::iter(store).for_each_concurrent(4, |entry| async move {
            //
            if let Err(e) = download(entry.file, entry.path).await {
                println!("ERRORORORORORORORORO in writing file: {}", e);
            };
        });
        fetches.await;

        // println!("{:?}", tree);
    }
}

fn download_tree(tree: &HashMap<String, Node>, path: &mut PathBuf, store: &mut Vec<DownloadEntry>) {
    for (_, val) in tree {
        match val {
            Node::File(f) => {
                println!("filename = {} path = {:?}", f.name, path.as_path());

                Command::new("/usr/bin/mkdir")
                    .arg("-p")
                    .arg(path.clone())
                    .spawn()
                    .expect("error in creating directories");

                store.push(DownloadEntry {
                    file: f.clone(),
                    path: path.clone(),
                });
            }

            Node::Folder(f) => {
                path.push(f.name.clone());
                download_tree(&f.childrens, path, store);
                path.pop();
            }
        }
    }
}

async fn download(file: FileNode, mut path: PathBuf) -> Result<(), reqwest::Error> {
    path.push(file.name.clone());

    if let Ok(f) = File::open(&path).await {
        if let Ok(meta) = f.metadata().await {
            if meta.len() == file.size {
                println!(
                    "SKIPPING FILE name: {} got size: {} path: {:?}",
                    file.name,
                    meta.len(),
                    path
                );

                return Ok(());
            } else {
                println!(
                    "size mismatch name: {} got size: {} expected size: {} path: {:?}",
                    file.name,
                    meta.len(),
                    file.size,
                    path
                )
            }
        } else {
            println!(
                "ERROR in reading file meta name: {} expected size: {} path: {:?}",
                file.name, file.size, path
            )
        }
    }

    let mut outfile = match File::create(&path).await {
        Ok(file) => file,
        Err(e) => panic!("error in creating file: {}", e),
    };

    println!("downloading {} path = {:?}", file.name, path);
    match reqwest::get(file.content_download_url.clone()).await {
        Ok(mut resp) => {
            while let Some(chunk) = resp.chunk().await? {
                match outfile.write_all(&chunk).await {
                    Ok(_) => (),
                    Err(e) => panic!("error in writing file: {}", e),
                }
            }

            Ok(())
        }
        Err(e) => {
            println!("filename = {} path = {:?}", file.name, path);
            panic!("Error in downloading: {}", e);
        }
    }
}

async fn generate_directory_structure(
    share_id: &str,
) -> Result<HashMap<String, Node>, reqwest::Error> {
    let response = get_share_details(share_id).await?;
    println!("Share Name: {}", response.name);

    let mut tree: HashMap<String, Node> = response
        .children
        .iter()
        .map(|child| {
            (
                child.id.clone(),
                Node::Folder(FolderNode {
                    name: child.name.clone(),
                    childrens: HashMap::new(),
                }),
            )
        })
        .collect();

    for child in response.children {
        dfs(&mut tree, share_id, child).await;
    }

    Ok(tree)
}

#[async_recursion]
async fn dfs(map: &mut HashMap<String, Node>, share_id: &str, node: Children) {
    if node.file.is_some() {
        map.insert(
            node.id,
            Node::File(FileNode {
                name: node.name,
                size: node.size,
                content_download_url: node.content_download_url.unwrap(),
            }),
        );

        return;
    }

    let child_details = match get_child_details(share_id, &node.id).await {
        Ok(v) => v,
        Err(e) => {
            println!("child.name: {} id ={}", node.name, node.id);
            panic!("error in getting child details: {}", e);
        }
    };

    let mut mapref = FolderNode {
        name: node.name,
        childrens: HashMap::new(),
    };

    for child in child_details.children {
        dfs(&mut mapref.childrens, share_id, child).await;
    }

    map.insert(node.id, Node::Folder(mapref));
}

async fn get_share_details(share_id: &str) -> Result<DriveItem, reqwest::Error> {
    let response = reqwest::get(format!(
        "https://api.onedrive.com/v1.0/shares/{}/driveItem/?$expand=children",
        share_id,
    ))
    .await?
    .json::<DriveItem>()
    .await?;

    Ok(response)
}

async fn get_child_details(share_id: &str, node_id: &str) -> Result<DriveItem, reqwest::Error> {
    let response = reqwest::get(format!(
        "https://api.onedrive.com/v1.0/shares/{}/driveItem/items/{}/?$expand=children",
        share_id, node_id,
    ))
    .await?
    .json::<DriveItem>()
    .await?;

    Ok(response)
}
