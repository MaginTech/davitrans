use std::fs::File;
use std::io::Read;
use serde_json;

use crate::davi_tree::*;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DaviDesNode {
  name: String,
  id: i32,
  children: Option<Vec<DaviTreeNode>>,

  #[serde(default)]
  position: Point3<f64>,
  #[serde(default)]
  rotation: Rotation3<f64>,
}


#[allow(dead_code)]
pub fn read_json_file(path: &str) -> Result<DaviTreeNode, serde_json::Error> {
  let mut file = File::open(path).expect("Unable to open file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Unable to read the file");

  let des: DaviDesNode = serde_json::from_str(&contents)?;
  

  Ok(davi)
}

#[cfg(test)]
mod tests {
  use super::*;
  use nalgebra::{Point3, Rotation3};

  #[test]
  fn test_read_json_file() {
    let test_file_path = "./test_data/sample.json";

    match read_json_file(test_file_path) {
      Ok(tree) => {
        assert_eq!(tree.name, "node");
        assert_eq!(tree.id, 1);
        assert_eq!(tree.position, Point3::<f64>::origin());
        assert_eq!(tree.rotation, Rotation3::<f64>::identity());
        assert!(tree.children.unwrap().is_empty());
      }
      Err(e) => panic!("Failed to parse JSON: {}", e),
    }
  }
}